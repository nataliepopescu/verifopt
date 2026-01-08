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

	#[flux_rs::trusted]
	#[flux_rs::sig(fn(&mut RVec<T>[@n], i: usize{i < n}) -> &mut T)]
	pub fn get_mut(&mut self, i: usize) -> &mut T {
	    &mut self.inner[i]
	}
}

impl<T> std::ops::Index<usize> for RVec<T> {
    type Output = T;
    #[flux_rs::trusted_impl]
    #[flux_rs::sig(fn(&RVec<T>[@n], i:usize{i < n}) -> &T)]
    fn index(&self, index: usize) -> &T {
        self.get(index)
    }
}

impl<T> std::ops::IndexMut<usize> for RVec<T> {
    #[flux_rs::trusted_impl]
    #[flux_rs::sig(fn(&mut RVec<T>[@n], i:usize{i < n}) -> &mut T)]
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index)
    }
}

/*
pub fn binary_search(vec: &RVec<i32>, x: i32) -> Result<usize, usize> {
    let mut size = vec.len();
    if size <= 1 {
      return Err(0);
    }
    let mut left = 0;
    let mut right = size;
    while left <= right {
        let mid = left + (size / 2);
        let val = vec[mid];
        if val < x {
            left = mid;
        } else if x < val {
            right = mid;
        } else {
            return Ok(mid);
        }
        size = right - left;
    }
    Err(left)
}
*/

