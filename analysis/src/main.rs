use analyzeme::ProfilingData;
use indicatif::ProgressIterator;
use std::fmt::Write as _;
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::Command;

fn download_crates() {
    use crates_io_api::{CratesQuery, Sort, SyncClient};
    let client = SyncClient::new(
        "rustc-perf-analysis (someone@somewhere.org)",
        std::time::Duration::from_millis(1000),
    )
    .unwrap();

    let mut crates = Vec::new();
    for page in 1..2 {
        let mut query = CratesQuery::builder()
            .sort(Sort::Downloads)
            .page_size(100)
            .build();
        query.set_page(page);
        let response = client.crates(query).unwrap();

        for c in response.crates {
            crates.push((c.name, c.max_stable_version.unwrap_or(c.max_version)));
        }
    }
    for krate in crates {
        Command::new("./target/release/collector")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .arg("download")
            .arg("crate")
            .arg(krate.0)
            .arg(krate.1)
            .status()
            .unwrap();
    }
}

#[derive(Default, Debug, Clone)]
pub struct CompilationSection {
    pub name: String,
    // It is unspecified if this is duration, fraction or something else. It should only be
    // evaluated against the total sum of values.
    pub value: u64,
}

fn extract_profiling_data(path: &Path) -> anyhow::Result<ProfilingData> {
    let data = std::fs::read(path)?;
    ProfilingData::from_paged_buffer(data, None)
        .map_err(|_| anyhow::Error::msg("could not parse profiling data"))
}

fn compute_compilation_sections(profile: &ProfilingData) -> Vec<CompilationSection> {
    let mut first_event_start = None;
    let mut backend_start = None;
    let mut backend_end = None;
    let mut linker_duration = None;
    let mut mir_borrowck = 0;
    let mut sections = vec![];

    for event in profile.iter_full() {
        if first_event_start.is_none() {
            first_event_start = event.payload.timestamp().map(|t| t.start());
        }
        if event.label == "type_check_crate" {
            sections.push(CompilationSection {
                name: "typeck".to_string(),
                value: event
                    .payload
                    .timestamp()
                    .map(|t| t.end().duration_since(t.start()))
                    .unwrap()
                    .unwrap()
                    .as_nanos() as u64,
            });
        } else if event.label == "mir_borrowck" {
            mir_borrowck += event
                .payload
                .timestamp()
                .map(|t| t.end().duration_since(t.start()))
                .unwrap()
                .unwrap()
                .as_nanos() as u64;
        } else if event.label == "generate_crate_metadata" {
            sections.push(CompilationSection {
                name: "metadata".to_string(),
                value: event
                    .payload
                    .timestamp()
                    .map(|t| t.end().duration_since(t.start()))
                    .unwrap()
                    .unwrap()
                    .as_nanos() as u64,
            });
        } else if event.label == "codegen_crate" {
            // Start of "codegen_crate" => start of backend
            backend_start = event.payload.timestamp().map(|t| t.start());
        } else if event.label == "finish_ongoing_codegen" {
            // End of "finish_ongoing_codegen" => end of backend
            backend_end = event.payload.timestamp().map(|t| t.end());
        } else if event.label == "link_crate" {
            // The "link" query overlaps codegen, so we want to look at the "link_crate" query
            // instead.
            linker_duration = event.duration();
        }
    }
    if let (Some(start), Some(end)) = (first_event_start, backend_start) {
        if let Ok(duration) = end.duration_since(start) {
            sections.push(CompilationSection {
                name: "Frontend".to_string(),
                value: duration.as_nanos() as u64,
            });
        }
    }
    if let (Some(start), Some(end)) = (backend_start, backend_end) {
        if let Ok(duration) = end.duration_since(start) {
            sections.push(CompilationSection {
                name: "Backend".to_string(),
                value: duration.as_nanos() as u64,
            });
        }
    }
    if let Some(duration) = linker_duration {
        sections.push(CompilationSection {
            name: "Linker".to_string(),
            value: duration.as_nanos() as u64,
        });
    }
    sections.push(CompilationSection {
        name: "borrowck".to_string(),
        value: mir_borrowck,
    });

    sections
}

struct BenchResult {
    benchmark: String,
    kind: String,
    profile: String,
    scenario: String,
    frontend: u64,
    backend: u64,
    linker: u64,
    typeck: u64,
    borrowck: u64,
    metadata: u64,
}

