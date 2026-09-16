#!/usr/bin/env python3
"""plot_criterion_comparison.py - a single, combined comparison view
across any number of criterion benchmarks that live in separate
--target-dirs (and so never show up in any one's own, built-in HTML
report together) - e.g. a never-rewritten baseline vs. one with the
real MIR-level rewrite applied, each built via run_mir_bench.sh's own,
separate --target-dir convention.

Everything is organized into "paired groups" - a baseline (never
rewritten) result and a rewritten result for the same underlying
benchmark, shown as two adjacent bars under one shared label. Two ways
to feed it groups - freely combinable:

  - Explicit BASELINE_LABEL BASELINE_PATH REWRITTEN_LABEL REWRITTEN_PATH
    quadruples, one quadruple per group. Any number of groups (not just
    one).
  - --discover ROOT [ROOT ...]: recursively finds every estimates.json
    under the given root(s) (matching criterion's own
    .../<benchmark>/new/estimates.json layout), then groups them by
    <package-dir>/<criterion-group>/<benchmark-name> - the package dir
    is whatever directory directly contains the --target-dir (itself
    whatever sits directly above "criterion/" in the path). Within each
    such group, whichever --target-dir's own name contains "not-rw" or
    "not_rw" is the baseline, and whichever contains "mir-rw" or
    "mir_rw" is the rewritten result - matching run_mir_bench.sh's own,
    established --target-dir naming convention. This is what lets the
    *same* benchmark name (e.g. "negative_inner", built from one shared
    bench source file into two separate --target-dirs) get correctly
    paired up as one group, rather than being treated as two,
    independent, unrelated results.

    Groups missing either a baseline or a rewritten result (or whose
    --target-dir name doesn't match either pattern) are skipped, with a
    note explaining why - not silently dropped.

Reads each result's own estimates.json directly - criterion always
writes this to $TARGET_DIR/criterion/<group>/<benchmark>/new/
estimates.json (confirmed directly against criterion 0.7's own source,
in src/estimate.rs and src/lib.rs) - and produces a three-panel chart:
mean time with std-dev error bars (auto-scaled to ns/us/ms based on
magnitude, baseline/rewritten as two adjacent bars per group), the same
mean +/- std-dev zoomed in with a non-zero-based y-axis (since a
std-dev that's only ~1-2% of the mean is otherwise invisible against a
zero-based bar), and % change (rewritten vs. baseline) per group, using
the same median-vs-mean divergence-as-outlier-signal reasoning as this
repo's own, existing plot_bench_results.py.

Statistical significance: criterion's own, built-in significance test
only ever compares a benchmark against its own past runs (via
--baseline), never against a second, differently-named benchmark like
the pairs this script targets - so there is no significance test to
reuse here. Instead, this loads each side's own, raw per-sample
measurements from sample.json (sibling to estimates.json, same "new"
directory - confirmed directly against criterion's own source, in
src/lib.rs's SavedSample struct: {"iters": [...], "times": [...]} -
per-iteration time is times[i]/iters[i]) and runs a genuine two-sample
Mann-Whitney U test via scipy, per group (baseline vs. rewritten), the
same test this repo's own bench_stats.py already uses for its
plain-vs-verifopt comparisons. Gracefully skipped per-group (with a
note) if either side's own sample.json can't be found or scipy isn't
installed - the chart and estimates-based comparison above still work
either way.

Usage:
    plot_criterion_comparison.py \\
        [BASELINE_LABEL BASELINE_PATH REWRITTEN_LABEL REWRITTEN_PATH ...] \\
        [--discover ROOT ...] [-o OUTPUT_PNG]

Example, matching run_mir_bench.sh's own visitor-use output:
    plot_criterion_comparison.py \\
        not_rw ../../benching_examples/visitor-ex/visitor-use/target-not-rw/criterion/visitor/visitor_not_rw/new/estimates.json \\
        mir_rw ../../benching_examples/visitor-ex/visitor-use/target-mir-rw/criterion/visitor/visitor_mir_rw/new/estimates.json

Example, auto-discovering and auto-pairing every benchmark under
benching_examples/ (e.g. negative-ex's own negative_inner/
negative_outer, each built from the same source into both
target-not-rw and target-mir-rw):
    plot_criterion_comparison.py --discover ../../benching_examples
"""

import argparse
import json
import os
import re
import sys

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

# Conventional default - p < this counts as "significant", matching
# bench_stats.py's own SIGNIFICANCE_THRESHOLD.
SIGNIFICANCE_THRESHOLD = 0.05


def load_estimates(path):
    with open(path) as f:
        data = json.load(f)
    return {
        "mean": data["mean"]["point_estimate"],
        "median": data["median"]["point_estimate"],
        "std_dev": data["std_dev"]["point_estimate"],
    }


