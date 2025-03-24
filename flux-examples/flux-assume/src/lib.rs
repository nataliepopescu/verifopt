/* From flux_support */

#[flux_rs::sig(fn(b:bool) ensures b)]
pub const fn assume(b: bool) {
    if !b {
        panic!("assume fails")
    }
}

/* Test compilation effect of assume */

pub struct Array {
    array: [usize; 3]
}

impl Array {
    pub fn new() -> Self {
        Self {
            array: [2, 1, 0]
        }
    }

    pub fn get_idx(&self) -> usize {
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

    pub fn get(&self) -> usize {
        let idx = self.get_idx();
        // can comment out below line
        assume(idx < 3);
        self.array[idx]
    }
}

