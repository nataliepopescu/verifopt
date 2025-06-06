#![allow(dead_code)]

use vstd::prelude::*;

verus! {

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

    fn get_timeslice_us(&self, idx: usize) -> u32
      requires
        idx < 3
    {
        match idx {
            0 => 10000,
            1 => 20000,
            2 => 50000,
            _ => 0, //panic!("invalid idx"),
        }
    }

    //fn special_inc(val: u8) -> (r: u8)
    //  requires
    //    val + 1 < 256,
    //  ensures
    //    r == val + 1,
    //{
    //    val + 1
    //}

    fn get_idx(&self) -> (i: usize)
      ensures
        i < 3
    {
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

        //let mut ctr: u8 = 0;
        //let mut idx: u8 = 0;
        //for idx in 0..3
        //  invariant
        //    idx <= 3,
        //    ctr <= 3,
        //    idx == ctr,
        //{
        //    if ctr == 2 {
        //      return idx;
        //    }
        //    ctr = Self::special_inc(ctr);
        //}
    }

    fn next(&self) -> u32 {
        let idx = self.get_idx();
        let time = self.get_timeslice_us(idx);
        time
    }
}

fn main() {}

} // verus!