def load_per_iter_samples(estimates_path):
    # sample.json sits right next to estimates.json, same "new" dir.
    sample_path = os.path.join(os.path.dirname(estimates_path), "sample.json")
    with open(sample_path) as f:
        data = json.load(f)
    return [t / i for t, i in zip(data["times"], data["iters"])]


def run_significance_test(baseline_path, rewritten_path):
    try:
        from scipy.stats import mannwhitneyu
    except ImportError:
        return None, "scipy not installed - skipping significance test"

    try:
        baseline_samples = load_per_iter_samples(baseline_path)
        rewritten_samples = load_per_iter_samples(rewritten_path)
    except FileNotFoundError as exc:
        return None, f"sample.json not found ({exc}) - skipping significance test"

    _, p_value = mannwhitneyu(baseline_samples, rewritten_samples, alternative="two-sided")
    return p_value, None


def pct_change(base_val, other_val):
    if base_val <= 0:
        return 0.0
    return (other_val - base_val) / base_val * 100.0


def pick_unit(max_ns):
    # criterion always stores time in nanoseconds internally - pick a
    # display unit the same way its own HTML reports do, based on
    # magnitude, rather than always showing raw nanoseconds.
    if max_ns >= 1e9:
        return 1e9, "s"
    if max_ns >= 1e6:
        return 1e6, "ms"
    if max_ns >= 1e3:
        return 1e3, "\u00b5s"
    return 1.0, "ns"


def classify_target_dir(target_dir_name):
    """Returns 'baseline', 'rewritten', or None (unrecognized), based on
    run_mir_bench.sh's own, established --target-dir naming convention
    (target-not-rw / target-mir-rw).
    """
    lower = target_dir_name.lower()
    if "not-rw" in lower or "not_rw" in lower:
        return "baseline"
    if "mir-rw" in lower or "mir_rw" in lower:
        return "rewritten"
    return None


def extract_bench_order(bench_file_path):
    """Reads an actual criterion benchmark .rs file directly and returns
    the list of benchmark names in the order their own bench_function(...)
    calls appear in the source - the only reliable way to know this
    order, since estimates.json itself carries no such information at
    all. Skips commented-out (//) lines, so a benchmark temporarily
    disabled in source doesn't still influence ordering.
    """
    order = []
    pattern = re.compile(r'bench_function\(\s*"([^"]+)"')
    with open(bench_file_path) as f:
        for line in f:
            if line.strip().startswith("//"):
                continue
            match = pattern.search(line)
            if match:
                order.append(match.group(1))
    return order


