pub trait Shape {
    fn area(&self) -> f32;
}

pub fn area_wrapper(s: &dyn Shape) -> f32 {
    s.area()
}
