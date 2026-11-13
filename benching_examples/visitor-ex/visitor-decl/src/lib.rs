pub trait AnimalVisitor {
    fn receive_dog(&self, a: &dyn Animal) -> usize;
    fn receive_cat(&self, a: &dyn Animal) -> usize;
    /// Like receive_*, with per-call work that depends on `x`, for the
    /// timing harness (visitor-use/src/bench.rs): constants like speak()'s
    /// let a devirtualized loop fold away entirely.
    fn work_dog(&self, a: &dyn Animal, x: usize) -> usize;
    fn work_cat(&self, a: &dyn Animal, x: usize) -> usize;
}

pub trait Animal {
    fn speak(&self) -> usize;
    fn visit(&self, av: &dyn AnimalVisitor) -> usize;
    /// Per-call work for the timing harness (see AnimalVisitor::work_dog).
    fn work(&self, x: usize) -> usize;
    fn visit_work(&self, av: &dyn AnimalVisitor, x: usize) -> usize;
}

pub struct Cat;
pub struct Dog;

impl Animal for Cat {
    fn speak(&self) -> usize {
        11111
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_cat(self)
    }

    fn work(&self, x: usize) -> usize {
        x.wrapping_mul(3) ^ 0x5a
    }

    fn visit_work(&self, av: &dyn AnimalVisitor, x: usize) -> usize {
        av.work_cat(self, x)
    }
}

impl Animal for Dog {
    fn speak(&self) -> usize {
        22222
    }

    fn visit(&self, av: &dyn AnimalVisitor) -> usize {
        av.receive_dog(self)
    }

    fn work(&self, x: usize) -> usize {
        x.rotate_left(5).wrapping_add(7)
    }

    fn visit_work(&self, av: &dyn AnimalVisitor, x: usize) -> usize {
        av.work_dog(self, x)
    }
}
