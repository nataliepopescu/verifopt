// Timing harness for the static rewrites on the double-dispatch visitor,
// like twovariant-ex's bench_twovariant (see there). The analysis runs on
// this binary (`cargo verifopt --release --bin bench_visitor`).
//
// One invocation = one workload: fill the object arrays, one untimed warm-up
// pass of ITERS/10 dispatches, one timed pass of ITERS, print
//   workload,iters,ns_per_dispatch,checksum
// Usage: bench_visitor <workload> <iters> <seed>   |   bench_visitor --list
// ("dispatch" = one `visit_work` call, i.e. the whole double dispatch.)

use std::hint::black_box;
use std::time::Instant;
use visitor_use::bench::*;

const WORKLOADS: [&str; 5] = ["mono", "dynanimal", "dynvisitor", "both_alt", "both_rand"];

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

    // Each arm calls its own functions directly (no fn-pointer table).
    match wl {
        "mono" => {
            let objs = objects_mono();
            black_box(wl_mono(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_mono(&objs, iters));
            report(wl, iters, t, sum);
        }
        "dynanimal" => {
            let objs = objects_dynanimal(seed);
            black_box(wl_dynanimal(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_dynanimal(&objs, iters));
            report(wl, iters, t, sum);
        }
        "dynvisitor" => {
            let objs = objects_dynvisitor(seed);
            black_box(wl_dynvisitor(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_dynvisitor(&objs, iters));
            report(wl, iters, t, sum);
        }
        "both_alt" => {
            let objs = objects_both_alt();
            black_box(wl_both_alt(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_both_alt(&objs, iters));
            report(wl, iters, t, sum);
        }
        "both_rand" => {
            let objs = objects_both_rand(seed);
            black_box(wl_both_rand(&objs, warm));
            let t = Instant::now();
            let sum = black_box(wl_both_rand(&objs, iters));
            report(wl, iters, t, sum);
        }
        _ => {
            eprintln!("unknown workload {wl}; --list shows them");
            std::process::exit(2);
        }
    }
}
