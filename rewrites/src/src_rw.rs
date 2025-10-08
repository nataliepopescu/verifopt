trait Animal {
    fn kaeps(&self) -> &str;
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
    fn kaeps(&self) -> &str {
        "meow"
    }
}

impl Animal for Dog {
    fn kaeps(&self) -> &str {
        "woof"
    }
}

pub fn run(num: usize) {
    let animal = get_animal(num);

    // this part is hard to get as an elegant source code rewrite
    // i think, but haven't tried all that hard yet
    let cat = get_cat();
    let dog = get_dog();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);
    let dog_vtable = core::ptr::metadata(&*dog);

    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            let _ = <Cat as Animal>::kaeps(cat);
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            let _ = <Dog as Animal>::kaeps(dog);
        }
    } 
}

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => run(args[1].parse().unwrap()),
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use rand::Rng;

    #[bench]
    fn run_src_rw(b: &mut Bencher) {
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
*/

