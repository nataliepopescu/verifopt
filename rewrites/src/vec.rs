#![feature(ptr_metadata)]

use std::sync::Mutex;

#[unsafe(no_mangle)]
pub static my_vec: Mutex<Vec<Box<dyn Animal>>> = Mutex::new(vec![]);

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
    for x_ref in xs.iter() {
		let x_vtable = core::ptr::metadata(&**x_ref);
		let cat_vtable = core::ptr::metadata(&*cat);

        unsafe {
            let raw_x: *const () = std::mem::transmute::<&Box<dyn Animal>, *const ()>(&*x_ref);

		    if x_vtable == cat_vtable {
		    	unsafe {
		    		let cat: &Cat = std::mem::transmute::<*const (), &Cat>(raw_x);
		    		ret = <Cat as Animal>::speak(cat).to_string();
		    	}
		    } else {
		    	unsafe {
		    		let dog: &Dog = std::mem::transmute::<*const (), &Dog>(raw_x);
		    		ret = <Dog as Animal>::speak(dog).to_string();
		    	}
		    }
        }
	}
	ret
}

/*
pub fn main() {
    my_vec.lock().unwrap().insert(0, Box::new(Cat));
    run(&my_vec.lock().unwrap());
}
*/
