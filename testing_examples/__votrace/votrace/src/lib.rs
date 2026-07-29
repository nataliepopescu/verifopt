use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Write;

pub use votrace_macros::trace;

thread_local! {
    static STARTED: RefCell<bool> = RefCell::new(false);
}

pub fn hit(f: &str) {
    STARTED.with(|s| {
        if *s.borrow() {
            return;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("rw_calls")
            .unwrap();
        *s.borrow_mut() = true;
    });

    let out = &mut OpenOptions::new().append(true).open("rw_calls").unwrap();
    writeln!(out, "{}", f).unwrap();
}
