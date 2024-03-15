from typing import List, Optional

import matplotlib.patches as mpatches
import numpy as np
import pandas as pd
import seaborn as sns
from matplotlib import pyplot as plt

PALETTE = sns.color_palette(n_colors=12)


def filter_data(data: pd.DataFrame, profile: Optional[str] = None, scenario: Optional[str] = None,
                kind: Optional[str] = None) -> pd.DataFrame:
    if profile is not None:
        data = data[data["profile"] == profile]
    if scenario is not None:
        data = data[data["scenario"] == scenario]
    if kind is not None:
        data = data[data["kind"] == kind]
    return data


def plot_violin(data: pd.DataFrame, profile: Optional[str] = None, scenario: Optional[str] = None,
                kind: Optional[str] = None, **kwargs):
    data = filter_data(data=data, profile=profile, scenario=scenario, kind=kind)
    data = pd.melt(data, id_vars=["profile", "scenario", "kind"],
                   value_vars=["frontend", "backend", "linker"],
                   var_name="section",
                   value_name="Percent")
    sns.violinplot(data=data, x="section", y="Percent")


def plot_bar(data: pd.DataFrame, profile: Optional[str] = None, scenario: Optional[str] = None,
             kind: Optional[str] = None, **kwargs):
    data = filter_data(data=data, profile=profile, scenario=scenario, kind=kind)
    data = pd.melt(data, id_vars=["profile", "scenario", "kind"],
                   value_vars=["frontend", "backend", "linker", "borrowck", "typeck", "metadata"],
                   var_name="section",
                   value_name="Percent")

    avg_backend = float(data[data["section"] == "backend"]["Percent"].mean())
    avg_linker = float(data[data["section"] == "linker"]["Percent"].mean())
    avg_borrowck = float(data[data["section"] == "borrowck"]["Percent"].mean())
    avg_typeck = float(data[data["section"] == "typeck"]["Percent"].mean())
    avg_metadata = float(data[data["section"] == "metadata"]["Percent"].mean())
    fe_base = avg_backend + avg_linker

    args = dict(align="edge")
    height = 1
    y_offset = height / 2
    inner_height = height - y_offset

    # Frontend, full height
    plt.bar(y=0, x=[0], width=[100], color=PALETTE[0], height=height, **args)
    plt.bar(y=y_offset, x=[fe_base], width=[avg_borrowck], color=PALETTE[3],
            height=inner_height, **args)
    fe_base += avg_borrowck
    plt.bar(y=y_offset, x=[fe_base], width=[avg_typeck], color=PALETTE[4],
            height=inner_height, **args)
    fe_base += avg_typeck
    plt.bar(y=y_offset, x=[fe_base], width=[avg_metadata], color=PALETTE[5],
            height=inner_height, **args)
    plt.bar(y=0, x=[avg_linker], width=[avg_backend], color=PALETTE[1], height=height, **args)
    plt.bar(y=0, x=[0], width=[avg_linker], color=PALETTE[2], height=height, **args)
    plt.gca().yaxis.set_major_locator(plt.NullLocator())
    plt.gca().set_xlabel("Percent out of whole compilation")


def create_bars(with_metadata: bool = True) -> List[mpatches.Patch]:
    bars = [
        mpatches.Patch(color=PALETTE[0], label="Frontend"),
        mpatches.Patch(color=PALETTE[1], label="Backend"),
        mpatches.Patch(color=PALETTE[2], label="Linker"),
        mpatches.Patch(color=PALETTE[3], label="borrowck"),
        mpatches.Patch(color=PALETTE[4], label="typeck"),
    ]
    if with_metadata:
        bars.append(
            mpatches.Patch(color=PALETTE[5], label="metadata"),
        )
    return bars


def single_bench(data: pd.DataFrame, name: str, benchmark: str, profile: str, scenario: str,
                 metadata: bool = False):
    ripgrep = data[data["benchmark"] == benchmark]
    plot_bar(ripgrep, profile=profile, scenario=scenario)
    plt.legend(handles=create_bars(with_metadata=metadata), loc="lower left",
               bbox_to_anchor=(1, 0.1))
    fig = plt.gcf()
    for ax in fig.axes:
        ax.spines["top"].set_visible(False)
        ax.spines["right"].set_visible(False)
        ax.spines["bottom"].set_visible(False)
        ax.spines["left"].set_visible(False)
    fig.set_size_inches(5, 2)
    fig.set_dpi(300)
    plt.savefig(f"{name}.png", bbox_inches="tight")


def grid(data: pd.DataFrame, name: str, benchmark: Optional[str] = None,
         kind: Optional[str] = None, metadata: bool = False):
    if benchmark is not None:
        data = data[data["benchmark"] == benchmark]
    if kind is not None:
        data = data[data["kind"] == kind]
    g = sns.FacetGrid(data=data, row="profile", row_order=["Check", "Debug", "Opt"], col="scenario",
                      col_order=["Full", "IncrFull", "IncrPatched0", "IncrUnchanged"], sharey=False)
    g.map_dataframe(plot_bar)
    plt.legend(handles=create_bars(with_metadata=metadata), loc="lower left",
               bbox_to_anchor=(1, 0.4))

    fig = plt.gcf()
    fig.set_size_inches(14, 6)
    fig.set_dpi(300)
    plt.tight_layout()
    plt.savefig(f"{name}.png", bbox_inches="tight")


def single_benchmark(df: pd.DataFrame, benchmark: str, metadata: bool = False):
    single_bench(df, f"{benchmark}-debug-full", benchmark, profile="Debug", scenario="Full",
                 metadata=True)
    single_bench(df, f"{benchmark}-debug-incr-patched", benchmark, profile="Debug",
                 scenario="IncrPatched0", metadata=True)
    single_bench(df, f"{benchmark}-opt-incr-patched", benchmark, profile="Opt",
                 scenario="IncrPatched0", metadata=True)
    single_bench(df, f"{benchmark}-check-incr-patched", benchmark, profile="Check",
                 scenario="IncrPatched0", metadata=True)


df = pd.read_csv("results-full.csv")
print(df.groupby(["benchmark", "kind"]).size().reset_index()["kind"].value_counts())

df["total"] = df["frontend"] + df["backend"] + df["linker"]
for key in ("frontend", "backend", "linker", "borrowck", "typeck", "metadata"):
    df[key] = (df[key].astype(np.float32) / df["total"]) * 100
df = df.drop(columns=["total"])

single_benchmark(df, "ripgrep-14.1.0")
single_benchmark(df, "regex-automata-0.4.6", metadata=True)
single_benchmark(df, "diesel-1.4.8", metadata=True)

grid(df, "ripgrep", "ripgrep-14.1.0")
grid(df, "binaries", kind="bin")
grid(df, "libraries", kind="lib")
