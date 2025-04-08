use vstd::prelude::*;

verus! {

/// Original `get_elems` implementation 
///
/// pub fn get_elems(slice: &[i32]) {
///   for i in 0..5 {
///     let elem = slice[i];
///   }
/// }

pub fn get_array_elems(array: &[i32; 5]) {
  for i in 0..5 {
    let elem = array[i];
  }
}

pub fn get_slice_elems(slice: &[i32])
  requires
    slice.len() >= 5,
    forall|n: nat| 0 <= n < slice.len() ==> n < 5,
{
  for i in 0..5 {
    let elem = slice[i];
  }
}

fn main() {
  let array: [i32; 5] = [1, 2, 3, 4, 5];

  get_array_elems(&array);

  get_slice_elems(&array);
}

} // verus!
