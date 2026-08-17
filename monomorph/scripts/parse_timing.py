#!/usr/bin/env python3
import re
import sys
import csv
import argparse

# "EXCLUSIVE TIMING BY SCOPE [...]" is checked before the plain
# "TIMING BY SCOPE [...]" pattern below, since the latter's .search() would
# otherwise also match inside the former's header line (it's a substring,
# not a separately-anchored marker).
EXCLUSIVE_SCOPE_START = re.compile(r"=== EXCLUSIVE TIMING BY SCOPE \[(.*)\] .*===")
EXCLUSIVE_SCOPE_END = re.compile(r"=== END EXCLUSIVE TIMING BY SCOPE \[(.*)\] ===")

TIMING_START = re.compile(r"=== TIMING REPORT \[(.*)\] ===")
TIMING_END = re.compile(r"=== END TIMING REPORT \[(.*)\] ===")
TIMING_ROW = re.compile(
    r"^\s*([\d.]+)\s+(\d+)\s+([\d.]+)\s+([\d.]+)\s+([\d.]+)\s+(\S+)\s*$"
)

SCOPE_START = re.compile(r"=== TIMING BY SCOPE \[(.*)\] .*===")
SCOPE_END = re.compile(r"=== END TIMING BY SCOPE \[(.*)\] ===")
SCOPE_ROW = re.compile(
    r"^\s*([\d.]+)ms\s+(\d+)\s+([\d.]+)ms\s+(\S+)\s+(\".*\")\s*$"
)

SELFTIME_START = re.compile(r"=== SELF-TIME REPORT \[(.*)\] .*===")
SELFTIME_END = re.compile(r"=== END SELF-TIME REPORT \[(.*)\] ===")
SELFTIME_ROW = re.compile(r"^\s*([\d.]+)ms\s+(\d+)\s+([\d.]+)ms\s+(\".*\")\s*$")

BB_N = re.compile(r"bb visit (\d+)|window ending bb visit (\d+)")


def bb_n_from_label(label):
    m = BB_N.search(label)
    if not m:
        return None
    return int(m.group(1) or m.group(2))


def is_exclusive_label(label):
    return "EXCLUSIVE" in label


def parse(path):
    timing_rows = []             # (bb_n, label, category, total_ms, count, avg_ms, min_ms, max_ms)
    timing_rows_exclusive = []   # same shape
    scope_rows = []              # (bb_n, label, scope, category, total_ms, count, avg_ms)
    scope_rows_exclusive = []    # same shape
    selftime_rows = []           # (bb_n, label, scope, self_ms, calls, avg_ms)

    mode = None
    label = None

    with open(path, errors="replace") as f:
        for line in f:
            if mode is None:
                m = EXCLUSIVE_SCOPE_START.search(line)
                if m:
                    mode, label = "scope_excl", m.group(1)
                    continue
                m = TIMING_START.search(line)
                if m:
                    mode, label = "timing", m.group(1)
                    continue
                m = SCOPE_START.search(line)
                if m:
                    mode, label = "scope", m.group(1)
                    continue
                m = SELFTIME_START.search(line)
                if m:
                    mode, label = "selftime", m.group(1)
                    continue
                continue

            if mode == "timing":
                if TIMING_END.search(line):
                    mode = None
                    continue
                m = TIMING_ROW.match(line)
                if m:
                    total, count, avg, mn, mx, cat = m.groups()
                    row = (bb_n_from_label(label), label, cat,
                           float(total), int(count), float(avg), float(mn), float(mx))
                    (timing_rows_exclusive if is_exclusive_label(label) else timing_rows).append(row)
                continue

            if mode == "scope":
                if SCOPE_END.search(line):
                    mode = None
                    continue
                m = SCOPE_ROW.match(line)
                if m:
                    total, count, avg, cat, scope = m.groups()
                    scope_rows.append(
                        (bb_n_from_label(label), label, scope.strip('"'), cat,
                         float(total), int(count), float(avg))
                    )
                continue

            if mode == "scope_excl":
                if EXCLUSIVE_SCOPE_END.search(line):
                    mode = None
                    continue
                m = SCOPE_ROW.match(line)
                if m:
                    total, count, avg, cat, scope = m.groups()
                    scope_rows_exclusive.append(
                        (bb_n_from_label(label), label, scope.strip('"'), cat,
                         float(total), int(count), float(avg))
                    )
                continue

            if mode == "selftime":
                if SELFTIME_END.search(line):
                    mode = None
                    continue
                m = SELFTIME_ROW.match(line)
                if m:
                    total, count, avg, scope = m.groups()
                    selftime_rows.append(
                        (bb_n_from_label(label), label, scope.strip('"'),
                         float(total), int(count), float(avg))
                    )
                continue

    return timing_rows, timing_rows_exclusive, scope_rows, scope_rows_exclusive, selftime_rows


