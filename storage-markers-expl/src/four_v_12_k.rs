// Example from LLVM project: https://github.com/llvm/llvm-project/blob/main/llvm/lib/CodeGen/StackColoring.cpp
// Naively-compiled, this program would use 12k of stack space. However, the
// stack slot corresponding to `z` is always destroyed before either of the
// stack slots for `x` or `y` are used, and then `x` is only used if `var`
// is true, while `y` is only used if `var` is false. So in no time are 2
// of the stack slots used together, and therefore we can merge them,
// compiling the function using only a single 4k alloca

//     void bar(char *, int);
//     void foo(bool var) {
//         A: {
//             char z[4096];
//             bar(z, 0);
//         }
//
//         char *p;
//         char x[4096];
//         char y[4096];
//         if (var) {
//             p = x;
//         } else {
//             bar(y, 1);
//             p = y + 1024;
//         }
//        B:
//            bar(p, 2);
//     }
//

/************************************************/
/*************** Converted to Rust **************/
/************************************************/
// Fully safe Rust version, functionally as close as possible.
//
// Safe replacement for `bar(char*, int)`.
// We take a mutable slice so the caller can pass either a full array
// or a subslice (like y + 1024).
fn bar(buf: &mut [u8], tag: i32) {
    // Do something observable-ish so the compiler can't trivially delete it.
    // This simulates "bar reads/writes through the pointer".
    if !buf.is_empty() {
        buf[0] = buf[0].wrapping_add(tag as u8);
    }
}

enum PChoice {
    X,
    YOffset1024,
}



pub fn foo(var: bool) {
    {
        let mut z = [0u8; 4096];
        bar(&mut z[..], 0);
    }

    let mut x = [0u8; 4096];
    let mut y = [0u8; 4096];

    // In C: `p` is a pointer to either `x` or `y + 1024`.
    // In safe Rust: model `p` as a mutable slice that aliases exactly one of those.
    //
    // Note: to keep this safe (and avoid "borrow x and y both mutably then pick one"),
    // we branch and call bar inside each branch.
    let choice = if var {
        PChoice::X
    } else {
        bar(&mut y[..], 1);
        PChoice::YOffset1024
    };

    match choice {
        PChoice::X => {
            let p = &mut x[..];
            bar(p, 2);
        }
        PChoice::YOffset1024 => {
            let p = &mut y[1024..];
            bar(p, 2);
        }
    }
}

