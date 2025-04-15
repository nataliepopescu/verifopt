#![allow(dead_code)]

//use vstd::prelude::*;
//
//verus! {

/* array wrapper */

struct ArrayWrapper {
    array: [i32; 3],
}

impl ArrayWrapper {
    fn new() -> Self {
        Self {
            array: [7, 8, 9],
        }
    }

    fn get_timeslice_us(&self, idx: usize) -> u32 {
        match idx {
            0 => 10000,
            1 => 20000,
            2 => 50000,
            _ => 0, //panic!("invalid idx"),
        }
    }

    fn get_idx(&self) -> usize {
        use rand::prelude::IndexedRandom;
        let mut rng = rand::rng();
        let nums: Vec<i32> = (1..2).collect();

        for (idx, _) in self.array.iter().enumerate() {
            let rand = nums.choose(&mut rng).unwrap();
            if *rand != 2 {
                return idx;
            }
        }
        0
    }

    fn next(&self) -> u32 {
        let idx = self.get_idx();
        let time = self.get_timeslice_us(idx);
        time
    }
}

//} // verus!
