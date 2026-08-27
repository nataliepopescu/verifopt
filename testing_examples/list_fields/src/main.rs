#![feature(box_patterns)]

use rand::RngExt;
use std::fmt::Debug;
use std::hint::black_box;

#[derive(Clone, Debug)]
struct FunctionalList<T> {
    pub head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Clone, Debug)]
struct Node<T> {
    pub next: Link<T>,
    pub field: T,
}

impl<T> FunctionalList<T> 
where T: Clone + Default + Debug
{
    pub fn new() -> FunctionalList<T> {
        Self { head: None }
    }

    fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            next: self.head.take(),
            field: elem,
        });

        self.head = Some(new_node);
    }

    /*
    pub fn new_fixed_loop() -> FunctionalList<T> {
        let mut l = FunctionalList::<T>::new();

        for _ in 0..4 {
            l.push(T::default());
        }

        l
    }
    */

    pub fn new_fixed() -> FunctionalList<T> {
        let mut l = FunctionalList::<T>::new();

        l.push(T::default());
        l.push(T::default());
        l.push(T::default());
        l.push(T::default());

        l
    }

    /*
    pub fn new_arbitrary_loop() -> FunctionalList<T> {
        let mut l = FunctionalList::<T>::new();

        loop {
            if rand::rng().random_range(..2usize) == 1 {
                l.push(T::default());
            } else {
                break;
            }
        }

        l
    }
    */

    pub fn new_arbitrary() -> FunctionalList<T> {
        let mut l = FunctionalList::<T>::new();

        if rand::rng().random_range(..2usize) == 1 {
            l.push(T::default());
        }
        if rand::rng().random_range(..2usize) == 1 {
            l.push(T::default());
        }
        if rand::rng().random_range(..2usize) == 1 {
            l.push(T::default());
        }
        if rand::rng().random_range(..2usize) == 1 {
            l.push(T::default());
        }

        l
    }
}

/*
fn g() {
    todo!();
}

fn f<T>(l: &FunctionalList<T>) {
    l.field = g();
    if let Some(next) = l.next {
        if rand::rng().random_range(..2usize) == 1 {
            return;
        } else {
            f(next);
        }
    }
}
*/

#[inline(never)]
fn noop(num: usize) {
    black_box(num);
}

fn main() {
    let l = FunctionalList::<u32>::new_fixed();
    //let l = FunctionalList::<u32>::new_arbitrary();
    match l.head {
        Some(box link1) => match link1.next {
            Some(box link2) => {
                noop(1);
                black_box(link2.field);
                noop(2);
            }
            None => {}
        }
        None => {},
    }
    std::hint::black_box(l);
    //println!("functional list: {:?}", l);
}
