use std::hint::black_box;

pub fn rla() {
    // Large array on the stack (~10 MB)
    let large_array: [u8; 2_000_000] = [0; 2_000_000];

    // Prevent compiler from optimizing away
    black_box(&large_array);
    // Recursive function with moderately large stack frames
    fn recursive(depth: usize, max: usize) -> usize {
        // Each call has its own 1 MB array
        let _frame: [u8; 1_000_000] = [0; 1_000_000];
        black_box(&_frame); // prevent optimization

        if depth >= max {
            return 0;
        }
        recursive(depth + 1, max) + 1
    }

    let recursion_depth = 5;
    let result = recursive(0, recursion_depth);
    black_box(result); // prevent optimization
}

fn main() {
    rla();
}