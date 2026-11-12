// Objects and timed loops for the harness (src/bin/bench_twovariant.rs).
// They live in the lib, next to Cat and Dog: a rewritten call in another
// crate can't reach a small cross-crate-inlinable target like
// `Cat::speak` (rustc doesn't export it, and the rewrite runs after the
// crate's items are fixed), so dispatch sites in the binary stay dyn calls.
//
// Patterns: mono = every object a Cat; alt = Cat, Dog, Cat, ...; rand =
// seeded random Cat/Dog. Each pattern has its own constructor function and
// its own loops, so each dispatch site only sees that pattern's objects:
// verifopt finds 1 target for mono, 2 for alt and rand. "direct" dispatches
// in the loop itself; "wrap" calls the per-pattern #[inline(never)] wrapper,
// which dispatches.

use crate::{Animal, make_animal, make_cat, wrap_alt, wrap_mono, wrap_rand};
use std::hint::black_box;

// Pattern length: fits in L1, too long for the branch predictor to memorize
// a random pattern.
pub const LEN: usize = 4096;

// A fixed array, not a Vec: the analysis loses elements written into a
// Vec's heap buffer (writes through the buffer pointer), so every site
// would fall back to CHA. The fill loops run over black_box(0)..black_box(LEN):
// the analysis walks a loop body once (CLAUDE.md §6, loops walked once), so
// with constant bounds it would see only i = 0 and prune the loop's exit;
// opaque bounds keep the exit and make `i` unknown, so `i & 1` is both 0
// and 1.
pub type Objs = [&'static dyn Animal; LEN];

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[inline(never)]
pub fn objects_mono() -> Objs {
    let mut v = [make_cat(); LEN];
    for i in black_box(0)..black_box(LEN) {
        v[i] = make_cat();
    }
    v
}

#[inline(never)]
pub fn objects_alt() -> Objs {
    let mut v = [make_animal(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        v[i] = make_animal(i & 1);
    }
    v
}

#[inline(never)]
pub fn objects_rand(seed: u64) -> Objs {
    let mut s = seed | 1;
    let mut v = [make_animal(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        v[i] = make_animal((xorshift(&mut s) & 1) as usize);
    }
    v
}

#[inline(never)]
pub fn wl_mono_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].work(i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_mono_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_mono(objs[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_alt_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].work(i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_alt_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_alt(objs[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_rand_direct(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(objs[i % LEN].work(i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_rand_wrap(objs: &Objs, iters: u64) -> u64 {
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_rand(objs[i % LEN], i) as u64);
    }
    sum
}
