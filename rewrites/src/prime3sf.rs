#![feature(ptr_metadata)]

use rand::Rng;
use core::ptr::DynMetadata;

pub trait Animal {
    fn speak(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {
            tmp1: rand::rng().random_range(..100usize),
            tmp2: rand::rng().random_range(..100usize),
            tmp3: rand::rng().random_range(..100usize),
        });
    } else {
        return Box::new(Dog {
            tmp1: rand::rng().random_range(..100usize),
            tmp2: rand::rng().random_range(..100usize),
            tmp3: rand::rng().random_range(..100usize),
        });
    }
}

#[inline(always)]
pub fn get_alligator() -> Box<dyn Animal> {
    return Box::new(Alligator {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_bird() -> Box<dyn Animal> {
    return Box::new(Bird {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_elephant() -> Box<dyn Animal> {
    return Box::new(Elephant {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_frog() -> Box<dyn Animal> {
    return Box::new(Frog {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_giraffe() -> Box<dyn Animal> {
    return Box::new(Giraffe {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_hippo() -> Box<dyn Animal> {
    return Box::new(Hippo {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_iguana() -> Box<dyn Animal> {
    return Box::new(Iguana {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_jaguar() -> Box<dyn Animal> {
    return Box::new(Jaguar {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_kangaroo() -> Box<dyn Animal> {
    return Box::new(Kangaroo {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

#[inline(always)]
pub fn get_lion() -> Box<dyn Animal> {
    return Box::new(Lion {
        tmp1: rand::rng().random_range(..100usize),
        tmp2: rand::rng().random_range(..100usize),
        tmp3: rand::rng().random_range(..100usize),
    });
}

pub struct Alligator {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Bird {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Cat {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Dog {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Elephant {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Frog {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Giraffe {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Hippo {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Iguana {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Jaguar {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Kangaroo {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Lion {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

pub struct Monkey {
    pub tmp1: usize,
    pub tmp2: usize,
    pub tmp3: usize,
}

impl Animal for Alligator {
    fn speak(&self) -> usize {
        11111
    }
}

impl Animal for Bird {
    fn speak(&self) -> usize {
        22222
    }
}

impl Animal for Cat {
    fn speak(&self) -> usize {
        33333
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        44444
    }
}

impl Animal for Elephant {
    fn speak(&self) -> usize {
        55555
    }
}

impl Animal for Frog {
    fn speak(&self) -> usize {
        66666
    }
}

impl Animal for Giraffe {
    fn speak(&self) -> usize {
        77777
    }
}

impl Animal for Hippo {
    fn speak(&self) -> usize {
        88888
    }
}

impl Animal for Iguana {
    fn speak(&self) -> usize {
        99999
    }
}

impl Animal for Jaguar {
    fn speak(&self) -> usize {
        1010101010
    }
}

impl Animal for Kangaroo {
    fn speak(&self) -> usize {
        1111111111
    }
}

impl Animal for Lion {
    fn speak(&self) -> usize {
        1212121212
    }
}

impl Animal for Monkey {
    fn speak(&self) -> usize {
        1313131313
    }
}


pub fn run_best(cat: &Cat) -> usize {
    <Cat as Animal>::speak(cat)
}

pub fn run_not_rw(animal: Box<dyn Animal>) -> usize {
    animal.speak()
}

pub fn run_naive_cha(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    alligator_vtable: DynMetadata<dyn Animal>,
    bird_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
    elephant_vtable: DynMetadata<dyn Animal>,
    frog_vtable: DynMetadata<dyn Animal>,
    giraffe_vtable: DynMetadata<dyn Animal>,
    hippo_vtable: DynMetadata<dyn Animal>,
    iguana_vtable: DynMetadata<dyn Animal>,
    jaguar_vtable: DynMetadata<dyn Animal>,
    kangaroo_vtable: DynMetadata<dyn Animal>,
    lion_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == alligator_vtable {
        unsafe {
            let a: &Alligator = std::mem::transmute::<*const (), &Alligator>(raw_animal);
            <Alligator as Animal>::speak(a)
        }
    } else if animal_vtable == bird_vtable {
        unsafe {
            let a: &Bird = std::mem::transmute::<*const (), &Bird>(raw_animal);
            <Bird as Animal>::speak(a)
        }
    } else if animal_vtable == cat_vtable {
        unsafe {
            let a: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(a)
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let a: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(a)
        }
    } else if animal_vtable == elephant_vtable {
        unsafe {
            let a: &Elephant = std::mem::transmute::<*const (), &Elephant>(raw_animal);
            <Elephant as Animal>::speak(a)
        }
    } else if animal_vtable == frog_vtable {
        unsafe {
            let a: &Frog = std::mem::transmute::<*const (), &Frog>(raw_animal);
            <Frog as Animal>::speak(a)
        }
    } else if animal_vtable == giraffe_vtable {
        unsafe {
            let a: &Giraffe = std::mem::transmute::<*const (), &Giraffe>(raw_animal);
            <Giraffe as Animal>::speak(a)
        }
    } else if animal_vtable == hippo_vtable {
        unsafe {
            let a: &Hippo = std::mem::transmute::<*const (), &Hippo>(raw_animal);
            <Hippo as Animal>::speak(a)
        }
    } else if animal_vtable == iguana_vtable {
        unsafe {
            let a: &Iguana = std::mem::transmute::<*const (), &Iguana>(raw_animal);
            <Iguana as Animal>::speak(a)
        }
    } else if animal_vtable == jaguar_vtable {
        unsafe {
            let a: &Jaguar = std::mem::transmute::<*const (), &Jaguar>(raw_animal);
            <Jaguar as Animal>::speak(a)
        }
    } else if animal_vtable == kangaroo_vtable {
        unsafe {
            let a: &Kangaroo = std::mem::transmute::<*const (), &Kangaroo>(raw_animal);
            <Kangaroo as Animal>::speak(a)
        }
    } else if animal_vtable == lion_vtable {
        unsafe {
            let a: &Lion = std::mem::transmute::<*const (), &Lion>(raw_animal);
            <Lion as Animal>::speak(a)
        }
    } else {
        unsafe {
            let a: &Monkey = std::mem::transmute::<*const (), &Monkey>(raw_animal);
            <Monkey as Animal>::speak(a)
        }
    }
}

pub fn run_src_rw_into_raw(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    }
}

pub fn run_src_rw_transmutes(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    }
}

pub fn run_src_rw_into_raw_fallback(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
) -> usize {
    if animal_vtable == cat_vtable {
        let raw_animal = Box::into_raw(animal) as *const ();
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else if animal_vtable == dog_vtable {
        let raw_animal = Box::into_raw(animal) as *const ();
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    } else {
        animal.speak()
    }
}

pub fn run_src_rw_transmutes_fallback(
    animal: Box<dyn Animal>,
    animal_vtable: DynMetadata<dyn Animal>,
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
) -> usize {
    if animal_vtable == cat_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat)
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let raw_animal: *const () =
                std::mem::transmute::<&Box<dyn Animal>, *const ()>(&animal);
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog)
        }
    } else {
        animal.speak()
    }
}

// if copying into godbolt, make main `pub`
/*
fn main() {
    let args: Vec<String> = std::env::args().collect();
    //let cat: &Cat = &Cat {};

    match args.len() {
        1 => println!("Pass in a number and see what happens!"),
        _ => {
            let animal = get_animal(args[1].parse().unwrap());
            let cat = get_cat();
            let s = run_src_rw_transmutes(animal, cat);
            println!("{}", s);
        },
    }
}
*/
