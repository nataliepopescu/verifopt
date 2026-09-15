#!/usr/bin/env python3
"""plot_criterion_comparison.py - a single, combined comparison view for
two criterion benchmarks that live in separate --target-dirs (and so
never show up in either one's own, built-in HTML report together) -
e.g. a never-rewritten baseline vs. one with the real MIR-level rewrite
applied, each built via run_mir_bench.sh's own, separate --target-dir
convention.

Reads each side's own estimates.json directly - criterion always writes
this to $TARGET_DIR/criterion/<group>/<benchmark>/new/estimates.json
(confirmed directly against criterion 0.7's own source, in
src/estimate.rs and src/lib.rs) - and produces a two-panel chart: mean
time with std-dev error bars (auto-scaled to ns/us/ms based on
magnitude), and % change between the two, using the same median-vs-mean
divergence-as-outlier-signal reasoning as this repo's own, existing
plot_bench_results.py.

Statistical significance: criterion's own, built-in significance test
only ever compares a benchmark against its own past runs (via
--baseline), never against a second, differently-named benchmark like
the not_rw/mir_rw pair this script targets - so there is no
significance test to reuse here. Instead, this loads each side's own,
raw per-sample measurements from sample.json (sibling to
estimates.json, same "new" directory - confirmed directly against
criterion's own source, in src/lib.rs's SavedSample struct: {"iters":
[...], "times": [...]} - per-iteration time is times[i]/iters[i]) and
runs a genuine two-sample Mann-Whitney U test via scipy, the same test
this repo's own bench_stats.py already uses for its plain-vs-verifopt
comparisons. Gracefully skipped (with a note) if sample.json can't be
found or scipy isn't installed - the chart and estimates-based
comparison above still work either way.

Usage:
    plot_criterion_comparison.py LABEL1 ESTIMATES_JSON1 \\
        LABEL2 ESTIMATES_JSON2 [-o OUTPUT_PNG]

Example, matching run_mir_bench.sh's own visitor-use output:
    plot_criterion_comparison.py \\
        not_rw ../../benching_examples/visitor-ex/visitor-use/target-not-rw/criterion/visitor/visitor_not_rw/new/estimates.json \\
        mir_rw ../../benching_examples/visitor-ex/visitor-use/target-mir-rw/criterion/visitor/visitor_mir_rw/new/estimates.json
"""

import argparse
import json
import os
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


def run_significance_test(estimates1_path, estimates2_path):
    try:
        from scipy.stats import mannwhitneyu
    except ImportError:
        return None, "scipy not installed - skipping significance test"

    try:
        samples1 = load_per_iter_samples(estimates1_path)
        samples2 = load_per_iter_samples(estimates2_path)
    except FileNotFoundError as exc:
        return None, f"sample.json not found ({exc}) - skipping significance test"

    _, p_value = mannwhitneyu(samples1, samples2, alternative="two-sided")
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


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("label1", help="display label for the first benchmark (e.g. not_rw)")
    ap.add_argument("estimates1", help="path to the first benchmark's own estimates.json")
    ap.add_argument("label2", help="display label for the second benchmark (e.g. mir_rw)")
    ap.add_argument("estimates2", help="path to the second benchmark's own estimates.json")
    ap.add_argument("-o", "--output", default="criterion_comparison.png", help="output image path")
    args = ap.parse_args()

    try:
        e1 = load_estimates(args.estimates1)
        e2 = load_estimates(args.estimates2)
    except FileNotFoundError as exc:
        print(f"error: {exc}", file=sys.stderr)
        print(
            "(criterion only writes estimates.json after a benchmark has actually run - "
            "make sure both binaries have been run with --bench first)",
            file=sys.stderr,
        )
        sys.exit(1)

    p_value, skip_note = run_significance_test(args.estimates1, args.estimates2)

    unit_div, unit_name = pick_unit(max(e1["mean"], e2["mean"]))

    labels = [args.label1, args.label2]
    means = [e1["mean"] / unit_div, e2["mean"] / unit_div]
    mean_errs = [e1["std_dev"] / unit_div, e2["std_dev"] / unit_div]
    medians = [e1["median"] / unit_div, e2["median"] / unit_div]

    mean_pct = pct_change(e1["mean"], e2["mean"])
    median_pct = pct_change(e1["median"], e2["median"])

    fig, (ax_time, ax_zoom, ax_pct) = plt.subplots(3, 1, figsize=(7, 10))

    x = range(len(labels))
    ax_time.bar(x, means, yerr=mean_errs, capsize=6, color=["#888888", "#4c72b0"])
    ax_time.scatter(x, medians, marker="D", color="black", zorder=3, label="median")
    ax_time.set_xticks(list(x))
    ax_time.set_xticklabels(labels)
    ax_time.set_ylabel(f"time ({unit_name})")
    time_title = "mean time (bars) vs median (diamonds), \u00b1 std dev"
    if p_value is not None:
        sig_word = "significant" if p_value < SIGNIFICANCE_THRESHOLD else "not significant"
        time_title += f"\n(Mann-Whitney U: p={p_value:.4f}, {sig_word} at \u03b1={SIGNIFICANCE_THRESHOLD})"
    ax_time.set_title(time_title, fontsize=10)
    ax_time.legend()

    # A zero-based bar chart makes a std-dev that's only ~1-2% of the
    # mean essentially invisible - this panel plots the same mean +/-
    # std-dev as a point and error bar instead, with the y-axis scaled
    # to the actual data range (not zero-based), so the error bars
    # themselves are the thing you can actually see.
    ax_zoom.errorbar(
        x, means, yerr=mean_errs, fmt="o", capsize=8, markersize=8,
        color="#4c72b0", ecolor="black", elinewidth=1.5, capthick=1.5,
    )
    ax_zoom.set_xticks(list(x))
    ax_zoom.set_xticklabels(labels)
    ax_zoom.set_xlim(-0.5, len(labels) - 0.5)
    ax_zoom.set_ylabel(f"time ({unit_name})")
    ax_zoom.set_title("zoomed in: mean \u00b1 std dev only (y-axis not zero-based)", fontsize=10)

    ax_pct.bar(["mean", "median"], [mean_pct, median_pct], color=["#4c72b0", "#dd8452"])
    ax_pct.axhline(0, color="black", linewidth=0.8)
    ax_pct.set_ylabel(f"% change ({args.label2} vs {args.label1})")
    ax_pct.set_title(
        "bars diverging noticeably means outliers are pulling the mean-based\n"
        "comparison away from the typical (median) case",
        fontsize=10,
    )

    fig.tight_layout()
    fig.savefig(args.output, dpi=150)

    print(f"{args.label1}: mean={means[0]:.3f}{unit_name} median={medians[0]:.3f}{unit_name}")
    print(f"{args.label2}: mean={means[1]:.3f}{unit_name} median={medians[1]:.3f}{unit_name}")
    print(f"change: mean={mean_pct:+.1f}% median={median_pct:+.1f}%")
    if p_value is not None:
        sig_word = "significant" if p_value < SIGNIFICANCE_THRESHOLD else "not significant"
        print(f"significance (Mann-Whitney U): p={p_value:.4f} ({sig_word} at \u03b1={SIGNIFICANCE_THRESHOLD})")
    else:
        print(f"significance: {skip_note}")
    print(f"chart written to {args.output}")


if __name__ == "__main__":
    main()
