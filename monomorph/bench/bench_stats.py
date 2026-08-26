#!/usr/bin/env python3
"""bench_stats.py - summarize per-run timings collected by
compare_verifopt.sh into a plain-vs-verifopt comparison table, and write
one summary row per (example, build kind) to a CSV.

Input: a JSONL file where each line is
    {"kind": "plain"|"verifopt", "example": "...", "run_index": N, "seconds": X}

Usage:
    bench_stats.py RESULTS_JSONL --csv OUTPUT_CSV
"""

import argparse
import csv
import json
import statistics
import sys
from collections import defaultdict


def load_runs(path):
    runs = defaultdict(lambda: defaultdict(list))  # runs[example][kind] = [seconds, ...]
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            runs[rec["example"]][rec["kind"]].append(rec["seconds"])
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


def fmt(x):
    return f"{x:.4f}s"


def print_table(example, plain_stats, verifopt_stats):
    print(f"--- {example} ---")
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

    for example in sorted(runs.keys()):
        plain_stats = summarize(runs[example].get("plain", []))
        verifopt_stats = summarize(runs[example].get("verifopt", []))
        print_table(example, plain_stats, verifopt_stats)

        for kind, s in (("plain", plain_stats), ("verifopt", verifopt_stats)):
            if s is None:
                continue
            csv_rows.append(
                {
                    "example": example,
                    "kind": kind,
                    "n": s["n"],
                    "mean_seconds": f"{s['mean']:.6f}",
                    "median_seconds": f"{s['median']:.6f}",
                    "stddev_seconds": f"{s['stddev']:.6f}",
                    "min_seconds": f"{s['min']:.6f}",
                    "max_seconds": f"{s['max']:.6f}",
                }
            )

        if plain_stats and verifopt_stats and plain_stats["mean"] > 0:
            pct = (verifopt_stats["mean"] - plain_stats["mean"]) / plain_stats["mean"] * 100.0
            overall_pct_changes.append((example, pct))

    if overall_pct_changes:
        print("=== Overall (mean %% change, verifopt vs plain) ===".replace("%%", "%"))
        for example, pct in overall_pct_changes:
            direction = "slower" if pct >= 0 else "faster"
            print(f"  {example:30s} {abs(pct):6.1f}% {direction}")
        avg_pct = statistics.mean(p for _, p in overall_pct_changes)
        direction = "slower" if avg_pct >= 0 else "faster"
        print(f"  {'(average across examples)':30s} {abs(avg_pct):6.1f}% {direction}")
        print()

    with open(args.csv, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "example",
                "kind",
                "n",
                "mean_seconds",
                "median_seconds",
                "stddev_seconds",
                "min_seconds",
                "max_seconds",
            ],
        )
        writer.writeheader()
        writer.writerows(csv_rows)


if __name__ == "__main__":
    main()