def write_csv(rows, header, path):
    with open(path, "w", newline="") as f:
        w = csv.writer(f)
        w.writerow(header)
        w.writerows(rows)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("logfile")
    ap.add_argument("--outdir", default=".")
    ap.add_argument("--top", type=int, default=20)
    args = ap.parse_args()

    (timing_rows, timing_rows_exclusive,
     scope_rows, scope_rows_exclusive,
     selftime_rows) = parse(args.logfile)

    timing_header = ["bb_visit", "label", "category", "total_ms", "count", "avg_ms", "min_ms", "max_ms"]
    scope_header = ["bb_visit", "label", "scope", "category", "total_ms", "count", "avg_ms"]

    write_csv(timing_rows, timing_header, f"{args.outdir}/timing_windows.csv")
    write_csv(timing_rows_exclusive, timing_header, f"{args.outdir}/timing_windows_exclusive.csv")
    write_csv(scope_rows, scope_header, f"{args.outdir}/timing_by_scope.csv")
    write_csv(scope_rows_exclusive, scope_header, f"{args.outdir}/timing_by_scope_exclusive.csv")
    write_csv(
        selftime_rows,
        ["bb_visit", "label", "scope", "self_ms", "calls", "avg_ms"],
        f"{args.outdir}/self_time.csv",
    )

    print(f"parsed {len(timing_rows)} timing rows, {len(timing_rows_exclusive)} exclusive timing rows, "
          f"{len(scope_rows)} scope rows, {len(scope_rows_exclusive)} exclusive scope rows, "
          f"{len(selftime_rows)} self-time rows")

    ## Pivot: category -> [(bb_visit, avg_ms), ...] sorted by bb_visit, to eyeball trend
    #by_cat = {}
    #for bb_n, label, cat, total, count, avg, mn, mx in timing_rows:
    #    by_cat.setdefault(cat, []).append((bb_n, avg, mn, mx, total, count))
    #for cat in by_cat:
    #    by_cat[cat].sort(key=lambda r: (r[0] is None, r[0]))

    #print("\n=== per-category avg_ms trend across windows (inclusive) ===")
    #for cat, series in sorted(by_cat.items()):
    #    vals = [f"{bb}:{avg:.4f}" for bb, avg, *_ in series]
    #    print(f"{cat:>20}  " + "  ".join(vals))

    ## last snapshot of self-time, sorted desc - "who's slow right now"
    #if selftime_rows:
    #    latest_label = max(
    #        (r[1] for r in selftime_rows), key=lambda l: (bb_n_from_label(l) is None, bb_n_from_label(l))
    #    )
    #    latest = [r for r in selftime_rows if r[1] == latest_label]
    #    latest.sort(key=lambda r: -r[3])
    #    print(f"\n=== top {args.top} scopes by self-time in latest snapshot ({latest_label}) ===")
    #    for bb_n, label, scope, self_ms, calls, avg in latest[: args.top]:
    #        print(f"{self_ms:>10.3f}ms  calls={calls:<8} avg={avg:.4f}ms  {scope}")

    ## same, but for exclusive (scope, category) time - "who's actually
    ## burning time on its own work, in this specific category, right now"
    #if scope_rows_exclusive:
    #    latest_label = max(
    #        (r[1] for r in scope_rows_exclusive),
    #        key=lambda l: (bb_n_from_label(l) is None, bb_n_from_label(l)),
    #    )
    #    latest = [r for r in scope_rows_exclusive if r[1] == latest_label]
    #    latest.sort(key=lambda r: -r[4])
    #    print(f"\n=== top {args.top} (scope, category) by EXCLUSIVE time in latest snapshot ({latest_label}) ===")
    #    for bb_n, label, scope, cat, total, count, avg in latest[: args.top]:
    #        print(f"{total:>10.3f}ms  calls={count:<8} avg={avg:.4f}ms  {cat:<24} {scope}")


if __name__ == "__main__":
    main()
