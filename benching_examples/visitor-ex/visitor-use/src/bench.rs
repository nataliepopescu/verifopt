// Objects and timed loops for the harness (src/bin/bench_visitor.rs), set
// up like twovariant-ex/src/bench.rs (which says why: fixed arrays instead
// of Vecs, opaque fill-loop bounds, loops in a lib next to the targets).
//
// Double dispatch: the loop's `a.visit_work(v, i)` (dispatch 1, in this
// crate) runs Cat/Dog::visit_work in visitor-decl, whose `av.work_cat` /
// `av.work_dog` (dispatch 2) runs a visitor here; SpeakBetterDogs::work_cat
// and SpeakBetterCats::work_dog call `a.work(x)` (dispatch 3, in this
// crate). Dispatch 2 lives in visitor-decl, which can't name its targets
// (the visitors are downstream), so it stays a dyn call unless LTO
// resolves it.
//
// Workloads (each with its own constructors and loops, so each loop's site
// only sees its own objects); each in two forms: _direct dispatches in the
// loop, _wrap through a per-workload #[inline(never)] wrapper:
//   mono        every animal a Dog, every visitor SpeakBetterDogs
//   dynanimal   random Cat/Dog, the visitor a fixed SpeakBetterDogs
//   dynvisitor  a fixed Dog (static call of Dog::visit_work), random visitors
//   both_alt    Cat, Dog, Cat, ... with SBC, SBC, SBD, SBD, ... (all 4 pairs)
//   both_rand   random animal and random visitor

use crate::{SpeakBetterCats, SpeakBetterDogs};
use std::hint::black_box;
use visitor_decl::{Animal, AnimalVisitor, Cat, Dog};

// Fits in L1, too long for the branch predictor to memorize a random pattern.
pub const LEN: usize = 4096;

pub type Animals = [&'static dyn Animal; LEN];
pub type Visitors = [&'static dyn AnimalVisitor; LEN];

fn xorshift(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

#[inline(never)]
pub fn make_dog() -> &'static dyn Animal {
    &Dog {}
}

#[inline(never)]
pub fn make_animal(sel: usize) -> &'static dyn Animal {
    if sel == 0 { &Cat {} } else { &Dog {} }
}

#[inline(never)]
pub fn make_sbd() -> &'static dyn AnimalVisitor {
    &SpeakBetterDogs {}
}

#[inline(never)]
pub fn make_visitor(sel: usize) -> &'static dyn AnimalVisitor {
    if sel == 0 { &SpeakBetterCats {} } else { &SpeakBetterDogs {} }
}

#[inline(never)]
pub fn objects_mono() -> (Animals, Visitors) {
    let mut a = [make_dog(); LEN];
    let mut v = [make_sbd(); LEN];
    for i in black_box(0)..black_box(LEN) {
        a[i] = make_dog();
        v[i] = make_sbd();
    }
    (a, v)
}

#[inline(never)]
pub fn objects_dynanimal(seed: u64) -> Animals {
    let mut s = seed | 1;
    let mut a = [make_animal(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        a[i] = make_animal((xorshift(&mut s) & 1) as usize);
    }
    a
}

#[inline(never)]
pub fn objects_dynvisitor(seed: u64) -> Visitors {
    let mut s = seed | 1;
    let mut v = [make_visitor(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        v[i] = make_visitor((xorshift(&mut s) & 1) as usize);
    }
    v
}

#[inline(never)]
pub fn objects_both_alt() -> (Animals, Visitors) {
    let mut a = [make_animal(0); LEN];
    let mut v = [make_visitor(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        a[i] = make_animal(i & 1);
        v[i] = make_visitor((i >> 1) & 1);
    }
    (a, v)
}

#[inline(never)]
pub fn objects_both_rand(seed: u64) -> (Animals, Visitors) {
    let mut s = seed | 1;
    let mut a = [make_animal(0); LEN];
    let mut v = [make_visitor(0); LEN];
    for i in black_box(0)..black_box(LEN) {
        let r = xorshift(&mut s);
        a[i] = make_animal((r & 1) as usize);
        v[i] = make_visitor(((r >> 1) & 1) as usize);
    }
    (a, v)
}

// One #[inline(never)] wrapper per workload for the _wrap loops: the
// dispatch then happens inside a function the loop calls, so a rewritten
// call can't be inlined into the loop (twovariant's wrap_* likewise); each
// wrapper only sees its own workload's objects.
#[inline(never)]
pub fn wrap_mono(a: &dyn Animal, v: &dyn AnimalVisitor, x: usize) -> usize {
    a.visit_work(v, x)
}

#[inline(never)]
pub fn wrap_dynanimal(a: &dyn Animal, v: &SpeakBetterDogs, x: usize) -> usize {
    a.visit_work(v, x)
}

#[inline(never)]
pub fn wrap_dynvisitor(d: &Dog, v: &dyn AnimalVisitor, x: usize) -> usize {
    d.visit_work(v, x)
}

#[inline(never)]
pub fn wrap_both_alt(a: &dyn Animal, v: &dyn AnimalVisitor, x: usize) -> usize {
    a.visit_work(v, x)
}

#[inline(never)]
pub fn wrap_both_rand(a: &dyn Animal, v: &dyn AnimalVisitor, x: usize) -> usize {
    a.visit_work(v, x)
}

#[inline(never)]
pub fn wl_mono_direct(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(a[i % LEN].visit_work(v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_mono_wrap(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_mono(a[i % LEN], v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_dynanimal_direct(a: &Animals, iters: u64) -> u64 {
    let v = SpeakBetterDogs {};
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(a[i % LEN].visit_work(&v, i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_dynanimal_wrap(a: &Animals, iters: u64) -> u64 {
    let v = SpeakBetterDogs {};
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_dynanimal(a[i % LEN], &v, i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_dynvisitor_direct(v: &Visitors, iters: u64) -> u64 {
    let d = Dog {};
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(d.visit_work(v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_dynvisitor_wrap(v: &Visitors, iters: u64) -> u64 {
    let d = Dog {};
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_dynvisitor(&d, v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_both_alt_direct(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(a[i % LEN].visit_work(v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_both_alt_wrap(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_both_alt(a[i % LEN], v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_both_rand_direct(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(a[i % LEN].visit_work(v[i % LEN], i) as u64);
    }
    sum
}

#[inline(never)]
pub fn wl_both_rand_wrap(objs: &(Animals, Visitors), iters: u64) -> u64 {
    let (a, v) = objs;
    let mut sum = 0u64;
    for i in 0..iters as usize {
        sum = sum.wrapping_add(wrap_both_rand(a[i % LEN], v[i % LEN], i) as u64);
    }
    sum
}
