#![feature(ptr_metadata)]

trait Animal {
    fn speak(&self) -> &str;
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

pub struct Cat {}
pub struct Dog {}

impl Animal for Cat {
    fn speak(&self) -> &str{
        "meow"
    }
}

impl Animal for Dog {
    fn speak(&self) -> &str {
        "woof"
    }
}

pub fn run_best(cat: &Cat) -> String {
    return <Cat as Animal>::speak(cat).to_string();
}

pub fn run_best_normalized(num: usize, cat: &Cat) -> String {
    let _animal = get_animal(num);
    let _cat = get_cat();
    return <Cat as Animal>::speak(cat).to_string();
}

pub fn run_best_normalized_fallback(num: usize, cat: &Cat) -> String {
    let _animal = get_animal(num);
    let _cat = get_cat();
    let _dog = get_dog();
    return <Cat as Animal>::speak(cat).to_string();
}

pub fn run_not_rw(num: usize) -> String {
    let animal = get_animal(num);
    let _cat = get_cat();
    animal.speak().to_string()
}

pub fn run_not_rw_fallback(num: usize) -> String {
    let animal = get_animal(num);
    let _cat = get_cat();
    let _dog = get_dog();
    animal.speak().to_string()
}

pub fn run_src_rw_into_raw(num: usize) -> String {
    let animal = get_animal(num);
    let cat = get_cat();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);
    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            return <Cat as Animal>::speak(cat).to_string();
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            return <Dog as Animal>::speak(dog).to_string();
        }
    }
}

pub fn run_src_rw_transmutes(num: usize) -> String {
    let animal = get_animal(num);
    let cat = get_cat();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);

    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            return <Cat as Animal>::speak(cat).to_string();
        }
    } else {
        unsafe {
            let raw_animal: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            return <Dog as Animal>::speak(dog).to_string();
        }
    }
}

pub fn run_src_rw_into_raw_fallback(num: usize) -> String {
    let animal = get_animal(num);
    let cat = get_cat();
    let dog = get_dog();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);
    let dog_vtable = core::ptr::metadata(&*dog);

    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal = Box::into_raw(animal) as *const ();
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            return <Cat as Animal>::speak(cat).to_string();
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let raw_animal = Box::into_raw(animal) as *const ();
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            return <Dog as Animal>::speak(dog).to_string();
        }
    } else {
        return animal.speak().to_string();
    }
}

pub fn run_src_rw_transmutes_fallback(num: usize) -> String {
    let animal = get_animal(num);
    let cat = get_cat();
    let dog = get_dog();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);
    let dog_vtable = core::ptr::metadata(&*dog);

    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            return <Cat as Animal>::speak(cat).to_string();
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let raw_animal: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            return <Dog as Animal>::speak(dog).to_string();
        }
    } else {
        return animal.speak().to_string();
    }
}

// if copying into godbolt, make main `pub`
/*
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cat: &Cat = &Cat {};

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let s = run_best(args[1].parse().unwrap(), cat);
            println!("{}", s);
        },
    }
}
*/

