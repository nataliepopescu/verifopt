#!/usr/bin/env python3
"""
Split a MIR dump file containing repeated
    ######### MIR before #########
    ...
    ######### END before #########
    ######### MIR after #########
    ...
    ######### END after #########
blocks into two separate files (one with all "before" blocks, one with
all "after" blocks) so they can be diffed easily.

Usage:
    python3 split_mir.py mir_dump.txt
    # writes mir_dump.before.txt and mir_dump.after.txt

    python3 split_mir.py mir_dump.txt -o out_prefix
    # writes out_prefix.before.txt and out_prefix.after.txt
"""
import argparse
import re
import sys
from pathlib import Path

START_RE = re.compile(r"^#########\s*MIR\s+(before|after)\s*#########\s*$")
END_RE = re.compile(r"^#########\s*END\s+(before|after)\s*#########\s*$")


def split_mir(text: str):
    """Return (before_blocks, after_blocks) as lists of block text (including header line)."""
    lines = text.splitlines(keepends=True)
    blocks = {"before": [], "after": []}
    current_kind = None
    current_lines = []

    for line in lines:
        stripped = line.rstrip("\n")
        start_match = START_RE.match(stripped)
        end_match = END_RE.match(stripped)

        if start_match:
            if current_kind is not None:
                sys.stderr.write(
                    f"Warning: new block started before previous '{current_kind}' block "
                    f"was closed; discarding partial block.\n"
                )
            current_kind = start_match.group(1)
            current_lines = [line]
            continue

        if end_match:
            kind = end_match.group(1)
            if current_kind != kind:
                sys.stderr.write(
                    f"Warning: END {kind} does not match currently open block "
                    f"'{current_kind}'; skipping.\n"
                )
            else:
                current_lines.append(line)
                blocks[kind].append("".join(current_lines))
            current_kind = None
            current_lines = []
            continue

        if current_kind is not None:
            current_lines.append(line)

    if current_kind is not None:
        sys.stderr.write(
            f"Warning: file ended while '{current_kind}' block was still open; "
            f"discarding partial block.\n"
        )

    return blocks["before"], blocks["after"]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("input", type=Path, help="Path to the MIR dump file")
    parser.add_argument(
        "-o", "--out-prefix", type=str, default=None,
        help="Prefix for output files (default: input filename without extension)",
    )
    parser.add_argument(
        "--separator", type=str, default="\n",
        help="Extra text inserted between consecutive blocks in each output file "
             "(default: single blank line)",
    )
    args = parser.parse_args()

    text = args.input.read_text()
    before_blocks, after_blocks = split_mir(text)

    if not before_blocks and not after_blocks:
        sys.stderr.write("No MIR before/after blocks found in the input file.\n")
        sys.exit(1)

    prefix = args.out_prefix or args.input.with_suffix("")
    before_path = Path(f"{prefix}.before.txt")
    after_path = Path(f"{prefix}.after.txt")

    before_path.write_text(args.separator.join(before_blocks))
    after_path.write_text(args.separator.join(after_blocks))

    print(f"Wrote {len(before_blocks)} 'before' block(s) to {before_path}")
    print(f"Wrote {len(after_blocks)} 'after' block(s) to {after_path}")


if __name__ == "__main__":
    main()
