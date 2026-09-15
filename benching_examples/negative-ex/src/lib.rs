pub trait Animal {
    fn speak(&self) -> usize;
}

#[inline(never)]
pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        Box::new(Cat {})
    } else {
        Box::new(Dog {})
    }
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }
}

#[inline(never)]
pub fn wrap_dyn_call1(a: &dyn Animal) -> usize {
    a.speak()
}

#[inline(never)]
pub fn wrap_dyn_call2(a: &dyn Animal) -> usize {
    a.speak()
}

pub fn inner_main() {
    let x = 0;
    let animal = get_animal(x);
    std::hint::black_box(wrap_dyn_call2(&*animal));
}

