#!/usr/bin/env python3
"""bench_stats.py - summarize per-run timings collected by
compare_verifopt.sh into a plain-vs-verifopt comparison table, and write
one summary row per (example, scenario, build kind) to a CSV.

Input: a JSONL file where each line is
    {"kind": "plain"|"verifopt", "example": "...", "scenario": "...",
     "run_index": N, "seconds": X}

"scenario" identifies one of possibly several named argument sets a
single example was run with (see bench_args.txt's format in
compare_verifopt.sh) - an example with no bench_args.txt, or a
bench_args.txt with only one unlabeled line, gets a single scenario
named "default".

Usage:
    bench_stats.py RESULTS_JSONL --csv OUTPUT_CSV
"""

import argparse
import csv
import json
import statistics
import sys
from collections import defaultdict

from scipy.stats import mannwhitneyu

# Conventional default - p < this counts as "significant" throughout.
SIGNIFICANCE_THRESHOLD = 0.05


def load_runs(path):
    # runs[(example, scenario)][kind] = [seconds, ...]
    runs = defaultdict(lambda: defaultdict(list))
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            key = (rec["example"], rec.get("scenario", "default"))
            runs[key][rec["kind"]].append(rec["seconds"])
    return runs


def summarize(seconds):
    if not seconds:
        return None
    return {
        "n": len(seconds),
        "mean": statistics.mean(seconds),
        "median": statistics.median(seconds),
        "stddev": statistics.stdev(seconds) if len(seconds) > 1 else 0.0,
        "min": min(seconds),
        "max": max(seconds),
    }


def significance_test(plain_seconds, verifopt_seconds):
    """Mann-Whitney U test (two-sided): is it plausible plain and
    verifopt's own run times are draws from the same underlying
    distribution, or does the observed difference look like more than
    run-to-run noise? Returns a p-value, or None if it couldn't be
    computed (e.g. either side has zero runs).

    Deliberately not a t-test: that assumes an approximately normal
    distribution, which wall-clock run times often aren't - a long
    tail of unusually slow runs (cache misses, scheduler preemption,
    etc.) skews them right, the same reason this script already tracks
    median alongside mean rather than mean alone. Mann-Whitney makes no
    assumption about the shape of either distribution.

    Interpretation reminder: a small p-value means the difference is
    unlikely to be pure noise, not that the effect is large - and with
    only a handful of runs per side, even a real, genuine effect may
    not reach significance at all (low statistical power), independent
    of whether verifopt actually helped or hurt.
    """
    if not plain_seconds or not verifopt_seconds:
        return None
    return float(mannwhitneyu(plain_seconds, verifopt_seconds).pvalue)


def fmt(x):
    return f"{x:.4f}s"


def display_name(example, scenario):
    if scenario == "default":
        return example
    return f"{example} [{scenario}]"


