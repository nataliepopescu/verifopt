pub trait Animal {
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

pub fn run_best(num: usize, cat: &Cat) -> String {
    let _animal = get_animal(num);
    let _cat = get_cat();
    <Cat as Animal>::speak(cat).to_string()
}

pub fn run_not_rw(num: usize) -> String { //, animal: &dyn Animal) -> String {
    let animal = get_animal(num);
    let _cat = get_cat();
    animal.speak().to_string()
}

pub fn run_src_rw(num: usize) -> String { //, animal: &dyn Animal) -> String {
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
