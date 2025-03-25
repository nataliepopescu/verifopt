#![allow(dead_code)]

/* Simplified MLFQ panic */

//#[flux_rs::refined_by(array_len: int)]
//#[flux_rs::invariant(array_len == 3)]

//#[flux_rs::refined_by()]
struct ArrayWrapper {
    //#[field({[i32; 3]})]
    array: [i32; 3],
}

//flux_rs::defs! {
//    fn len(aw: ArrayWrapper) -> int { 3 }
//}

impl ArrayWrapper {
    fn new() -> Self {
        Self {
            array: [7, 8, 9],
        }
    }

    #[flux_rs::sig(fn(&ArrayWrapper[@a], i:usize{i < 3}) -> u32{r: r > 0})]
    fn get_timeslice_us(&self, idx: usize) -> u32 {
        match idx {
            0 => 10000,
            1 => 20000,
            2 => 50000,
            _ => 0, //panic!("invalid idx"),
        }
    }

    #[flux_rs::sig(fn(&ArrayWrapper[@a]) -> usize{i: i < 3})]
    //requires len(a) == 3)]
    fn get_idx(&self) -> usize {
        use rand::prelude::IndexedRandom;
        let mut rng = rand::rng();
        let nums: Vec<i32> = (1..2).collect();

        for (idx, _) in self.array.iter().enumerate() {
            // FIXME how to verify without this helper? uses panic
            flux_assume::assume(idx < 3);
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

