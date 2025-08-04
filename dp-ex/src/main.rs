use rand::Rng;

pub trait Animal {
    fn speak(&self);
}

struct Alligator {
    num: u32,
}

struct Bird {
    num: u32,
}

struct Cat {
    num: u32,
}

struct Dog {
    num: u32,
}

struct Elephant {
    num: u32,
}

struct Frog {
    num: u32,
}

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

    let bird = Bird { num: 0 };
    let cat = Cat { num: 1 };
    let dog = Dog { num: 2 };

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        a = &bird;
    } else if num == 1 {
        a = &cat;
    } else {
        a = &dog;
    }

    a.speak();
}

fn static_dp_rand() {
    let a: &dyn Animal;

    let cat = Cat { num: 1 };

    let num: u32 = rand::rng().random_range(..3);

    if num == 0 {
        a = &cat;
    } else if num == 1 {
        a = &cat;
    } else {
        a = &cat;
    }

    a.speak();
}

fn static_dp_rand_dummy() {
    let cat;

    let mut rng = rand::rng();
    if rng.random() {}

    cat = &Cat { num: 1 };

    cat.speak();
}

fn static_dp() {
    let cat = &Cat { num: 1 };
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
