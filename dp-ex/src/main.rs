use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Alligator {}
struct Bird {}
struct Cat {}
struct Dog {}
struct Elephant {}
struct Frog {}

impl Animal for Alligator {
    fn speak(&self) {
        println!("chomp");
    }
}

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


//#[unsafe(no_mangle)]
//#[inline(never)]
//pub fn animal_speak(animal: &dyn Animal) {
//    animal.speak();
//}

fn dyn_dp() {
    let a: &dyn Animal;

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        a = &Bird {}
    } else if num == 1 {
        a = &Cat {}
    } else {
        a = &Dog {}
    }

    a.speak();
}

fn static_dp_rand() {
    let a: &dyn Animal;

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        a = &Cat {}
    } else if num == 1 {
        a = &Cat {}
    } else {
        a = &Cat {}
    }

    a.speak();
}

fn static_dp_rand_dummy() {
    let cat;

    let mut rng = rand::rng();
    if rng.random() {}

    cat = &Cat {};

    cat.speak();
}

fn static_dp() {
    let cat = &Cat {};
    cat.speak();
}

fn main() {
    let iter = 100000;
    for _ in 0..iter {
        dyn_dp();
    }
    for _ in 0..iter {
        static_dp_rand();
    }
    for _ in 0..iter {
        static_dp_rand_dummy();
    }
    for _ in 0..iter {
        static_dp();
    }

    //dyn_dp();
    //static_dp();
}
