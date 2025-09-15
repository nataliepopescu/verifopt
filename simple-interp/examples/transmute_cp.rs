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

            unsafe {
                let (_, animal_vtable) = std::mem::transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(&animal);
                let (_, cat_vtable) = std::mem::transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(&cat);
                let (_, dog_vtable) = std::mem::transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(&dog);
                //println!("animal_vtable: {:?}", animal_vtable);
                //println!("cat_vtable: {:?}", cat_vtable);
                //println!("dog_vtable: {:?}", dog_vtable);

                if animal_vtable == cat_vtable {
                    let rawptr = Box::into_raw(animal) as *const ();
                    let cat: &Cat = std::mem::transmute::<*const (), &Cat>(rawptr);
                    //println!("IS CAT!");
                    <Cat as Animal>::speak(cat);
                } else if animal_vtable == dog_vtable {
                    let rawptr = Box::into_raw(animal) as *const ();
                    let dog: &Dog = std::mem::transmute::<*const (), &Dog>(rawptr);
                    //println!("IS DOG!");
                    <Dog as Animal>::speak(dog);
                }
                //else {
                //    println!("FALLBACK :(");
                //    animal.speak();
                //}
            }
        }
    }
}
