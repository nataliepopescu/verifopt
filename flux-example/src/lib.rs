#![allow(dead_code)]

use rand::prelude::*;

#[flux_rs::sig(fn(x: i32) -> i32{v: x < v})]
fn inc(x: i32) -> i32 {
    x + 1
}

struct ArrayWrapper {
    array: [i32; 3],
}

impl ArrayWrapper {
    fn new() -> Self {
        Self {
            array: [7, 8, 9],
        }
    }

    fn get_timeslice_us_SIM(&self, idx: usize) -> u32 {
        match idx {
            0 => 10000,
            1 => 20000,
            2 => 50000,
            _ => panic!("invalid idx"),
        }
    }

    fn get_next_ready_proc_node_SIM(&self) -> (Option<i32>, usize) {
        let mut rng = rand::rng();
        let mut nums: Vec<i32> = (1..10).collect();

        for (idx, num) in self.array.iter().enumerate() {
            let randnum = nums.choose(&mut rng).unwrap();
            if *randnum < 5 {
                return (None, idx);
            }
        }
        (None, 0)
    }

    fn next_SIM(&self) {
        let (_, idx) = self.get_next_ready_proc_node_SIM();
        let time = self.get_timeslice_us_SIM(idx);
    }
}

