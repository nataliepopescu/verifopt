#[flux_rs::opaque]
#[flux_rs::refined_by(len: int)]
#[flux_rs::invariant(0 <= len)]
pub struct RVec<T> {
  inner: Vec<T>,
}

impl<T> RVec<T> {
    #[flux_rs::trusted]
    #[flux_rs::sig(fn() -> RVec<T>[0])]
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

	#[flux_rs::trusted]
	#[flux_rs::sig(fn(self: &strg RVec<T>[@n], T)
	            ensures self: RVec<T>[n+1])]
	pub fn push(&mut self, item: T) {
	    self.inner.push(item);
	}
	
	#[flux_rs::trusted]
	#[flux_rs::sig(fn(self: &strg {RVec<T>[@n] | 0 < n}) -> T
	            ensures self: RVec<T>[n-1])]
	pub fn pop(&mut self) -> T {
	  self.inner.pop().unwrap()
	}
	
	#[flux_rs::trusted]
	#[flux_rs::sig(fn(&RVec<T>[@n]) -> usize[n])]
	pub fn len(&self) -> usize {
	    self.inner.len()
	}

	#[flux_rs::trusted]
	#[flux_rs::sig(fn(&RVec<T>[@n], i: usize{i < n}) -> &T)]
	pub fn get(&self, i: usize) -> &T {
	    &self.inner[i]
	}
}

/// Original `get_elems` implementation 
///
/// pub fn get_elems(slice: &[i32]) {
///   for i in 0..5 {
///     let elem = slice[i];
///   }
/// }

#[flux_rs::sig(fn(b: bool[true]))]
pub fn assert(_: bool) {}

//#[flux_rs::sig(fn(array: &[i32; 5], i: usize{i < 5}))]
//pub fn get_array_elems_helper(array: &[i32; 5], i: usize) {
//  let _elem = array[i];
//}

//#[flux_rs::sig(fn(array: &[i32; 5]))]
//pub fn get_array_elems(array: &[i32; 5]) {
//  //assert(array.len() == 5);
//  let mut i = 0;
//  while i < array.len() {
//    //assert(i < 5);
//    let _elem = array.get(i);
//    i += 1;
//  }
//}

//#[flux_rs::sig(fn(array: &[i32; 5], i_cap: usize{i_cap <= 5}))]
//pub fn get_array_elems_cap(array: &[i32; 5], i_cap: usize) {
//  for i in 0..i_cap {
//    flux_assume::assume(i < 5);
//    get_array_elems_helper(&array, i);
//  }
//}

#[flux_rs::sig(fn(&RVec<T>[@n]))]
pub fn get_elems<T>(rvec: &RVec<T>) {
  let mut i: usize = 0;
  while i < rvec.len() {
    let _elem = rvec.get(i);
    i += 1;
  }
}

//// triggers ICE
//#[flux_rs::sig(fn())]
//pub fn inc_elems(array: &mut [i32]) {
//  for i in 0..5 {
//    array[i] += 1;
//  }
//}

fn main() {
  //let array: [i32; 5] = [1, 2, 3, 4, 5];
  //get_array_elems(&array);
  //get_array_elems_cap(&array, 5);

  let mut rvec: RVec<i32> = RVec::new();
  rvec.push(1);
  rvec.push(2);
  rvec.push(3);
  get_elems(&rvec);
}

