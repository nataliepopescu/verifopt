#![feature(ptr_metadata)]
#![allow(dead_code)]

use std::time::Instant;

pub trait Animal {
    fn speak(&mut self) -> usize;
    fn walk(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat { speak_ctr: 0 })
    } else {
        Box::new(Dog { speak_ctr: 0 })
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat { speak_ctr: 0 });
}

pub struct Cat {
    speak_ctr: u64,
}

pub struct Dog {
    speak_ctr: u64,
}

impl Cat {
    fn meow(&self) -> usize {
        0
    }
}

impl Animal for Cat {
    fn speak(&mut self) -> usize {
        self.speak_ctr += 1;
        11111
    }
    fn walk(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&mut self) -> usize {
        self.speak_ctr += 1;
        22222
    }
    fn walk(&self) -> usize {
        44444
    }
}

// TODO inc struct field (side effect) + print at end to confirm used
#[inline(never)]
fn wrap_call(cat: &mut Cat) {
    let _ = <Cat as Animal>::speak(cat);
}

fn main() {
    let mut cat = Cat { speak_ctr: 0 };
    let warmup = 100000;
    let runs = 1000000;

    for _ in 0..warmup {
        wrap_call(&mut cat);
    }

    let mut times = Vec::new();
    for _ in 0..runs {
        let start = Instant::now();
        wrap_call(&mut cat);
        let duration = start.elapsed().as_nanos();
        times.push(duration);
    }

    // FIXME not handling overflow
    let sum: u128 = Iterator::sum(times.iter());
    let mean = f64::from(sum as u32) / (times.len() as f64);
    println!("mean: {:?}", mean);
}

