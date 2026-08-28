#!/usr/bin/env python3
"""bench_sizes.py - summarize binary sizes collected by
compare_verifopt.sh into a plain-vs-verifopt comparison table, and write
one row per (example, build kind) to a CSV.

Input: a JSONL file where each line is
    {"kind": "plain"|"verifopt", "example": "...", "text": N, "data": N,
     "bss": N, "dec": N, "file_size_bytes": N}

"text"/"data"/"bss"/"dec" come from the `size` command (Berkeley
format); "file_size_bytes" is the binary's raw on-disk size, which can
differ from "dec" (e.g. if debug info or symbols aren't stripped).
Unlike run timing, size doesn't vary run-to-run, so there's one record
per (example, kind) rather than N samples to average.

Usage:
    bench_sizes.py SIZES_JSONL --csv OUTPUT_CSV
"""

import argparse
import csv
import json
import sys
from collections import defaultdict


def load_sizes(path):
    sizes = defaultdict(dict)  # sizes[example][kind] = {...}
    with open(path) as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            rec = json.loads(line)
            sizes[rec["example"]][rec["kind"]] = rec
    return sizes


def fmt_bytes(n):
    size = float(n)
    for unit in ("B", "KB", "MB", "GB"):
        if size < 1024 or unit == "GB":
            return f"{size:,.1f}{unit}" if unit != "B" else f"{int(size):,}B"
        size /= 1024
    return f"{size:.1f}GB"


def pct_change(a, b):
    if a <= 0:
        return 0.0
    return (b - a) / a * 100.0


def print_table(example, plain, verifopt):
    print(f"--- {example} ---")
    header = f"{'':10s} {'text':>10s} {'data':>8s} {'bss':>8s} {'dec (size)':>11s} {'file size':>11s}"
    print(header)
    for label, s in (("plain", plain), ("verifopt", verifopt)):
        if s is None:
            print(f"{label:10s} (no data)")
            continue
        print(
            f"{label:10s} {fmt_bytes(s['text']):>10s} {fmt_bytes(s['data']):>8s} "
            f"{fmt_bytes(s['bss']):>8s} {fmt_bytes(s['dec']):>11s} {fmt_bytes(s['file_size_bytes']):>11s}"
        )
    if plain and verifopt:
        pct_dec = pct_change(plain["dec"], verifopt["dec"])
        pct_file = pct_change(plain["file_size_bytes"], verifopt["file_size_bytes"])
        d1 = "larger" if pct_dec >= 0 else "smaller"
        d2 = "larger" if pct_file >= 0 else "smaller"
        print(f"{'':10s} verifopt is {abs(pct_dec):.1f}% {d1} by size (text+data+bss), {abs(pct_file):.1f}% {d2} by file size")
    print()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("sizes_jsonl")
    ap.add_argument("--csv", required=True, help="output CSV path")
    args = ap.parse_args()

    sizes = load_sizes(args.sizes_jsonl)

    if not sizes:
        print("No binary size data collected.")
        sys.exit(0)

    csv_rows = []
    overall_pct_changes = []

    for example in sorted(sizes.keys()):
        plain = sizes[example].get("plain")
        verifopt = sizes[example].get("verifopt")
        print_table(example, plain, verifopt)

        for kind, s in (("plain", plain), ("verifopt", verifopt)):
            if s is None:
                continue
            csv_rows.append(
                {
                    "example": example,
                    "kind": kind,
                    "text_bytes": s["text"],
                    "data_bytes": s["data"],
                    "bss_bytes": s["bss"],
                    "dec_bytes": s["dec"],
                    "file_size_bytes": s["file_size_bytes"],
                }
            )

        if plain and verifopt:
            pct_dec = pct_change(plain["dec"], verifopt["dec"])
            overall_pct_changes.append((example, pct_dec))

    if overall_pct_changes:
        print("=== Overall (%% change in size, verifopt vs plain) ===".replace("%%", "%"))
        for example, pct in overall_pct_changes:
            direction = "larger" if pct >= 0 else "smaller"
            print(f"  {example:30s} {abs(pct):6.1f}% {direction}")
        avg_pct = sum(p for _, p in overall_pct_changes) / len(overall_pct_changes)
        direction = "larger" if avg_pct >= 0 else "smaller"
        print(f"  {'(average across examples)':30s} {abs(avg_pct):6.1f}% {direction}")
        print()

    with open(args.csv, "w", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "example",
                "kind",
                "text_bytes",
                "data_bytes",
                "bss_bytes",
                "dec_bytes",
                "file_size_bytes",
            ],
        )
        writer.writeheader()
        writer.writerows(csv_rows)


if __name__ == "__main__":
    main()
