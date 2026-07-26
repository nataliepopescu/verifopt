use std::hint::black_box;

pub trait Animal {
    fn speak(&self) -> usize;
}

pub struct Cat {
    age: i32,
}

pub struct Dog {
    num_siblings: i32,
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
}

#[inline(never)]
fn noop(num: usize) {
    black_box(num);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let num: u32 = args[1].parse().unwrap();

            noop(1);
            let x = Cat {
                age: 7,
            };
            noop(2);
            let y = Dog {
                num_siblings: 2,
            };
            noop(3);

            let z : &dyn Animal;
            if num == 0 {
                z = &x;
            } else {
                z = &y;
            }

            let res = z.speak();
            black_box(res);
        }
    }
}
