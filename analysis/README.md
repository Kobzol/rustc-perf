# Analysis of frontend/backend/linker ratio in `rustc`

1) Compile `rustc-perf` in release mode (`cargo build --release` in the root directory)
2) Execute `cargo run --release` :)

NOTE: On macOS you may see this error:

```
dtrace: failed to initialize dtrace: DTrace requires additional privileges
```

which can be solved by instead running thus:

```
sudo cargo run --release
```
