#!/usr/bin/env python3
import re
import sys
import argparse

LINE_RE = re.compile(r"CALL STACK START \((\d+)\)")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("logfile")
    ap.add_argument("--top", type=int, default=10, help="show top N deepest occurrences")
    args = ap.parse_args()

    max_depth = None
    max_line_no = None
    max_log_line_no = None
    count = 0
    top = []  # (depth, file_line_no, log_line_no)

    with open(args.logfile, errors="replace") as f:
        for file_line_no, line in enumerate(f, start=1):
            m = LINE_RE.search(line)
            if not m:
                continue
            depth = int(m.group(1))
            count += 1

            log_line_no = None
            prefix = line.split(":CALL STACK START", 1)[0].strip()
            if prefix.isdigit():
                log_line_no = int(prefix)

            top.append((depth, file_line_no, log_line_no))
            if max_depth is None or depth > max_depth:
                max_depth = depth
                max_line_no = file_line_no
                max_log_line_no = log_line_no

    if max_depth is None:
        print("no 'CALL STACK START (N)' lines found")
        return

    print(f"total CALL STACK START lines: {count}")
    print(f"max depth: {max_depth}")
    print(f"  at file line {max_line_no}" + (f", log line {max_log_line_no}" if max_log_line_no else ""))

    top.sort(key=lambda t: -t[0])
    print(f"\ntop {args.top} deepest:")
    for depth, file_line_no, log_line_no in top[: args.top]:
        loc = f"file_line={file_line_no}"
        if log_line_no:
            loc += f" log_line={log_line_no}"
        print(f"  depth={depth:<4} {loc}")


if __name__ == "__main__":
    main()
