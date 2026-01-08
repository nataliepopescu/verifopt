use rvec::RVec;

/// Original `get_elems` implementation 
///
/// pub fn get_elems(slice: &[i32]) {
///   for i in 0..5 {
///     let elem = slice[i];
///   }
/// }

//#[flux_rs::sig(fn(array: &[i32; 5], i: usize{i < 5}))]
//pub fn get_array_elems_helper(array: &[i32; 5], i: usize) {
//  let _elem = array[i];
//}

//#[flux_rs::sig(fn(array: &[i32; 5]))]
//pub fn get_array_elems(array: &[i32; 5]) {
//  let mut i = 0;
//  while i < array.len() {
//    let _elem = array.get(i);
//    i += 1;
//  }
//}

//#[flux_rs::sig(fn(array: &[i32; 5], i_cap: usize{i_cap <= 5}))]
//pub fn get_array_elems_cap(array: &[i32; 5], i_cap: usize) {
//  for i in 0..i_cap {
//    get_array_elems_helper(&array, i);
//  }
//}

#[flux_rs::sig(fn(&RVec<T>[@n]))]
pub fn get_elems<T>(vec: &RVec<T>) {
  let mut i: usize = 0;
  while i < vec.len() {
    let _elem = vec.get(i);
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

  let mut vec: RVec<i32> = RVec::new();
  vec.push(1);
  vec.push(2);
  vec.push(3);
  get_elems(&vec);
}

