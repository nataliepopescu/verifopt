#!/usr/bin/env python3
"""plot_bench_results.py - visualize the results.csv produced by
compare_verifopt.sh (plain vs verifopt build timing comparison).

Usage:
    plot_bench_results.py [RESULTS_CSV] [-o OUTPUT_PNG] [--sort-by {overhead,name}]

Produces a two-panel chart:
  - top: mean wall time per (example, scenario), log-scale, error bars
    from stddev
  - bottom: % change (verifopt vs plain) per (example, scenario),
    colored by slower (red) / faster (green)

An example with no bench_args.txt (or a bench_args.txt with a single
unlabeled line) has one scenario named "default" and is shown by its
example name alone; an example with multiple named scenarios (see
compare_verifopt.sh's bench_args.txt format) gets one bar/row per
scenario, labeled "example [scenario]".

(example, scenario) pairs with data for only one side (plain or
verifopt) are shown in the top panel but excluded from the % change
panel and printed as a separate note, since there's nothing to compare
them against.
"""

import argparse
import csv
import sys
from collections import defaultdict

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt


def display_name(example, scenario):
    if scenario == "default":
        return example
    return f"{example} [{scenario}]"


def load_results(path):
    by_key = defaultdict(dict)  # (example, scenario) -> {kind: {...}}
    with open(path, newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            key = (row["example"], row.get("scenario", "default"))
            by_key[key][row["kind"]] = {
                "n": int(row["n"]),
                "mean": float(row["mean_seconds"]),
                "median": float(row["median_seconds"]),
                "stddev": float(row["stddev_seconds"]),
                "min": float(row["min_seconds"]),
                "max": float(row["max_seconds"]),
            }
    return by_key


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("csv_path", nargs="?", default="results.csv", help="path to results.csv (default: results.csv)")
    ap.add_argument("-o", "--output", default="bench_comparison.png", help="output image path")
    ap.add_argument(
        "--sort-by",
        choices=["overhead", "name"],
        default="overhead",
        help="order bars by verifopt overhead (worst first) or alphabetically (default: overhead)",
    )
    args = ap.parse_args()

    try:
        by_key = load_results(args.csv_path)
    except FileNotFoundError:
        print(f"error: no such file: {args.csv_path}", file=sys.stderr)
        sys.exit(1)

    if not by_key:
        print(f"error: {args.csv_path} has no data rows", file=sys.stderr)
        sys.exit(1)

    paired = []       # (name, plain, verifopt, pct) for keys with both sides
    unpaired = []      # (name, kinds) for keys with only one side

    for (example, scenario), kinds in by_key.items():
        name = display_name(example, scenario)
        if "plain" in kinds and "verifopt" in kinds:
            plain, verifopt = kinds["plain"], kinds["verifopt"]
            pct = ((verifopt["mean"] - plain["mean"]) / plain["mean"] * 100.0) if plain["mean"] > 0 else 0.0
            paired.append((name, plain, verifopt, pct))
        else:
            unpaired.append((name, kinds))

    if not paired:
        print("No (example, scenario) has both plain and verifopt data - nothing to compare.", file=sys.stderr)
        if unpaired:
            print("Partial data only:", file=sys.stderr)
            for name, kinds in unpaired:
                print(f"  {name}: {', '.join(kinds.keys())}", file=sys.stderr)
        sys.exit(1)

    if args.sort_by == "overhead":
        paired.sort(key=lambda t: t[3], reverse=True)
    else:
        paired.sort(key=lambda t: t[0])

    names = [p[0] for p in paired]
    plain_means = [p[1]["mean"] for p in paired]
    plain_stddevs = [p[1]["stddev"] for p in paired]
    verifopt_means = [p[2]["mean"] for p in paired]
    verifopt_stddevs = [p[2]["stddev"] for p in paired]
    pct_changes = [p[3] for p in paired]

    n = len(names)
    fig_height = max(6, 0.9 * n + 3)
    fig, (ax_time, ax_pct) = plt.subplots(
        2, 1, figsize=(max(8, 0.8 * n + 4), fig_height), gridspec_kw={"height_ratios": [2, 1]}
    )

    x = range(n)
    width = 0.35

    ax_time.bar(
        [i - width / 2 for i in x],
        plain_means,
        width,
        yerr=plain_stddevs,
        capsize=3,
        label="plain",
        color="#4c72b0",
    )
    ax_time.bar(
        [i + width / 2 for i in x],
        verifopt_means,
        width,
        yerr=verifopt_stddevs,
        capsize=3,
        label="verifopt",
        color="#dd8452",
    )
    ax_time.set_yscale("log")
    ax_time.set_ylabel("mean wall time, seconds (log scale)")
    ax_time.set_title("Plain vs verifopt: mean run time per example/scenario")
    ax_time.set_xticks(list(x))
    ax_time.set_xticklabels(names, rotation=30, ha="right")
    ax_time.legend()
    ax_time.grid(axis="y", linestyle=":", alpha=0.5)

    colors = ["#c44e52" if p >= 0 else "#55a868" for p in pct_changes]
    ax_pct.bar(list(x), pct_changes, color=colors)
    ax_pct.axhline(0, color="black", linewidth=0.8)
    ax_pct.set_ylabel("% change\n(verifopt vs plain)")
    ax_pct.set_xticks(list(x))
    ax_pct.set_xticklabels(names, rotation=30, ha="right")
    ax_pct.grid(axis="y", linestyle=":", alpha=0.5)
    for i, p in enumerate(pct_changes):
        ax_pct.text(
            i,
            p + (2 if p >= 0 else -2),
            f"{p:+.1f}%",
            ha="center",
            va="bottom" if p >= 0 else "top",
            fontsize=8,
        )

    fig.tight_layout()
    fig.savefig(args.output, dpi=150)
    print(f"wrote {args.output}")

    print()
    print(f"{'example [scenario]':30s} {'plain mean':>12s} {'verifopt mean':>14s} {'% change':>10s}")
    for name, plain, verifopt, pct in paired:
        direction = "slower" if pct >= 0 else "faster"
        print(
            f"{name:30s} {plain['mean']:>10.4f}s {verifopt['mean']:>12.4f}s "
            f"{abs(pct):>8.1f}% {direction}"
        )

    if unpaired:
        print()
        print("Partial data only (excluded from % change):")
        for name, kinds in unpaired:
            print(f"  {name}: {', '.join(kinds.keys())}")


if __name__ == "__main__":
    main()
