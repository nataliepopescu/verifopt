#![feature(ptr_metadata)]

trait Animal {
    fn speak(&self) -> &str;
}

fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {
            name: "kitty",
            age: 9,
            fav_toy: "shoe",
        });
    } else {
        return Box::new(Dog {
            name: "doggo",
            age: 7,
            fav_walk_route: "anywhere",
        });
    }
}

#[inline(always)]
fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {
        name: "kitty",
        age: 9,
        fav_toy: "shoe",
    });
}

#[inline(always)]
fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {
        name: "doggo",
        age: 7,
        fav_walk_route: "anywhere",
    });
}

pub struct Cat<'a>{
    pub name: &'a str,
    pub age: usize,
    pub fav_toy: &'a str,
}

pub struct Dog<'a> {
    pub name: &'a str,
    pub age: usize,
    pub fav_walk_route: &'a str,
}

impl Animal for Cat<'_> {
    fn speak(&self) -> &str{
        "meow"
    }
}

impl Animal for Dog<'_> {
    fn speak(&self) -> &str {
        "woof"
    }
}

pub fn run_best(cat: &Cat) -> String {
    <Cat as Animal>::speak(cat).to_string()
}

pub fn run_best_normalized(num: usize, cat: &Cat) -> String {
    let _animal = get_animal(num);
    let _cat = get_cat();
    <Cat as Animal>::speak(cat).to_string()
}

pub fn run_best_normalized_fallback(num: usize, cat: &Cat) -> String {
    let _animal = get_animal(num);
    let _cat = get_cat();
    let _dog = get_dog();
    <Cat as Animal>::speak(cat).to_string()
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

pub fn run_src_rw(num: usize) -> String {
    let animal = get_animal(num);
    let cat = get_cat();

    let animal_vtable = core::ptr::metadata(&*animal);
    let cat_vtable = core::ptr::metadata(&*cat);

    let raw_animal = Box::into_raw(animal) as *const ();

    if animal_vtable == cat_vtable {
        unsafe {
            let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_animal);
            <Cat as Animal>::speak(cat).to_string()
        }
    } else {
        unsafe {
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog).to_string()
        }
    }
}

pub fn run_src_rw_fallback(num: usize) -> String {
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
            <Cat as Animal>::speak(cat).to_string()
        }
    } else if animal_vtable == dog_vtable {
        unsafe {
            let raw_animal = Box::into_raw(animal) as *const ();
            let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_animal);
            <Dog as Animal>::speak(dog).to_string()
        }
    } else {
        return animal.speak().to_string();
    }
}
