
pub fn decide_simple(input: usize) -> usize {
    if input == 0 {
        input + 1
    } else if input == 3 {
        input - 1
    } else {
        0
    }
}

pub fn decide_even(input: usize) -> usize {
    input * 2
}
