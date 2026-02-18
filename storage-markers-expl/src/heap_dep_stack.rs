use rand::Rng;
use std::hint::black_box;

pub fn hds() {
    let mut slots: [Option<Box<[u64]>>; 500] = std::array::from_fn(|_| None);

    let mut rng = rand::rng();

    for _ in 0..100_000 {
        let idx = rng.random_range(0..500);

        if slots[idx].is_none() {
            let len = rng.random_range(1..=10_000);

            // allocate (no elements created/array uninitialized)
            let mut v = Vec::with_capacity(len);
            // initialize them all to 0
            v.resize(len, 0);
            // if you wanted to do both at the same time:
            // let v = vec![0u64; len];

            // use one of the entries in the array so it's not optimized away
            if len > 0 {
                v[0] = idx as u64;
            }

            slots[idx] = Some(v.into_boxed_slice()); // convert to Box of fixed size
        } else {
            // No explicit free needed here
            slots[idx] = None; // drop happens here
        }
    }

    let live = slots.iter().filter(|e| e.is_some()).count();
    black_box(live);
    // println!("Done. Live allocations remaining: {live}");
}

fn main() {
    hds();
}