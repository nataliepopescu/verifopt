fn find_inner(eq: &mut dyn FnMut(usize) -> bool) -> Option<usize> {
    for i in 0..10 {
        if eq(i) { return Some(i); }
    }
    None
}

#[inline(never)]
fn find<T: PartialEq>(collection: &[T], target: &T) -> Option<usize> {
    let mut eq = |idx: usize| &collection[idx] == target;  // captures &T, T is free here
    find_inner(&mut eq)
}

fn main() {
    let data = vec![10usize, 20, 30, 5, 40];
    find(&data, &5usize);  // T = usize here
}
