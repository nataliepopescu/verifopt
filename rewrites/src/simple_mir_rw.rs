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

//#[inline(always)]
//fn get_dog() -> Box<dyn Animal> {
//    return Box::new(Dog {});
//}

struct Cat {}
struct Dog {}

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

pub fn run(num: usize) -> String {
    //println!("in run");
    let animal = get_animal(num);
    //println!("got animal");
    let _cat = get_cat();
    //println!("got cat");
    animal.speak().to_string()
}

// if copying into godbolt, make main `pub`
fn main() {
    println!("in main");
    let args: Vec<String> = std::env::args().collect();
    println!("got args");

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            println!("parsing args");
            let s = run(args[1].parse().unwrap());
            println!("{}", s);
        },
    }
}

