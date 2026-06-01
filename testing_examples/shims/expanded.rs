#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use std::cell::Cell;
fn add_one(x: i32) -> i32 {
    x + 1
}
pub const FOO: ::std::thread::LocalKey<Cell<fn(i32) -> i32>> = {
    const __RUST_STD_INTERNAL_INIT: Cell<fn(i32) -> i32> = { Cell::new(add_one) };
    unsafe {
        ::std::thread::LocalKey::new(const {
            if ::std::mem::needs_drop::<Cell<fn(i32) -> i32>>() {
                |_| {
                    #[thread_local]
                    static __RUST_STD_INTERNAL_VAL: ::std::thread::local_impl::EagerStorage<
                        Cell<fn(i32) -> i32>,
                    > = ::std::thread::local_impl::EagerStorage::new(
                        __RUST_STD_INTERNAL_INIT,
                    );
                    __RUST_STD_INTERNAL_VAL.get()
                }
            } else {
                |_| {
                    #[thread_local]
                    static __RUST_STD_INTERNAL_VAL: Cell<fn(i32) -> i32> = __RUST_STD_INTERNAL_INIT;
                    &__RUST_STD_INTERNAL_VAL
                }
            }
        })
    }
};
fn main() {
    let _res = FOO.get()(2);
}