fn run_benchmark(
    benchmark: &Path,
    root_dir: &Path,
    result_dir: &Path,
    results: &mut Vec<BenchResult>,
) -> anyhow::Result<()> {
    let name = benchmark.file_name().unwrap().to_str().unwrap();
    println!("Running {name}");

    let manifest = benchmark.join("Cargo.toml");
    let mut manifest_content = std::fs::read_to_string(&manifest)?;
    if !manifest_content.contains("[workspace]") {
        writeln!(manifest_content, "\n[workspace]")?;
        std::fs::write(manifest, manifest_content)?;
    }

    let metadata = cargo_metadata::MetadataCommand::new()
        .current_dir(&benchmark)
        .exec()?;
    let pkg = metadata
        .root_package()
        .or_else(|| metadata.workspace_packages().into_iter().next())
        .ok_or_else(|| anyhow::anyhow!("Could not find root package"))?;
    let is_binary = pkg.targets.iter().find(|t| t.is_bin()).is_some();

    let patch_file = benchmark.join("0-println.patch");
    let target = pkg
        .targets
        .iter()
        .find(|t| t.is_bin())
        .unwrap_or_else(|| pkg.targets.first().unwrap());
    let entry_file = &target.src_path;
    let mut content = std::fs::read_to_string(&entry_file)?;
    writeln!(content, r#"fn _____foo() {{ let a = 5; }}"#)?;
    let patched_path = PathBuf::from(format!(
        "/tmp/{}-patched.rs",
        benchmark.file_name().unwrap().to_str().unwrap()
    ));
    std::fs::write(&patched_path, content)?;
    let output = Command::new("git")
        .arg("diff")
        .arg("--no-index")
        .arg("--")
        .arg(&entry_file)
        .arg(&patched_path)
        .output()?;
    let diff = String::from_utf8(output.stdout)?;
    let relative_path = entry_file
        .strip_prefix(&benchmark.canonicalize()?)?
        .to_string();
    let diff = diff.replace(entry_file.as_str(), &format!("/{relative_path}"));
    let diff = diff.replace(patched_path.to_str().unwrap(), &format!("/{relative_path}"));
    std::fs::write(&patch_file, &diff)?;

    // TODO see if can get the right triple as an env var directly...
    #[cfg(target_arch = "x86_64")]
    let arch = "x86_64";
    #[cfg(target_arch = "arm")]
    let arch = "arm";
    #[cfg(target_arch = "aarch64")]
    let arch = "aarch64";

    #[cfg(target_os = "linux")]
    let os = "unknown-linux-gnu";
    #[cfg(target_os = "macos")]
    let os = "apple-darwin";

    let rustc_path = format!("{}/.rustup/toolchains/nightly-{arch}-{os}/bin/rustc", env!("HOME"));
    let status = Command::new("./target/release/collector")
        .current_dir(root_dir)
        .arg("profile_local")
        .arg("self-profile")
        .arg(rustc_path)
        .arg("--include")
        .arg(name)
        .status()?;
    if !status.success() {
        return Err(anyhow::anyhow!(
            "Failed to benchmark {name}: {}",
            status.code().unwrap_or(-1)
        ));
    }

    for dir in std::fs::read_dir(result_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .map(|e| e.path())
    {
        let profile = dir.join("Zsp.mm_profdata");
        let data = extract_profiling_data(&profile)?;
        let mut dir_parts = dir
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .split("-")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
            .rev();
        let scenario = dir_parts.next().unwrap();
        let profile = dir_parts.next().unwrap();

        let sections = compute_compilation_sections(&data);
        // println!("{name} ({profile}/{scenario}): {sections:?}");

        let result = BenchResult {
            benchmark: name.to_string(),
            kind: if is_binary {
                "bin".to_string()
            } else {
                "lib".to_string()
            },
            profile,
            scenario,
            frontend: sections
                .iter()
                .find(|s| s.name == "Frontend")
                .map(|s| s.value)
                .ok_or_else(|| anyhow::anyhow!("Could not find frontend"))?,
            backend: sections
                .iter()
                .find(|s| s.name == "Backend")
                .map(|s| s.value)
                .ok_or_else(|| anyhow::anyhow!("Could not find backend"))?,
            linker: sections
                .iter()
                .find(|s| s.name == "Linker")
                .map(|s| s.value)
                .ok_or_else(|| anyhow::anyhow!("Could not find linker"))?,
            typeck: sections
                .iter()
                .find(|s| s.name == "typeck")
                .map(|s| s.value)
                .unwrap_or(0),
            borrowck: sections
                .iter()
                .find(|s| s.name == "borrowck")
                .map(|s| s.value)
                .unwrap_or(0),
            metadata: sections
                .iter()
                .find(|s| s.name == "metadata")
                .map(|s| s.value)
                .unwrap_or(0),
        };
        results.push(result);
    }
    Ok(())
}

fn save_results(results: &[BenchResult]) -> anyhow::Result<()> {
    let mut file = BufWriter::new(File::create("results.csv")?);
    writeln!(
        file,
        "benchmark,kind,profile,scenario,frontend,backend,linker,borrowck,typeck,metadata"
    )?;
    for result in results {
        let BenchResult {
            benchmark,
            kind,
            profile,
            scenario,
            frontend,
            backend,
            linker,
            typeck,
            borrowck,
            metadata,
        } = result;
        writeln!(
            file,
            "{benchmark},{kind},{profile},{scenario},{frontend},{backend},{linker},{borrowck},{typeck},{metadata}"
        )?;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut benchmarks: Vec<_> = std::fs::read_dir("../collector/compile-benchmarks")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    benchmarks.sort();
    benchmarks.retain(|b| {
        let name = b.file_name().unwrap().to_str().unwrap();
        name.contains("diesel")
        // name.contains("du-dust") || name.contains("hyperfine")
        // name.contains("bat")
        //     || name.contains("fd-find")
        //     || name.contains("eza")
    });

    // Not part of the workspace so "CARGO_MANIFEST_DIR" resolves to /analysis
    let analysis_root_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root_dir = analysis_root_dir.parent()
        .ok_or(io::Error::new(io::ErrorKind::NotFound,
                              "Could not get 'root_dir' of 'rustc-perf'"))?;
    let result_dir = root_dir.join("results");

    let mut results: Vec<BenchResult> = vec![];
    for benchmark in benchmarks.iter().progress() {
        if result_dir.is_dir() {
            std::fs::remove_dir_all(&result_dir)?;
        }
        if let Err(error) = run_benchmark(&benchmark, &root_dir, &result_dir, &mut results) {
            println!("{} has failed: {error:?}", benchmark.display());
        }
        // Delete temporary files to clear disk space
        for dir in std::fs::read_dir("/tmp")? {
            let dir = dir?;
            let name = dir.file_name().to_str().unwrap().to_string();
            if name.starts_with("tmp") || name.starts_with(".tmp") {
                let _ = std::fs::remove_dir_all(dir.path());
            }
        }
        save_results(&results)?;
    }
    save_results(&results)?;

    Ok(())
}
