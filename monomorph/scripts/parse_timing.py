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

# Standalone lines, not block-delimited like the reports above - checked on
# every line regardless of what block (if any) we're currently inside.
OVERLAP_STATS = re.compile(
    r"MERGE_OVERLAP_STATS kind=(\S+) bb_visit=(\d+) lhs_len=(\d+) rhs_len=(\d+) shared=(\d+)"
    r"(?: ptr_identical=(true|false))?"
    r"(?: rhs_base_len=(\d+|n/a) rhs_new_entries=(\d+|n/a))?"
)

# Standalone, cumulative-since-run-start wall-clock checkpoint - deliberately
# outside the TimingCat machinery entirely, so it's an independent check on
# how much real time the sum of every category accounts for, not just
# another view of the same instrumented data.
WALL_CLOCK = re.compile(r"TOTAL WALL CLOCK bb_visit=(\d+) elapsed_ms=([\d.]+)")

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
    overlap_rows = []            # (bb_visit, kind, lhs_len, rhs_len, shared)
    wall_clock_rows = []         # (bb_visit, elapsed_ms)

    mode = None
    label = None

    with open(path, errors="replace") as f:
        for line in f:
            m = WALL_CLOCK.search(line)
            if m:
                bb_visit, elapsed_ms = m.groups()
                wall_clock_rows.append((int(bb_visit), float(elapsed_ms)))
                continue

            m = OVERLAP_STATS.search(line)
            if m:
                (kind, bb_visit, lhs_len, rhs_len, shared, ptr_identical,
                 rhs_base_len, rhs_new_entries) = m.groups()

                def _opt_int(x):
                    return int(x) if x is not None and x != "n/a" else None

                overlap_rows.append(
                    (int(bb_visit), kind, int(lhs_len), int(rhs_len), int(shared),
                     (ptr_identical == "true") if ptr_identical is not None else None,
                     _opt_int(rhs_base_len), _opt_int(rhs_new_entries))
                )
                continue

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

    return (timing_rows, timing_rows_exclusive, scope_rows, scope_rows_exclusive,
            selftime_rows, overlap_rows, wall_clock_rows)


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
     selftime_rows, overlap_rows, wall_clock_rows) = parse(args.logfile)

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
    write_csv(
        overlap_rows,
        ["bb_visit", "kind", "lhs_len", "rhs_len", "shared", "ptr_identical", "rhs_base_len", "rhs_new_entries"],
        f"{args.outdir}/merge_overlap_stats.csv",
    )
    write_csv(
        wall_clock_rows,
        ["bb_visit", "elapsed_ms"],
        f"{args.outdir}/wall_clock.csv",
    )

    print(f"parsed {len(timing_rows)} timing rows, {len(timing_rows_exclusive)} exclusive timing rows, "
          f"{len(scope_rows)} scope rows, {len(scope_rows_exclusive)} exclusive scope rows, "
          f"{len(selftime_rows)} self-time rows, {len(overlap_rows)} merge-overlap rows, "
          f"{len(wall_clock_rows)} wall-clock checkpoints")

    # Independent sanity check: does the sum of every category's EXCLUSIVE
    # total_ms up to the *last* wall-clock checkpoint roughly match that 
    # checkpoint's own independently-measured elapsed time?
    if wall_clock_rows and timing_rows_exclusive:
        last_bb = max(bb for bb, _ in wall_clock_rows)
        last_wall_ms = [ms for bb, ms in wall_clock_rows if bb == last_bb][-1]
        summed_ms = sum(
            total for bb, _, _, total, *_ in timing_rows_exclusive
            if bb is not None and bb <= last_bb
        )
        print(f"\n=== independent wall-clock check (at bb_visit={last_bb}) ===")
        print(f"  measured wall clock:          {last_wall_ms:10.1f}ms ({last_wall_ms/60000:.2f} min)")
        print(f"  sum of all categories (excl): {summed_ms:10.1f}ms ({summed_ms/60000:.2f} min)")
        if last_wall_ms > 0:
            print(f"  ratio (categories/wallclock): {summed_ms/last_wall_ms:.3f}")

    # Pivot: category -> [(bb_visit, avg_ms), ...] sorted by bb_visit, to eyeball trend
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

    ## merge-overlap diagnostics: avg overlap fraction per kind, to see
    ## whether the two sides of a union() call tend to share structure
    ## (cheap case) or are mostly disjoint (expensive case for a
    ## structural-sharing-dependent implementation)
    #if overlap_rows:
    #    print(f"\n=== merge overlap stats, by kind ===")
    #    by_kind = {}
    #    for bb_visit, kind, lhs_len, rhs_len, shared in overlap_rows:
    #        by_kind.setdefault(kind, []).append((lhs_len, rhs_len, shared))
    #    for kind, rows in sorted(by_kind.items()):
    #        n = len(rows)
    #        avg_lhs = sum(r[0] for r in rows) / n
    #        avg_rhs = sum(r[1] for r in rows) / n
    #        avg_shared = sum(r[2] for r in rows) / n
    #        avg_min_len = sum(min(r[0], r[1]) for r in rows) / n
    #        overlap_frac = (avg_shared / avg_min_len) if avg_min_len > 0 else 0.0
    #        print(
    #            f"  kind={kind:<8} n={n:<8} avg_lhs_len={avg_lhs:.1f} avg_rhs_len={avg_rhs:.1f} "
    #            f"avg_shared={avg_shared:.1f} overlap_frac_of_smaller={overlap_frac:.3f}"
    #        )


if __name__ == "__main__":
    main()
