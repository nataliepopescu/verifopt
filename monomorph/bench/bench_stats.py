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


def fmt(x):
    return f"{x:.4f}s"


def display_name(example, scenario):
    if scenario == "default":
        return example
    return f"{example} [{scenario}]"


def print_table(example, scenario, plain_stats, verifopt_stats):
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
        plain_stats = summarize(by_kind.get("plain", []))
        verifopt_stats = summarize(by_kind.get("verifopt", []))
        print_table(example, scenario, plain_stats, verifopt_stats)

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
                }
            )

        if plain_stats and verifopt_stats and plain_stats["mean"] > 0:
            pct = (verifopt_stats["mean"] - plain_stats["mean"]) / plain_stats["mean"] * 100.0
            overall_pct_changes.append((example, scenario, pct))

    if overall_pct_changes:
        print("=== Overall (mean %% change, verifopt vs plain) ===".replace("%%", "%"))
        for example, scenario, pct in overall_pct_changes:
            direction = "slower" if pct >= 0 else "faster"
            name = display_name(example, scenario)
            print(f"  {name:40s} {abs(pct):6.1f}% {direction}")
        avg_pct = statistics.mean(p for _, _, p in overall_pct_changes)
        direction = "slower" if avg_pct >= 0 else "faster"
        print(f"  {'(average across all)':40s} {abs(avg_pct):6.1f}% {direction}")
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
            ],
        )
        writer.writeheader()
        writer.writerows(csv_rows)


if __name__ == "__main__":
    main()
