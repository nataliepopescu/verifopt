#!/usr/bin/env python3
"""
Check whether specific categories get systematically slower as a run
progresses - not just noisy fluctuation, but a genuine ramp/plateau/decay
in per-call cost (avg_ms).

Two outputs:
  1. A printed table: for each category, avg_ms in the first/middle/last
     third of the run (by bb_visit), sorted by last-third/first-third
     ratio - categories at the top are getting slower fastest.
  2. A plot: rolling-average avg_ms over bb_visit for the top N trending
     categories, so you can see WHERE in the run the change happens (an
     early ramp then plateau, a gradual drift, vs. a one-off spike) rather
     than just a single before/after number.

A single extreme outlier window can dominate the ratio for whatever
categories it touches without reflecting a real trend - use
--exclude-bb-visit to drop known one-off spikes before computing either
output.

Usage:
    python3 plot_trends.py timing_windows_exclusive.csv
    python3 plot_trends.py timing_windows_exclusive.csv --top 8
    python3 plot_trends.py timing_windows_exclusive.csv --exclude-bb-visit 175800
"""
import argparse

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import pandas as pd

from category_colors import color_for


def thirds_table(df, min_count):
    x_max = df["bb_visit"].max()
    third = x_max / 3
    df = df.copy()
    df["period"] = pd.cut(
        df["bb_visit"],
        bins=[0, third, 2 * third, x_max],
        labels=["first_third", "middle_third", "last_third"],
    )

    rows = []
    for cat, g in df.groupby("category"):
        if g["count"].sum() < min_count:
            continue
        piv = g.groupby("period", observed=True)["avg_ms"].mean()
        if len(piv) < 3 or piv.isna().any():
            continue
        first = piv.get("first_third")
        mid = piv.get("middle_third")
        last = piv.get("last_third")
        if pd.isna(first) or first == 0:
            continue
        rows.append((cat, first, mid, last, last / first))

    rows.sort(key=lambda r: -r[4])
    return rows


def main():
    ap = argparse.ArgumentParser(
        description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter
    )
    ap.add_argument("csv", help="Path to timing_windows_exclusive.csv")
    ap.add_argument(
        "--out", default=None, help="Output PNG path (default: <csv>_trends.png)"
    )
    ap.add_argument(
        "--top",
        type=int,
        default=6,
        help="Number of top-trending categories to plot (default: 6)",
    )
    ap.add_argument(
        "--exclude-bb-visit",
        type=int,
        action="append",
        default=[],
        help="Exclude a specific bb_visit window (e.g. a known one-off "
        "spike) from both the table and the plot. Repeatable.",
    )
    ap.add_argument(
        "--min-count",
        type=int,
        default=20,
        help="Skip categories with fewer than this many total calls across "
        "the whole run (default: 20) - avoids noisy ratios from rarely-hit "
        "categories.",
    )
    ap.add_argument(
        "--rolling-window",
        type=int,
        default=15,
        help="Rolling-average window size, in 200-bb checkpoints (default: 15)",
    )
    ap.add_argument(
        "--metric",
        choices=["avg_ms", "total_ms"],
        default="avg_ms",
        help="Plot y-axis metric (default: avg_ms, per-call cost). "
        "total_ms is aggregate cost per window - conflates call-count "
        "changes with per-call cost changes, so the table above always "
        "uses avg_ms regardless of this flag.",
    )
    args = ap.parse_args()

    df = pd.read_csv(args.csv)
    if args.exclude_bb_visit:
        df = df[~df["bb_visit"].isin(args.exclude_bb_visit)]

    rows = thirds_table(df, args.min_count)

    print(
        f"{'category':32s} {'1st third':>12s} {'mid third':>12s} "
        f"{'last third':>12s} {'ratio':>8s}"
    )
    for cat, first, mid, last, ratio in rows:
        print(f"{cat:32s} {first:12.4f} {mid:12.4f} {last:12.4f} {ratio:8.2f}")

    top_cats = [r[0] for r in rows[: args.top]]

    fig, ax = plt.subplots(figsize=(16, 9))
    for cat in top_cats:
        g = df[df["category"] == cat].sort_values("bb_visit")
        roll = g[args.metric].rolling(args.rolling_window, min_periods=3).mean()
        ax.plot(g["bb_visit"], roll, label=cat, color=color_for(cat), linewidth=1.5)
    ax.set_yscale("log")
    ax.set_xlabel("bb_visit")
    metric_label = "avg_ms per call" if args.metric == "avg_ms" else "total_ms per window"
    ax.set_ylabel(f"{metric_label} ({args.rolling_window}-window rolling mean)")
    ax.set_title("Per-call cost trend over the run" if args.metric == "avg_ms" else "Aggregate cost trend over the run")
    ax.legend(loc="upper left", fontsize=9)
    ax.grid(True, which="both", linestyle="--", alpha=0.4)
    fig.tight_layout()

    out_path = args.out or args.csv.rsplit(".", 1)[0] + "_trends.png"
    fig.savefig(out_path, dpi=300)
    print(f"\nsaved plot to {out_path}")


if __name__ == "__main__":
    main()
