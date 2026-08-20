#!/usr/bin/env python3
"""
Plot per-category EXCLUSIVE timing from a timing_windows_exclusive.csv
(as produced by parse_timing.py).

Two selection modes:
  - Default (threshold-based pruning): keep every category whose peak
    total_ms clears the midpoint of the log-scale y-axis range that
    results from plotting ALL categories together. This is the "pruned
    categories" mode used throughout this investigation - it adapts to
    whatever's actually large in THIS run, rather than a fixed list.
  - --top N: keep only the N categories with the highest total_ms
    summed across the whole run (a fixed count, regardless of scale).

Requires category_colors.py in the same directory (or on PYTHONPATH) -
it's what keeps each category assigned the same color across every plot
you make, run to run.

Usage:
    python3 plot_timing.py timing_windows_exclusive.csv
    python3 plot_timing.py timing_windows_exclusive.csv --top 5
    python3 plot_timing.py timing_windows_exclusive.csv --out myplot.png
    python3 plot_timing.py timing_windows_exclusive.csv --title "My title"
"""
import argparse

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

from category_colors import color_for

NON_STMT_TERM_CATS = ("BbStatements", "BbTerminator")


def compute_pruned_categories(df, all_cats):
    """Threshold-based pruning: render everything once (in memory, not
    saved) to see what y-axis range a log-scale plot of ALL categories
    would need, then keep only categories whose own peak value clears
    the midpoint of that range. Categories that never get anywhere near
    the top of the chart get dropped, since they'd just clutter the
    legend without being visible against the dominant ones anyway."""
    all_cat_total = df.groupby("bb_visit")["total_ms"].sum().sort_index()

    fig, ax = plt.subplots()
    ax.plot(all_cat_total.index, all_cat_total.values)
    for cat in all_cats:
        g = df[df["category"] == cat]
        ax.plot(g["bb_visit"], g["total_ms"])
    ax.set_yscale("log")
    ymin, ymax = ax.get_ylim()
    plt.close(fig)

    log_mid = (np.log10(ymin) + np.log10(ymax)) / 2
    midpoint_value = 10**log_mid

    return [
        cat
        for cat in all_cats
        if df[df["category"] == cat]["total_ms"].max() > midpoint_value
    ]


def compute_top_n_categories(df, all_cats, n):
    by_total = df[df["category"].isin(all_cats)].groupby("category")["total_ms"].sum()
    return by_total.sort_values(ascending=False).head(n).index.tolist()


def main():
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("csv", help="Path to timing_windows_exclusive.csv")
    ap.add_argument(
        "--out", default=None, help="Output PNG path (default: <csv>_plot.png)"
    )
    ap.add_argument(
        "--top",
        type=int,
        default=None,
        help="Only plot the top N categories by total exclusive time "
        "(default: threshold-based pruning - see module docstring)",
    )
    ap.add_argument("--title", default=None, help="Plot title (default: auto-generated)")
    args = ap.parse_args()

    df = pd.read_csv(args.csv)
    # "FINAL GLOBAL EXCLUSIVE" is a cumulative re-statement of all the
    # windowed rows, not an additional window - keep it out of any sum.
    df = df[df["bb_visit"].notna()].copy()
    df["bb_visit"] = df["bb_visit"].astype(int)
    x_max = df["bb_visit"].max()
    all_cats = [
        c for c in sorted(df["category"].unique()) if c not in NON_STMT_TERM_CATS
    ] + list(NON_STMT_TERM_CATS)
    all_cat_total = df.groupby("bb_visit")["total_ms"].sum().sort_index()

    if args.top:
        cats = compute_top_n_categories(df, all_cats, args.top)
        title = args.title or f"Top {args.top} categories by total exclusive time (bb_visit 0-{x_max})"
    else:
        cats = compute_pruned_categories(df, all_cats)
        title = args.title or f"EXCLUSIVE timing, pruned categories (bb_visit 0-{x_max})"

    out_path = args.out or args.csv.rsplit(".", 1)[0] + "_plot.png"

    fig, ax = plt.subplots(figsize=(18, 9))
    #ax.plot(
    #    all_cat_total.index,
    #    all_cat_total.values,
    #    marker=".",
    #    markersize=2,
    #    linewidth=1.2,
    #    label="AllCatTotal",
    #    #color=color_for("BbTotal"),
    #    zorder=5,
    #)
    for cat in cats:
        g = df[df["category"] == cat].sort_values("bb_visit")
        linestyle = "--" if cat.startswith("Stmt") else "-"
        ax.plot(
            g["bb_visit"],
            g["total_ms"],
            marker=".",
            markersize=1,
            linewidth=0.8,
            linestyle=linestyle,
            label=cat,
            #color=color_for(cat),
            zorder=3,
        )
    ax.set_yscale("log")
    ax.set_xlabel("bb_visit (end of 200-bb window)")
    ax.set_ylabel("total_ms per window, EXCLUSIVE (log scale)")
    ax.set_title(title)
    ax.legend(loc="upper left", fontsize=8, ncol=2 if len(cats) > 10 else 1)
    ax.grid(True, which="both", linestyle="--", alpha=0.4)
    fig.tight_layout()
    fig.savefig(out_path, dpi=300)

    print(f"saved to {out_path}")
    print(f"plotted {len(cats)} categories: {cats}")


if __name__ == "__main__":
    main()
