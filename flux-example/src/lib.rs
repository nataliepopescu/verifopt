#![allow(dead_code)]

use rand::prelude::*;

/* From flux_support */

#[flux_rs::sig(fn(b:bool) ensures b)]
const fn flux_assume(b: bool) {
    if !b {
        panic!("assume fails")
    }
}

/* Simplified MLFQ panic */

struct ArrayWrapper {
    array: [i32; 3],
}

impl ArrayWrapper {
    fn new() -> Self {
        Self {
            array: [7, 8, 9],
        }
    }

    #[flux_rs::sig(fn(&ArrayWrapper[@a], i:usize{i < 3}) -> u32{r: r > 0})]
    fn get_timeslice_us_sim(&self, idx: usize) -> u32 {
        match idx {
            0 => 10000,
            1 => 20000,
            2 => 50000,
            _ => 0, //panic!("invalid idx"),
        }
    }

    #[flux_rs::sig(fn(&ArrayWrapper[@a]) -> usize{i: i < 3})]
    fn get_next_ready_proc_node_sim(&self) -> usize {
        let mut rng = rand::rng();
        let nums: Vec<i32> = (1..10).collect();

        for (idx, _) in self.array.iter().enumerate() {
            // FIXME how to verify without this helper? introduces a panic
            flux_assume(idx < 3);
            let randnum = nums.choose(&mut rng).unwrap();
            if *randnum < 5 {
                return idx;
            }
        }
        0
    }

    fn next_sim(&self) -> u32 {
        let idx = self.get_next_ready_proc_node_sim();
        //flux_assume(idx < 3);
        let time = self.get_timeslice_us_sim(idx);
        time
    }
}

