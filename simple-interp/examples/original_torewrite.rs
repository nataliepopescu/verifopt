#![feature(test)]

extern crate test;

trait Animal {
    fn speak(&self);
}

fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {});
    } else {
        return Box::new(Dog {});
    }
}

#[inline(always)]
fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

#[inline(always)]
fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

struct Cat {}
struct Dog {}

impl Animal for Cat {
    fn speak(&self) {
        println!("meow");
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("woof");
    }
}

fn run(num: usize) {
    let animal = get_animal(num);
    let _cat = get_cat();
    let _dog = get_dog();
    animal.speak();
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => run(args[1].parse().unwrap()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::Rng;

    #[bench]
    fn run_orig(b: &mut Bencher) {
        let mut nums_vec: Vec<usize> = vec![];
        for _ in 0..1000 {
            nums_vec.push(rand::rng().random_range(..2));
        }
        let nums: &[usize] = &nums_vec[..];
        let mut idx = 0;

        b.iter(|| {
            let num = test::black_box(nums[idx % 1000]);
            idx += 1;
            run(num)
        })
    }
}

