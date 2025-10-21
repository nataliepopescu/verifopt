#![feature(ptr_metadata)]

use std::sync::Mutex;

pub trait Animal: Sync + Send {
    fn speak(&self) -> &str;
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> &str {
        "meow"
    }
}

impl Animal for Dog {
    fn speak(&self) -> &str {
        "woof"
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

pub fn mk_vec() -> Mutex<Vec<Box<dyn Animal>>> {
	Mutex::new(vec![])
}

// return last speak() result

pub fn run_best(xs: &[Box<dyn Animal>], cat: &Cat) -> String {
	let mut ret = "".to_string();
    for _ in xs {
		ret = <Cat as Animal>::speak(cat).to_string();
	}
	ret
}

pub fn run_best_normalized(xs: &[Box<dyn Animal>], cat: &Cat) -> String {
	let _cat = get_cat();
	let mut ret = "".to_string();
    for _ in xs {
		ret = <Cat as Animal>::speak(cat).to_string();
	}
	ret
}

pub fn run_not_rw(xs: &[Box<dyn Animal>]) -> String {
	let mut ret = "".to_string();
    for x in xs {
		ret = x.speak().to_string();
	}
	ret
}

pub fn run_not_rw_normalized(xs: &[Box<dyn Animal>]) -> String {
	let _cat = get_cat();
	let mut ret = "".to_string();
    for x in xs {
		ret = x.speak().to_string();
	}
	ret
}

pub fn run_src_rw(xs: &[Box<dyn Animal>]) -> String {
	let cat = get_cat();
	let mut ret = "".to_string();
    for x in xs.iter() {
		let x_vtable = core::ptr::metadata(&**x);
		let cat_vtable = core::ptr::metadata(&*cat);

		if x_vtable == cat_vtable {
			unsafe {
                let raw_x: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
				let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
				ret = <Cat as Animal>::speak(cat).to_string();
			}
		} else {
			unsafe {
                let raw_x: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
				let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
				ret = <Dog as Animal>::speak(dog).to_string();
			}
		}
	}
	ret
}

pub fn run_best_normalized_fallback(xs: &[Box<dyn Animal>], cat: &Cat) -> String {
	let _cat = get_cat();
	let _dog = get_dog();
	let mut ret = "".to_string();
    for _ in xs {
		ret = <Cat as Animal>::speak(cat).to_string();
	}
	ret
}

pub fn run_not_rw_fallback(xs: &[Box<dyn Animal>]) -> String {
	let mut ret = "".to_string();
    for x in xs {
		ret = x.speak().to_string();
	}
	ret
}

pub fn run_not_rw_fallback_normalized(xs: &[Box<dyn Animal>]) -> String {
	let _cat = get_cat();
	let _dog = get_dog();
	let mut ret = "".to_string();
    for x in xs {
		ret = x.speak().to_string();
	}
	ret
}

pub fn run_src_rw_fallback(xs: &[Box<dyn Animal>]) -> String {
	let cat = get_cat();
	let dog = get_dog();
	let mut ret = "".to_string();
    for x in xs.iter() {
		let x_vtable = core::ptr::metadata(&**x);
		let cat_vtable = core::ptr::metadata(&*cat);
		let dog_vtable = core::ptr::metadata(&*dog);

		if x_vtable == cat_vtable {
			unsafe {
                let raw_x: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
				let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
				ret = <Cat as Animal>::speak(cat).to_string();
			}
		} else if x_vtable == dog_vtable {
			unsafe {
                let raw_x: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x);
				let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
				ret = <Dog as Animal>::speak(dog).to_string();
			}
		} else {
            ret = x.speak().to_string();
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
