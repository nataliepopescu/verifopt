#![allow(dead_code)]

/* Simplified MLFQ panic */

//#[flux_rs::refined_by(array_len: int)]
//#[flux_rs::invariant(array_len == 3)]
struct ArrayWrapper {
    //#[field({[i32; 3]})]
    array: [i32; 3],
}

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

/* array version */

#[flux_rs::sig(fn(i:usize{i < 3}) -> u32{r: r > 0})]
fn get_timeslice_us(idx: usize) -> u32 {
    match idx {
        0 => 10000,
        1 => 20000,
        2 => 50000,
        _ => 0, //panic!("invalid idx"),
    }
}

#[flux_rs::sig(fn([i32; 3]) -> usize{i: i < 3})]
fn get_idx(array: [i32; 3]) -> usize {
    use rand::prelude::IndexedRandom;
    let mut rng = rand::rng();
    let nums: Vec<i32> = (1..2).collect();

    /*
    // take(3) does not help
    for (idx, _) in array.iter().enumerate() {
        // FIXME how to verify without assume? uses panic
        //flux_assume::assume(idx < 3);
        let idx2 = get_flux_info_on_idx(idx);
        let rand = nums.choose(&mut rng).unwrap();
        if *rand != 2 {
            return idx2;
        }
    }
    */

    // this also cannot be verified without assume!!
    for idx in 0..array.len() {
        //flux_assume::assume(idx < 3);
        let idx2 = get_flux_info_on_idx(idx);
        let rand = nums.choose(&mut rng).unwrap();
        if *rand != 2 {
            return idx2;
        }
    }
    0
}

// doesn't use a panic but does introduce more code
#[flux_rs::sig(fn(i:usize{i < 3}) -> usize{i: i < 3})]
fn get_flux_info_on_idx(idx: usize) -> usize {
    idx
}

fn next() -> u32 {
    let arr = [7, 8, 9];
    let idx = get_idx(arr);
    let time = get_timeslice_us(idx);
    time
}