def discover_paired_groups(roots):
    """Recursively finds every criterion estimates.json under the given
    root directories, grouping them by
    <package-dir>/<criterion-group>/<benchmark-name> and classifying
    each result within a group as baseline or rewritten based on its
    own --target-dir name. Returns (groups, skipped) where groups maps
    group_label -> {"baseline": path_or_None, "rewritten": path_or_None}
    and skipped is a list of human-readable strings explaining any
    result that couldn't be classified at all.
    """
    groups = {}
    skipped = []

    for root in roots:
        for dirpath, _dirnames, filenames in os.walk(root):
            if "estimates.json" not in filenames:
                continue
            # Only the "new/estimates.json" copy - criterion also
            # keeps a "base/estimates.json" (its own previous-run
            # snapshot for --baseline comparisons), which would
            # otherwise show up here too as a spurious duplicate.
            if os.path.basename(dirpath) != "new":
                continue

            path = os.path.join(dirpath, "estimates.json")
            # .../<package-dir>/<target-dir>/criterion/<group>/<benchmark>/new/estimates.json
            benchmark_dir = os.path.dirname(dirpath)  # .../<benchmark>
            benchmark_name = os.path.basename(benchmark_dir)
            group_dir = os.path.dirname(benchmark_dir)  # .../<group>
            criterion_group_name = os.path.basename(group_dir)
            criterion_dir = os.path.dirname(group_dir)  # .../criterion
            target_dir = os.path.dirname(criterion_dir)  # .../<target-dir>
            target_dir_name = os.path.basename(target_dir)
            package_dir = os.path.dirname(target_dir)  # .../<package-dir>
            package_name = os.path.basename(package_dir) or package_dir

            role = classify_target_dir(target_dir_name)
            group_label = f"{package_name}/{criterion_group_name}/{benchmark_name}"

            if role is None:
                skipped.append(
                    f"{path}: --target-dir '{target_dir_name}' doesn't match "
                    "'not-rw'/'not_rw' or 'mir-rw'/'mir_rw' - not classified as "
                    "baseline or rewritten, skipped"
                )
                continue

            groups.setdefault(group_label, {})[role] = path

    return groups, skipped


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument(
        "quads",
        nargs="*",
        metavar="BASELINE_LABEL BASELINE_PATH REWRITTEN_LABEL REWRITTEN_PATH",
        help="one or more explicit groups, each a baseline label/path pair followed by a rewritten label/path pair",
    )
    ap.add_argument(
        "--discover",
        nargs="+",
        metavar="ROOT",
        default=[],
        help="recursively find every estimates.json under the given root director(y/ies), auto-grouping and auto-pairing baseline/rewritten by --target-dir name",
    )
    ap.add_argument(
        "--order-from",
        metavar="BENCH_RS_FILE",
        default=None,
        help="order results the way their bench_function(...) calls appear in this actual benchmark .rs source file, rather than alphabetically by discovered path; groups whose benchmark name isn't found in the file keep their own, original relative order, appended after the ones that are",
    )
    ap.add_argument("-o", "--output", default="criterion_comparison.png", help="output image path")
    args = ap.parse_args()

    if len(args.quads) % 4 != 0:
        print(
            "error: positional arguments must come in groups of four: "
            "BASELINE_LABEL BASELINE_PATH REWRITTEN_LABEL REWRITTEN_PATH",
            file=sys.stderr,
        )
        sys.exit(1)

    # label -> {"baseline": path, "rewritten": path}
    groups = {}
    for i in range(0, len(args.quads), 4):
        baseline_label, baseline_path, rewritten_label, rewritten_path = args.quads[i:i + 4]
        group_label = f"{baseline_label} vs {rewritten_label}"
        groups[group_label] = {"baseline": baseline_path, "rewritten": rewritten_path}

    discovered_groups, skipped = discover_paired_groups(args.discover)
    seen_paths = {p for g in groups.values() for p in g.values()}
    for label, sides in discovered_groups.items():
        if label in groups:
            continue
        if any(p in seen_paths for p in sides.values()):
            continue
        groups[label] = sides

    for note in skipped:
        print(f"note: {note}", file=sys.stderr)

    complete_groups = []
    for label, sides in groups.items():
        if "baseline" not in sides:
            print(f"note: '{label}' has a rewritten result but no baseline - skipped", file=sys.stderr)
            continue
        if "rewritten" not in sides:
            print(f"note: '{label}' has a baseline but no rewritten result - skipped", file=sys.stderr)
            continue
        complete_groups.append((label, sides["baseline"], sides["rewritten"]))

    if not complete_groups:
        print(
            "error: no complete (baseline + rewritten) groups found - via explicit "
            "quadruples and/or --discover",
            file=sys.stderr,
        )
        sys.exit(1)

    if args.order_from:
        bench_order = extract_bench_order(args.order_from)
        if not bench_order:
            print(f"note: --order-from '{args.order_from}' matched no bench_function(...) calls - order left unchanged", file=sys.stderr)
        else:
            def sort_key(indexed_group):
                original_index, (label, _, _) = indexed_group
                benchmark_name = label.split("/")[-1]
                try:
                    return (bench_order.index(benchmark_name), 0)
                except ValueError:
                    return (len(bench_order), original_index)

            complete_groups = [
                group for _, group in sorted(enumerate(complete_groups), key=sort_key)
            ]

    data = []
    for label, baseline_path, rewritten_path in complete_groups:
        try:
            baseline_est = load_estimates(baseline_path)
            rewritten_est = load_estimates(rewritten_path)
        except FileNotFoundError as exc:
            print(f"error: {exc}", file=sys.stderr)
            print(
                "(criterion only writes estimates.json after a benchmark has actually run - "
                "make sure the binary has been run with --bench first)",
                file=sys.stderr,
            )
            sys.exit(1)

        mean_pct = pct_change(baseline_est["mean"], rewritten_est["mean"])
        median_pct = pct_change(baseline_est["median"], rewritten_est["median"])
        p_value, skip_note = run_significance_test(baseline_path, rewritten_path)
        data.append((label, baseline_est, rewritten_est, mean_pct, median_pct, p_value, skip_note))

    unit_div, unit_name = pick_unit(max(max(b["mean"], r["mean"]) for _, b, r, *_ in data))

    n = len(data)
    rotate = n > 2
    tick_kwargs = {"rotation": 30, "ha": "right"} if rotate else {"rotation": 0, "ha": "center"}

    fig, (ax_time, ax_zoom, ax_pct) = plt.subplots(3, 1, figsize=(max(7, 1.6 * n), 14 if rotate else 12))

    x = range(n)
    bar_width = 0.35
    baseline_means = [b["mean"] / unit_div for _, b, r, *_ in data]
    rewritten_means = [r["mean"] / unit_div for _, b, r, *_ in data]
    baseline_errs = [b["std_dev"] / unit_div for _, b, r, *_ in data]
    rewritten_errs = [r["std_dev"] / unit_div for _, b, r, *_ in data]
    baseline_medians = [b["median"] / unit_div for _, b, r, *_ in data]
    rewritten_medians = [r["median"] / unit_div for _, b, r, *_ in data]
    labels = [label.split("/")[-1] for label, *_ in data]

    ax_time.bar([i - bar_width / 2 for i in x], baseline_means, width=bar_width, yerr=baseline_errs, capsize=4, color="#8172b3", label="baseline")
    ax_time.bar([i + bar_width / 2 for i in x], rewritten_means, width=bar_width, yerr=rewritten_errs, capsize=4, color="#55a868", label="rewritten")
    ax_time.scatter([i - bar_width / 2 for i in x], baseline_medians, marker="D", color="black", zorder=3, s=20)
    ax_time.scatter([i + bar_width / 2 for i in x], rewritten_medians, marker="D", color="black", zorder=3, s=20, label="median")
    ax_time.set_xticks(list(x))
    ax_time.set_xticklabels(labels, **tick_kwargs)
    ax_time.set_ylabel(f"time ({unit_name})")
    ax_time.set_title("mean time (bars) vs median (diamonds), \u00b1 std dev\nbaseline vs rewritten, per group", fontsize=10)
    ax_time.legend()

    # A zero-based bar chart makes a std-dev that's only ~1-2% of the
    # mean essentially invisible - this panel plots the same mean +/-
    # std-dev as a point and error bar instead, with the y-axis scaled
    # to the actual data range (not zero-based), so the error bars
    # themselves are the thing you can actually see.
    ax_zoom.errorbar(
        [i - bar_width / 2 for i in x], baseline_means, yerr=baseline_errs, fmt="o", capsize=6, markersize=7,
        color="#8172b3", ecolor="black", elinewidth=1.2, capthick=1.2, label="baseline",
    )
    ax_zoom.errorbar(
        [i + bar_width / 2 for i in x], rewritten_means, yerr=rewritten_errs, fmt="o", capsize=6, markersize=7,
        color="#55a868", ecolor="black", elinewidth=1.2, capthick=1.2, label="rewritten",
    )
    ax_zoom.set_xticks(list(x))
    ax_zoom.set_xticklabels(labels, **tick_kwargs)
    ax_zoom.set_xlim(-0.5, n - 0.5)
    ax_zoom.set_ylabel(f"time ({unit_name})")
    ax_zoom.set_title("zoomed in: mean \u00b1 std dev only (y-axis not zero-based)", fontsize=10)
    ax_zoom.legend()

    mean_pcts = [d[3] for d in data]
    median_pcts = [d[4] for d in data]
    ax_pct.bar([i - bar_width / 2 for i in x], mean_pcts, width=bar_width, color="#4c72b0", label="mean")
    ax_pct.bar([i + bar_width / 2 for i in x], median_pcts, width=bar_width, color="#dd8452", label="median")
    ax_pct.axhline(0, color="black", linewidth=0.8)
    ax_pct.set_xticks(list(x))
    ax_pct.set_xticklabels(labels, **tick_kwargs)
    ax_pct.set_ylabel("% change (rewritten vs baseline)")
    ax_pct.legend()
    ax_pct.set_title(
        "mean/median diverging noticeably means outliers are pulling the\n"
        "mean-based comparison away from the typical (median) case",
        fontsize=10,
    )

    fig.tight_layout()
    fig.savefig(args.output, dpi=150)

    for label, baseline_est, rewritten_est, mean_pct, median_pct, p_value, skip_note in data:
        b_mean, r_mean = baseline_est["mean"] / unit_div, rewritten_est["mean"] / unit_div
        b_median, r_median = baseline_est["median"] / unit_div, rewritten_est["median"] / unit_div
        print(f"{label}:")
        print(f"  baseline:  mean={b_mean:.3f}{unit_name} median={b_median:.3f}{unit_name}")
        print(f"  rewritten: mean={r_mean:.3f}{unit_name} median={r_median:.3f}{unit_name}")
        print(f"  change: mean={mean_pct:+.1f}% median={median_pct:+.1f}%")
        if p_value is not None:
            sig_word = "significant" if p_value < SIGNIFICANCE_THRESHOLD else "not significant"
            print(f"  significance (Mann-Whitney U): p={p_value:.4f} ({sig_word} at \u03b1={SIGNIFICANCE_THRESHOLD})")
        else:
            print(f"  significance: {skip_note}")
    print(f"chart written to {args.output}")


if __name__ == "__main__":
    main()
