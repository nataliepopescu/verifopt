#!/usr/bin/env python3
"""
Parses a verifopt `stats` file and reports, per dispatch site, the
CHA:FSA ratio (candidate-count only - not the candidate lists
themselves), plus mean/median/stddev - both restricted to the "maybe
examples" section and across the whole file (maybe + "not examples").

The "not examples" section is worth breaking out separately because
CHA always equals FSA there by definition (ratio 1.0, FSA never
narrowed anything) - folding it into the aggregate stats would pull
mean/median toward 1.0 in proportion to how many "not examples" exist,
which says nothing about how well FSA is actually narrowing candidates
where it has the chance to.

Usage: python3 analyze_stats.py path/to/stats
"""

import re
import statistics
import sys

CHA_RE = re.compile(r"^CHA \((\d+)\):")
FSA_RE = re.compile(r"^FSA \((\d+)\):")


def parse_ratios(path):
    """Returns (maybe_ratios, not_ratios, skipped) - the first two are
    lists of CHA/FSA floats from the "--MAYBE EXAMPLES--" and
    "--NOT EXAMPLES--" sections respectively; skipped counts dispatch
    sites with FSA (0) - CHA/FSA is undefined there (0/0), not merely
    small, so those are excluded from the ratios rather than treated
    as a ratio of 0 or 1.
    """
    maybe_ratios = []
    not_ratios = []
    skipped = 0
    current = None  # which section's list we're appending to
    pending_cha = None  # CHA count seen, waiting for the FSA line right after it

    with open(path, "r") as f:
        for raw_line in f:
            line = raw_line.rstrip("\n")

            if line == "--MAYBE EXAMPLES--":
                current = maybe_ratios
                continue
            if line == "--NOT EXAMPLES--":
                current = not_ratios
                continue

            m = CHA_RE.match(line)
            if m:
                pending_cha = int(m.group(1))
                continue

            m = FSA_RE.match(line)
            if m:
                if pending_cha is None:
                    # An FSA line with no preceding CHA line - format
                    # assumption broken somewhere upstream of this line.
                    print(
                        f"warning: FSA line with no preceding CHA line, skipping: {line[:80]}...",
                        file=sys.stderr,
                    )
                    continue
                fsa = int(m.group(1))
                if fsa == 0:
                    skipped += 1
                elif current is not None:
                    current.append(pending_cha / fsa)
                pending_cha = None
                continue

    return maybe_ratios, not_ratios, skipped


def report(name, ratios):
    print(f"\n=== {name} ===")
    print(f"count: {len(ratios)}")
    if not ratios:
        print("(no dispatch sites)")
        return
    print("ratios:", ", ".join(f"{r:.3f}" for r in ratios))
    print(f"mean:   {statistics.mean(ratios):.3f}")
    print(f"median: {statistics.median(ratios):.3f}")
    if len(ratios) > 1:
        print(f"stddev: {statistics.stdev(ratios):.3f}")
    else:
        print("stddev: n/a (only one data point)")


def main():
    if len(sys.argv) != 2:
        print(f"usage: {sys.argv[0]} path/to/stats", file=sys.stderr)
        sys.exit(1)

    maybe_ratios, not_ratios, skipped = parse_ratios(sys.argv[1])

    if skipped:
        print(
            f"note: skipped {skipped} dispatch site(s) with FSA (0) - "
            f"CHA/FSA is undefined (0/0) there, not just small",
            file=sys.stderr,
        )

    report("maybe examples only (excludes non-examples)", maybe_ratios)
    report("all dispatch sites (maybe examples + non-examples)", maybe_ratios + not_ratios)


if __name__ == "__main__":
    main()
