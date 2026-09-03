#!/usr/bin/env python3
"""
Deduplicates a verifopt `stats` file down to unique (Span, CHA, FSA)
entries, preserving the "--MAYBE EXAMPLES--"/"--NOT EXAMPLES--" section
structure and updating the header counts to match.

Duplication happens because `stats` accumulates across every separate
`cargo verifopt` invocation that touches a given dispatch site (the file
isn't cleared between runs the way `mir_dump.txt`/`verifopt_store.json`
are) - the same underlying cause as the mir_dump.txt duplication
diagnosed earlier this session, just for this file instead.

One thing this script can't fully solve, and warns about instead of
guessing: rustc assigns internal Ty/DefId numbers (e.g. `Ty { id: 12244,
... }`) freshly each compilation session - two occurrences of the exact
same logical finding, from two separate runs, can carry different
internal id numbers and therefore fail an exact byte-for-byte match. In
practice this is rare (only genuinely differing runs would trigger it),
so rather than try to normalize every internal id away (risking hiding
a real difference), this script flags any Span whose occurrences didn't
all collapse to one exact match, so a person can look at the specific
case rather than have it silently resolved one way or the other.

Usage: python3 dedupe_stats.py path/to/stats [-o output_path]
"""

import argparse
import sys


def parse_entries(path):
    """Returns a list of (section, span, cha, fsa) tuples, in the
    order they appear in the file. section is "maybe" or "not"."""
    entries = []
    section = None
    with open(path, "r") as f:
        lines = f.read().splitlines()

    i = 0
    while i < len(lines):
        line = lines[i]
        if line == "--MAYBE EXAMPLES--":
            section = "maybe"
        elif line == "--NOT EXAMPLES--":
            section = "not"
        elif line.startswith("Span:"):
            span = line
            cha = lines[i + 1] if i + 1 < len(lines) else ""
            fsa = lines[i + 2] if i + 2 < len(lines) else ""
            entries.append((section, span, cha, fsa))
            i += 2  # consumed CHA and FSA lines too
        i += 1
    return entries


def dedupe(entries):
    """Returns (unique_entries, conflicting_spans) - unique_entries
    preserves first-seen order; conflicting_spans lists any Span that
    appears more than once with genuinely differing CHA/FSA content
    after exact-match dedup (see module doc - often just rustc's own
    session-specific internal id numbering, not a real difference, but
    worth a person's own look rather than silently picking one)."""
    seen = set()
    unique_entries = []
    for entry in entries:
        if entry not in seen:
            seen.add(entry)
            unique_entries.append(entry)

    by_span = {}
    for section, span, cha, fsa in unique_entries:
        by_span.setdefault(span, []).append((section, cha, fsa))
    conflicting_spans = [span for span, variants in by_span.items() if len(variants) > 1]

    return unique_entries, conflicting_spans


def write_deduped(path, unique_entries):
    maybe = [e for e in unique_entries if e[0] == "maybe"]
    not_examples = [e for e in unique_entries if e[0] == "not"]

    with open(path, "w") as f:
        f.write("STATS:\n")
        f.write(f"Num maybe examples = {len(maybe)}\n")
        f.write(f"Num non-examples = {len(not_examples)}\n")
        f.write("--MAYBE EXAMPLES--\n")
        for _, span, cha, fsa in maybe:
            f.write(span + "\n")
            f.write(cha + "\n")
            f.write(fsa + "\n")
        f.write("--NOT EXAMPLES--\n")
        for _, span, cha, fsa in not_examples:
            f.write(span + "\n")
            f.write(cha + "\n")
            f.write(fsa + "\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("stats_path")
    ap.add_argument("-o", "--output", help="output path (default: overwrite in place)")
    args = ap.parse_args()

    entries = parse_entries(args.stats_path)
    unique_entries, conflicting_spans = dedupe(entries)

    output_path = args.output or args.stats_path
    write_deduped(output_path, unique_entries)

    maybe_before = sum(1 for e in entries if e[0] == "maybe")
    maybe_after = sum(1 for e in unique_entries if e[0] == "maybe")
    not_before = sum(1 for e in entries if e[0] == "not")
    not_after = sum(1 for e in unique_entries if e[0] == "not")

    print(f"maybe examples: {maybe_before} -> {maybe_after}", file=sys.stderr)
    print(f"not examples:   {not_before} -> {not_after}", file=sys.stderr)
    print(f"total:          {len(entries)} -> {len(unique_entries)}", file=sys.stderr)
    print(f"wrote: {output_path}", file=sys.stderr)

    if conflicting_spans:
        print(
            f"\nwarning: {len(conflicting_spans)} Span(s) still have more than one distinct "
            f"CHA/FSA variant after exact-match dedup - often just rustc's own session-specific "
            f"internal id numbering (see this script's own module doc), but worth a look rather "
            f"than assuming that:",
            file=sys.stderr,
        )
        for span in conflicting_spans:
            print(f"  {span[:120]}", file=sys.stderr)


if __name__ == "__main__":
    main()
