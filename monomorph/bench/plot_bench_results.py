#!/usr/bin/env python3
"""plot_bench_results.py - visualize the results.csv produced by
compare_verifopt.sh (plain vs verifopt build timing comparison).

Usage:
    plot_bench_results.py [RESULTS_CSV] [-o OUTPUT_PNG] [--sort-by {overhead,name}]

Produces a two-panel chart:
  - top: wall time per (example, scenario), log-scale. Solid bars are
    the mean; a black diamond marks the median on each bar, so you can
    see at a glance whether a handful of slow/fast runs are pulling the
    mean away from the typical case. Both the mean bars and the median
    diamonds carry the same stddev as error bars - there's only one
    stddev per dataset (how spread out the individual runs actually
    are), not a separate one per central-tendency measure; showing it
    on both just lets you gauge spread relative to whichever one you're
    looking at.
  - bottom: % change (verifopt vs plain) per (example, scenario), one
    pair of bars per entry - solid for % change by mean, hatched for
    % change by median. The two bars diverging noticeably is itself
    the signal that outliers are affecting the mean-based comparison;
    median is far less sensitive to a handful of unusually slow or
    fast runs (page cache misses, scheduler noise, etc.) than mean is.

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


def pct_change(plain_val, verifopt_val):
    if plain_val <= 0:
        return 0.0
    return (verifopt_val - plain_val) / plain_val * 100.0


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("csv_path", nargs="?", default="results.csv", help="path to results.csv (default: results.csv)")
    ap.add_argument("-o", "--output", default="bench_comparison.png", help="output image path")
    ap.add_argument(
        "--sort-by",
        choices=["overhead", "name"],
        default="overhead",
        help="order bars by verifopt overhead by mean (worst first) or alphabetically (default: overhead)",
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

    paired = []       # (name, plain, verifopt, pct_mean, pct_median)
    unpaired = []      # (name, kinds) for keys with only one side

    for (example, scenario), kinds in by_key.items():
        name = display_name(example, scenario)
        if "plain" in kinds and "verifopt" in kinds:
            plain, verifopt = kinds["plain"], kinds["verifopt"]
            pm = pct_change(plain["mean"], verifopt["mean"])
            pmed = pct_change(plain["median"], verifopt["median"])
            paired.append((name, plain, verifopt, pm, pmed))
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
    plain_medians = [p[1]["median"] for p in paired]
    plain_stddevs = [p[1]["stddev"] for p in paired]
    verifopt_means = [p[2]["mean"] for p in paired]
    verifopt_medians = [p[2]["median"] for p in paired]
    verifopt_stddevs = [p[2]["stddev"] for p in paired]
    pct_means = [p[3] for p in paired]
    pct_medians = [p[4] for p in paired]

    n = len(names)
    fig_height = max(6, 0.9 * n + 3)
    fig, (ax_time, ax_pct) = plt.subplots(
        2, 1, figsize=(max(8, 0.8 * n + 4), fig_height), gridspec_kw={"height_ratios": [2, 1]}
    )

    x = list(range(n))
    width = 0.35

    plain_x = [i - width / 2 for i in x]
    verifopt_x = [i + width / 2 for i in x]

    ax_time.bar(plain_x, plain_means, width, yerr=plain_stddevs, capsize=3, label="plain (mean)", color="#4c72b0")
    ax_time.bar(
        verifopt_x, verifopt_means, width, yerr=verifopt_stddevs, capsize=3, label="verifopt (mean)", color="#dd8452"
    )
    # Same stddev used for the mean's own error bars above - there's
    # only one stddev per dataset (how spread out the individual runs
    # are), not a separate one for mean vs median; showing it here too
    # just lets the reader gauge spread relative to whichever central
    # tendency they're actually looking at. errorbar (not scatter, which
    # has no yerr) with fmt="D" keeps the same diamond marker as before.
    # Offset toward each bar's own outer edge (still well within the
    # bar's own width) rather than sitting at the bar's exact center,
    # so these error bars land at a genuinely different x-position from
    # the mean bars' own centered ones - distinguishable regardless of
    # color perception or a grayscale printout, unlike relying on color
    # alone would be.
    median_offset = width / 4
    ax_time.errorbar(
        [px - median_offset for px in plain_x],
        plain_medians,
        yerr=plain_stddevs,
        fmt="D",
        markersize=5,
        color="black",
        capsize=3,
        zorder=3,
        label="median",
    )
    ax_time.errorbar(
        [vx + median_offset for vx in verifopt_x],
        verifopt_medians,
        yerr=verifopt_stddevs,
        fmt="D",
        markersize=5,
        color="black",
        capsize=3,
        zorder=3,
    )
    ax_time.set_yscale("log")
    ax_time.set_ylabel("wall time, seconds (log scale)")
    ax_time.set_title(
        "Plain vs verifopt: run time per example/scenario (bars: mean, diamonds: median, "
        "error bars: stddev on both)"
    )
    ax_time.set_xticks(x)
    ax_time.set_xticklabels(names, rotation=30, ha="right")
    ax_time.legend()
    ax_time.grid(axis="y", linestyle=":", alpha=0.5)

    mean_colors = ["#c44e52" if p >= 0 else "#55a868" for p in pct_means]
    median_colors = ["#c44e52" if p >= 0 else "#55a868" for p in pct_medians]
    ax_pct.bar([i - width / 2 for i in x], pct_means, width, color=mean_colors, label="% change (mean)")
    ax_pct.bar(
        [i + width / 2 for i in x],
        pct_medians,
        width,
        color=median_colors,
        hatch="////",
        edgecolor="black",
        linewidth=0.5,
        label="% change (median)",
    )
    ax_pct.axhline(0, color="black", linewidth=0.8)
    ax_pct.set_ylabel("% change\n(verifopt vs plain)")
    ax_pct.set_xticks(x)
    ax_pct.set_xticklabels(names, rotation=30, ha="right")
    ax_pct.grid(axis="y", linestyle=":", alpha=0.5)
    ax_pct.legend(fontsize=8, loc="best")
    for i, (pm, pmed) in enumerate(zip(pct_means, pct_medians)):
        ax_pct.text(
            i - width / 2, pm + (2 if pm >= 0 else -2), f"{pm:+.0f}%",
            ha="center", va="bottom" if pm >= 0 else "top", fontsize=7,
        )
        ax_pct.text(
            i + width / 2, pmed + (2 if pmed >= 0 else -2), f"{pmed:+.0f}%",
            ha="center", va="bottom" if pmed >= 0 else "top", fontsize=7,
        )

    fig.tight_layout()
    fig.savefig(args.output, dpi=150)
    print(f"wrote {args.output}")

    print()
    print(
        f"{'example [scenario]':30s} {'plain mean':>11s} {'plain med':>10s} "
        f"{'vopt mean':>11s} {'vopt med':>10s} {'%chg mean':>10s} {'%chg med':>10s}"
    )
    for (name, plain, verifopt, pm, pmed) in paired:
        print(
            f"{name:30s} {plain['mean']:>9.4f}s {plain['median']:>8.4f}s "
            f"{verifopt['mean']:>9.4f}s {verifopt['median']:>8.4f}s "
            f"{pm:>+9.1f}% {pmed:>+9.1f}%"
        )
        if abs(pm - pmed) > 10:
            print(f"{'':30s} note: mean and median % change diverge by {abs(pm - pmed):.1f} points - check for outlier runs")

    if unpaired:
        print()
        print("Partial data only (excluded from % change):")
        for name, kinds in unpaired:
            print(f"  {name}: {', '.join(kinds.keys())}")


if __name__ == "__main__":
    main()
