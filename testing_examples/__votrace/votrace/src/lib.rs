use std::cell::RefCell;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

pub use votrace_macros::trace;

thread_local! {
    static STARTED: RefCell<bool> = RefCell::new(false);
}

pub fn hit(f: &str) {
    let file = env::current_dir().unwrap().join("calls");

    STARTED.with(|s| {
        if *s.borrow() {
            return;
        }

        OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&file)
            .unwrap();
        *s.borrow_mut() = true;
    });

    let out = &mut OpenOptions::new().append(true).open(&file).unwrap();
    writeln!(out, "{}", f).unwrap();
}
