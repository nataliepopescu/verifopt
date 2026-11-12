// Timing harness for the static rewrites. The analysis runs on this binary
// (`cargo verifopt --release --bin bench_twovariant`), so the store
// describes exactly the dispatches that are timed.
//
// One invocation = one workload: fill an array of LEN objects, one untimed warm-up pass
// of ITERS/10 dispatches, one timed pass of ITERS dispatches, print
//   workload,iters,ns_per_dispatch,checksum
// Usage: bench_twovariant <workload> <iters> <seed>   |   bench_twovariant --list
//
// The objects and the timed loops are in the lib (src/bench.rs, which says
// why). Only the sum goes through black_box, never the objects (black_box
// has no MIR, so objects passed to it escape and widen the analysis).

use std::hint::black_box;
use std::time::Instant;
use twovariant_ex::bench::*;

const WORKLOADS: [&str; 6] = [
    "mono_direct",
    "mono_wrap",
    "alt_direct",
    "alt_wrap",
    "rand_direct",
    "rand_wrap",
];

fn report(wl: &str, iters: u64, start: Instant, sum: u64) {
    let ns = start.elapsed().as_nanos() as f64 / iters as f64;
    println!("{wl},{iters},{ns:.4},{sum}");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && args[1] == "--list" {
        for w in WORKLOADS {
            println!("{w}");
        }
        return;
    }
    if args.len() != 4 {
        eprintln!("usage: {} <workload> <iters> <seed> | --list", args[0]);
        std::process::exit(2);
    }
    let wl = args[1].as_str();
    let iters: u64 = args[2].parse().expect("iters");
    let seed: u64 = args[3].parse().expect("seed");
    let warm = (iters / 10).max(1);

    // Each arm calls its own functions directly (no fn-pointer table: the
    // analysis would have to follow the calls through it).
    match wl {
        "mono_direct" => {
            let objs = objects_mono();
            black_box(wl_mono_direct(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_mono_direct(&objs, iters));
            report(wl, iters, t, sum);
        }
        "mono_wrap" => {
            let objs = objects_mono();
            black_box(wl_mono_wrap(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_mono_wrap(&objs, iters));
            report(wl, iters, t, sum);
        }
        "alt_direct" => {
            let objs = objects_alt();
            black_box(wl_alt_direct(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_alt_direct(&objs, iters));
            report(wl, iters, t, sum);
        }
        "alt_wrap" => {
            let objs = objects_alt();
            black_box(wl_alt_wrap(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_alt_wrap(&objs, iters));
            report(wl, iters, t, sum);
        }
        "rand_direct" => {
            let objs = objects_rand(seed);
            black_box(wl_rand_direct(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_rand_direct(&objs, iters));
            report(wl, iters, t, sum);
        }
        "rand_wrap" => {
            let objs = objects_rand(seed);
            black_box(wl_rand_wrap(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_rand_wrap(&objs, iters));
            report(wl, iters, t, sum);
        }
        _ => {
            eprintln!("unknown workload {wl}; --list shows them");
            std::process::exit(2);
        }
    }
}
