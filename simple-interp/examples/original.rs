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

//struct Bird {}
struct Cat {}
struct Dog {}

//impl Animal for Bird {
//    fn speak(&self) {
//        println!("chirp");
//    }
//}

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
            // TODO force inline? should really just be looking up a constant
            let _cat = get_cat();
            let _dog = get_dog();
            animal.speak();
        }
    }
}
