use std::hint::black_box;

trait Thing {
    fn do_thing(&self) -> u32;
}

struct Wrapper<'a> {
    inner: &'a dyn Thing,
}

struct Inner1;
struct Inner2;

impl Thing for Inner1 {
    fn do_thing(&self) -> u32 {
        123
    }
}

impl Thing for Inner2 {
    fn do_thing(&self) -> u32 {
        456
    }
}

fn dyn_call_on_field(wrapper: &Wrapper) -> u32 {
    wrapper.inner.do_thing()
}

fn main() {
    let inner = Inner1 {};
    let _inner = Inner2 {};
    let wrapper = Wrapper { inner: &inner };
    let res = dyn_call_on_field(&wrapper);
    //let res = wrapper.inner.do_thing();
    black_box(res);
}
