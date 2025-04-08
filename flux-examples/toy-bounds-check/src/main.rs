
/// Original `get_elems` implementation 
///
/// pub fn get_elems(slice: &[i32]) {
///   for i in 0..5 {
///     let elem = slice[i];
///   }
/// }

#[flux_rs::sig(fn(array: &[i32; 5], i: usize{i < 5}))]
pub fn get_array_elems_helper(array: &[i32; 5], i: usize) {
  let _elem = array[i];
}

#[flux_rs::sig(fn(array: &[i32; 5]))]
pub fn get_array_elems(array: &[i32; 5]) {
  for i in 0..5 {
    flux_assume::assume(i < 5);
    get_array_elems_helper(&array, i);
  }
}

#[flux_rs::sig(fn(array: &[i32; 5], i_cap: usize{i_cap <= 5}))]
pub fn get_array_elems_cap(array: &[i32; 5], i_cap: usize) {
  for i in 0..i_cap {
    flux_assume::assume(i < 5);
    get_array_elems_helper(&array, i);
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
  let array: [i32; 5] = [1, 2, 3, 4, 5];

  get_array_elems(&array);
  get_array_elems_cap(&array, 5);

  //inc_elems(&mut array);
}

