#![feature(ptr_metadata)]

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

            //let animal: &dyn Animal = &Cat{};
            //let cat: &dyn Animal = &Cat{};
            //let dog: &dyn Animal = &Dog{};
            //let raw_animal = &*animal; // as *const _ as *const (); //Box::into_raw(animal) as *const ();
            //let raw_cat = &*cat; // as *const _ as *const (); //Box::into_raw(cat) as *const ();
            //let raw_dog = &*dog; // as *const _ as *const (); //Box::into_raw(dog) as *const ();

            let animal_vtable = core::ptr::metadata(&*animal);
            let cat_vtable = core::ptr::metadata(&*cat);
            let dog_vtable = core::ptr::metadata(&*dog);

            let raw_animal = Box::into_raw(animal) as *const ();

            //println!("animal_vtable: {:?}", animal_vtable);
            //println!("cat_vtable: {:?}", cat_vtable);
            //println!("dog_vtable: {:?}", dog_vtable);

            if animal_vtable == cat_vtable {
                unsafe {
                    let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
                    <Cat as Animal>::speak(cat);
                    //println!("IS CAT!");
                }
            } else if animal_vtable == dog_vtable {
                unsafe {
                    let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
                    <Dog as Animal>::speak(dog);
                    //println!("IS DOG!");
                }
            } 
        }
    }
}
