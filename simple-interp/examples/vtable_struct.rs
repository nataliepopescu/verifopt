use std::os::raw::c_void;

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

fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {});
}

fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {});
}

struct Bird {}

struct Cat {}

struct Dog {}

impl Animal for Bird {
    fn speak(&self) {
        println!("chirp");
    }
}

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

// requires this additional struct from 
// https://geo-ant.github.io/blog/2023/rust-dyn-trait-objects-fat-pointers/
#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct AnimalVtable {
    drop: fn(*mut c_void),
    size: usize,
    alignment: usize,
    speak: fn(*const c_void),
}

const POINTER_SIZE : usize = std::mem::size_of::<usize>();

// if copying into godbolt, make main `pub`
fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let num = args[1].parse().unwrap();
            let animal = get_animal(num);
            let cat = get_cat();
            let dog = get_dog();

            unsafe {
                let addr_of_data_ptr = &animal as *const _ as *const c_void as usize;
                let addr_of_pointer_to_vtable = addr_of_data_ptr + POINTER_SIZE;
                let ptr_to_ptr_to_vtable = addr_of_pointer_to_vtable as *const *const AnimalVtable;
                println!("animal vtable address: {:?}", *ptr_to_ptr_to_vtable);
                println!("animal vtable: {:?}", **ptr_to_ptr_to_vtable);
                let speak_func = (*(*ptr_to_ptr_to_vtable)).speak;
                speak_func(addr_of_data_ptr as *const c_void);


                let addr_of_cat_ptr = &cat as *const _ as *const c_void as usize;
                let addr_of_pointer_to_cat_vtable = addr_of_cat_ptr + POINTER_SIZE;
                let ptr_to_ptr_to_cat_vtable = addr_of_pointer_to_cat_vtable as *const *const AnimalVtable;
                println!("cat vtable address: {:?}", *ptr_to_ptr_to_cat_vtable);
                println!("cat vtable: {:?}", **ptr_to_ptr_to_cat_vtable);
                let cat_speak_func = (*(*ptr_to_ptr_to_cat_vtable)).speak;
                cat_speak_func(addr_of_cat_ptr as *const c_void);

                let addr_of_dog_ptr = &dog as *const _ as *const c_void as usize;
                let addr_of_pointer_to_dog_vtable = addr_of_dog_ptr + POINTER_SIZE;
                let ptr_to_ptr_to_dog_vtable = addr_of_pointer_to_dog_vtable as *const *const AnimalVtable;
                println!("dog vtable address: {:?}", *ptr_to_ptr_to_dog_vtable);
                println!("dog vtable: {:?}", **ptr_to_ptr_to_dog_vtable);
                let dog_speak_func = (*(*ptr_to_ptr_to_dog_vtable)).speak;
                dog_speak_func(addr_of_dog_ptr as *const c_void);

                if *ptr_to_ptr_to_vtable == *ptr_to_ptr_to_cat_vtable {
                    println!("IS CAT!");
                } else if *ptr_to_ptr_to_vtable == *ptr_to_ptr_to_dog_vtable {
                    println!("IS DOG!");
                } else {
                    println!("FALLBACK :(");
                }
            }
        }
    }
}
