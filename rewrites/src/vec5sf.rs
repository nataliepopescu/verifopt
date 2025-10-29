use core::ptr::DynMetadata;

pub trait Animal: Sync + Send {
    fn speak(&self) -> usize;
}

pub fn get_animal(num: usize) -> Box<dyn Animal> {
    if num == 0 {
        return Box::new(Cat {
            age: 9,
            num_siblings: 11,
        });
    } else {
        return Box::new(Dog {
            age: 7,
            num_siblings: 3,
        });
    }
}

#[inline(always)]
pub fn get_cat() -> Box<dyn Animal> {
    return Box::new(Cat {
        age: 8,
        num_siblings: 10,
    });
}

#[inline(always)]
pub fn get_dog() -> Box<dyn Animal> {
    return Box::new(Dog {
        age: 4,
        num_siblings: 2,
    });
}

pub struct Cat {
    pub age: usize,
    pub num_siblings: usize,
}

pub struct Dog {
    pub age: usize,
    pub num_siblings: usize,
}

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

pub fn run_best(xs: &[(Box<dyn Animal>, DynMetadata<dyn Animal>)], cat: &Cat) -> usize {
    let mut ret = 0;
    for _ in xs {
        ret = <Cat as Animal>::speak(cat);
    }
    ret
}

pub fn run_not_rw(xs: &[(Box<dyn Animal>, DynMetadata<dyn Animal>)]) -> usize {
    let mut ret = 0;
    for (x, _) in xs {
        ret = x.speak();
    }
    ret
}

pub fn run_src_rw_into_raw(
    xs: &[(Box<dyn Animal>, DynMetadata<dyn Animal>)],
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let mut ret = 0;
    for (x, x_vtable) in xs.iter() {
        let raw_x = Box::into_raw(Box::new(x)) as *const ();
        if *x_vtable == cat_vtable {
            unsafe {
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
                ret = <Cat as Animal>::speak(cat);
            }
        } else {
            unsafe {
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
                ret = <Dog as Animal>::speak(dog);
            }
        }
    }
    ret
}

pub fn run_src_rw_transmutes(
    xs: &[(Box<dyn Animal>, DynMetadata<dyn Animal>)],
    cat_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let mut ret = 0;
    for (x, x_vtable) in xs.iter() {
        if *x_vtable == cat_vtable {
            unsafe {
                let raw_x: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
                ret = <Cat as Animal>::speak(cat);
            }
        } else {
            unsafe {
                let raw_x: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
                ret = <Dog as Animal>::speak(dog);
            }
        }
    }
    ret
}

pub fn run_src_rw_transmutes_fallback(
    xs: &[(Box<dyn Animal>, DynMetadata<dyn Animal>)],
    cat_vtable: DynMetadata<dyn Animal>,
    dog_vtable: DynMetadata<dyn Animal>,
) -> usize {
    let mut ret = 0;
    for (x, x_vtable) in xs.iter() {
        if *x_vtable == cat_vtable {
            unsafe {
                let raw_x: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
                let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
                ret = <Cat as Animal>::speak(cat);
            }
        } else if *x_vtable == dog_vtable {
            unsafe {
                let raw_x: *const () =
                    std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
                let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
                ret = <Dog as Animal>::speak(dog);
            }
        } else {
            ret = x.speak();
        }
    }
    ret
}

/*
#[unsafe(no_mangle)]
pub static my_vec: Mutex<Vec<Box<dyn Animal>>> = Mutex::new(vec![]);

pub fn main() {
    my_vec.lock().unwrap().insert(0, Box::new(Cat));
    run(&my_vec.lock().unwrap());
}
*/
