
/*
fn find_inner(eq: &mut dyn FnMut(usize) -> bool) -> Option<usize> {
    for i in 0..10 {
        if eq(i) {
            return Some(i);
        }
    }
    None
}

// This middle layer is the key difference - it's generic over T
fn find<T: PartialEq>(eq: &mut dyn FnMut(T) -> bool) -> Option<T> {
    find_inner(eq)
}

fn main() {
    let target = 5usize;

	// monomorphized
    //let result = find_inner(&mut |idx| idx == target);
    //println!("{:?}", result); // Some(5)

    // generics
    //let data = vec![10usize, 20, 30, 5, 40];
    //lookup(&data, &5usize);
    find(&mut |idx| idx == target);
}

// Mirrors hashbrown's find_inner - always works on raw usize indices
fn find_inner(eq: &mut dyn FnMut(usize) -> bool) -> Option<usize> {
    for i in 0..10 {
        if eq(i) { return Some(i); }
    }
    None
}

// Mirrors hashbrown's find - generic over T, coerces into find_inner
fn find<T: PartialEq>(collection: &[T], eq: &mut dyn FnMut(&T) -> bool) -> Option<usize> {
    find_inner(&mut |idx| eq(&collection[idx]))
}
*/



use std::hint::black_box;
use std::marker::PhantomData;

struct Bucket<T> {
    idx: usize,
    phantom: PhantomData<T>,
}

impl<T> Bucket<T> {
    fn as_ref<'a>(&self) -> &'a T {
    }
}

fn equivalent_key<T: PartialEq>(y: T) -> impl Fn(&T) -> bool {
    move |x| *x == y
}

struct RawTable<T> {
    phantom: PhantomData<T>,
}

impl<T> RawTable<T> {
    fn bucket(idx: usize) -> Bucket<T> {
        Bucket {
            idx,
            phantom: PhantomData,
        }
    }

    fn inner_get<T>(&self, eq: impl FnMut(&T) -> bool) -> Option<T> {
        find(eq)
    }
    
    fn find<T>(&self, mut eq: impl FnMut(&T) -> bool) -> Option<T> {
        find_inner(&mut |index| eq(self.bucket(index)))
    }
    
    fn find_inner(&self, eq: &mut dyn FnMut(usize) -> bool) -> Option<usize> {
        for i in 0..10 {
            if eq(i) { return Some(i); }
        }
        None
    }
}

fn main() {
    let res = inner_get(equivalent_key(5usize));
    black_box(res);
}