def print_table(example, scenario, plain_stats, verifopt_stats, p_value):
    print(f"--- {display_name(example, scenario)} ---")
    header = f"{'':10s} {'n':>4s} {'mean':>10s} {'median':>10s} {'stddev':>10s} {'min':>10s} {'max':>10s}"
    print(header)
    for label, s in (("plain", plain_stats), ("verifopt", verifopt_stats)):
        if s is None:
            print(f"{label:10s} (no successful runs)")
            continue
        print(
            f"{label:10s} {s['n']:>4d} {fmt(s['mean']):>10s} {fmt(s['median']):>10s} "
            f"{fmt(s['stddev']):>10s} {fmt(s['min']):>10s} {fmt(s['max']):>10s}"
        )
    if plain_stats and verifopt_stats and plain_stats["mean"] > 0:
        pct = (verifopt_stats["mean"] - plain_stats["mean"]) / plain_stats["mean"] * 100.0
        direction = "slower" if pct >= 0 else "faster"
        print(f"{'':10s} verifopt is {abs(pct):.1f}% {direction} than plain (by mean)")
    if p_value is not None:
        verdict = "significant" if p_value < SIGNIFICANCE_THRESHOLD else "not significant"
        print(f"{'':10s} Mann-Whitney U p-value: {p_value:.4f} ({verdict} at p<{SIGNIFICANCE_THRESHOLD})")
    print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("results_jsonl")
    ap.add_argument("--csv", required=True, help="output CSV path")
    args = ap.parse_args()

    runs = load_runs(args.results_jsonl)

    if not runs:
        print("No timing data collected - every example was skipped or every run failed.")
        sys.exit(0)

    csv_rows = []
    overall_pct_changes = []

    for example, scenario in sorted(runs.keys()):
        by_kind = runs[(example, scenario)]
        plain_seconds = by_kind.get("plain", [])
        verifopt_seconds = by_kind.get("verifopt", [])
        plain_stats = summarize(plain_seconds)
        verifopt_stats = summarize(verifopt_seconds)
        p_value = significance_test(plain_seconds, verifopt_seconds)
        print_table(example, scenario, plain_stats, verifopt_stats, p_value)

        for kind, s in (("plain", plain_stats), ("verifopt", verifopt_stats)):
            if s is None:
                continue
            csv_rows.append(
                {
                    "example": example,
                    "scenario": scenario,
                    "kind": kind,
                    "n": s["n"],
                    "mean_seconds": f"{s['mean']:.6f}",
                    "median_seconds": f"{s['median']:.6f}",
                    "stddev_seconds": f"{s['stddev']:.6f}",
                    "min_seconds": f"{s['min']:.6f}",
                    "max_seconds": f"{s['max']:.6f}",
                    # Same p-value on both this (example, scenario)
                    # pair's rows - it's a property of comparing the
                    # two sides, not of one kind alone, but duplicating
                    # it keeps the CSV's existing one-row-per-kind shape
                    # intact rather than needing a second, separate file.
                    "p_value": f"{p_value:.6f}" if p_value is not None else "",
                    "significant": ("yes" if p_value < SIGNIFICANCE_THRESHOLD else "no") if p_value is not None else "",
                }
            )

        if plain_stats and verifopt_stats and plain_stats["mean"] > 0:
            pct = (verifopt_stats["mean"] - plain_stats["mean"]) / plain_stats["mean"] * 100.0
            overall_pct_changes.append((example, scenario, pct, p_value))

    if overall_pct_changes:
        print("=== Overall (mean %% change, verifopt vs plain) ===".replace("%%", "%"))
        for example, scenario, pct, p_value in overall_pct_changes:
            direction = "slower" if pct >= 0 else "faster"
            name = display_name(example, scenario)
            sig_marker = ""
            if p_value is not None:
                sig_marker = "  *" if p_value < SIGNIFICANCE_THRESHOLD else ""
            print(f"  {name:40s} {abs(pct):6.1f}% {direction}{sig_marker}")
        avg_pct = statistics.mean(pct for _, _, pct, _ in overall_pct_changes)
        direction = "slower" if avg_pct >= 0 else "faster"
        print(f"  {'(average across all)':40s} {abs(avg_pct):6.1f}% {direction}")
        num_with_p = sum(1 for *_, p in overall_pct_changes if p is not None)
        num_significant = sum(1 for *_, p in overall_pct_changes if p is not None and p < SIGNIFICANCE_THRESHOLD)
        if num_with_p:
            print(f"  {num_significant}/{num_with_p} results significant at p<{SIGNIFICANCE_THRESHOLD} (marked with *)")
        print()

    with open(args.csv, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "example",
                "scenario",
                "kind",
                "n",
                "mean_seconds",
                "median_seconds",
                "stddev_seconds",
                "min_seconds",
                "max_seconds",
                "p_value",
                "significant",
            ],
        )
        writer.writeheader()
        writer.writerows(csv_rows)


if __name__ == "__main__":
    main()
