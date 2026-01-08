/* From flux_support */

#[flux_rs::sig(fn(b: bool[true]))]
pub fn assert(_: bool) {}

#[flux_rs::sig(fn(b:bool) ensures b)]
pub const fn assume(b: bool) {
    if !b {
        panic!("assume fails")
    }
}

/* Try layout_scalar_valid_range hack */

//#![feature(rustc_attrs)]
//
//#[rustc_layout_scalar_valid_range_start(0)]
//#[rustc_layout_scalar_valid_range_end(2)]
//struct Index(usize);

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
    //pub fn get_idx(&self) -> Index {
        use rand::prelude::IndexedRandom;
        let mut rng = rand::rng();
        let nums: Vec<i32> = (1..2).collect();

        for (idx, _) in self.array.iter().enumerate() {
            let rand = nums.choose(&mut rng).unwrap();
            if *rand != 2 {
                return idx;
                //return unsafe { Index(idx) };
            }
        }
        0
        //unsafe { Index(0) }
    }

    pub fn get(&self) -> usize {
        let idx = self.get_idx();

        // succeeds verif
        assume(idx < 3);
        self.array[idx]

        // fails verif
        //self.array[Index::usize_to_index(idx).idx]

        // fails verif
        //let restricted_idx = unsafe { Index(idx) };
        //self.array[restricted_idx.0]

        // fails verif
        //self.array[idx.0]
    }
}

