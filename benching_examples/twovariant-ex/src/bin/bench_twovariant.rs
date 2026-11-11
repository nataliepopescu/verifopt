// Timing harness for the static rewrites. The analysis runs on this binary
// (`cargo verifopt --release --bin bench_twovariant`), so the store
// describes exactly the dispatches that are timed.
//
// One invocation = one workload: fill an array of LEN objects, one untimed warm-up pass
// of ITERS/10 dispatches, one timed pass of ITERS dispatches, print
//   workload,iters,ns_per_dispatch,checksum
// Usage: bench_twovariant <workload> <iters> <seed>   |   bench_twovariant --list
//
// Patterns: mono = every object a Cat; alt = Cat, Dog, Cat, ...; rand =
// seeded random Cat/Dog. Each pattern has its own constructor function and
// its own loops, so each dispatch site only sees that pattern's objects:
// verifopt finds 1 target for mono, 2 for alt and rand.
// "direct" dispatches in this crate's loop; "wrap" calls the lib's
// per-pattern #[inline(never)] wrapper, which dispatches.
// Only the sum goes through black_box, never the objects (black_box has no
// MIR, so objects passed to it escape and widen the analysis).

use std::hint::black_box;
use std::time::Instant;
use twovariant_ex::{Animal, make_animal, make_cat, wrap_alt, wrap_mono, wrap_rand};

// Pattern length: fits in L1, too long for the branch predictor to memorize
// a random pattern.
const LEN: usize = 4096;

const WORKLOADS: [&str; 6] = [
    "mono_direct",
    "mono_wrap",
    "alt_direct",
    "alt_wrap",
    "rand_direct",
    "rand_wrap",
];

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

// A fixed array, not a Vec: the analysis loses elements written into a
// Vec's heap buffer (writes through the buffer pointer), so every site
// would fall back to CHA.
type Objs = [&'static dyn Animal; LEN];

#[inline(never)]
fn objects_mono() -> Objs {
    let mut v = [make_cat(); LEN];
    for i in 0..LEN {
        v[i] = make_cat();
    }
    v
}

#[inline(never)]
fn objects_alt() -> Objs {
    let mut v = [make_animal(0); LEN];
    for i in 0..LEN {
        v[i] = make_animal(i & 1);
    }
    v
}

#[inline(never)]
fn objects_rand(seed: u64) -> Objs {
    let mut s = seed | 1;
    let mut v = [make_animal(0); LEN];
    for i in 0..LEN {
        v[i] = make_animal((xorshift(&mut s) & 1) as usize);
    }
    v
}

#[inline(never)]
fn wl_mono_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].speak() as u64);
    }
    sum
}

#[inline(never)]
fn wl_mono_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_mono(objs[i % LEN]) as u64);
    }
    sum
}

#[inline(never)]
fn wl_alt_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].speak() as u64);
    }
    sum
}

#[inline(never)]
fn wl_alt_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_alt(objs[i % LEN]) as u64);
    }
    sum
}

#[inline(never)]
fn wl_rand_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].speak() as u64);
    }
    sum
}

#[inline(never)]
fn wl_rand_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_rand(objs[i % LEN]) as u64);
    }
    sum
}

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
