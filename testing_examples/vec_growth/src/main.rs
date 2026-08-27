use std::hint::black_box;

fn main() {
    // Force multiple reallocations (capacity growth) of a Vec<u8>'s
    // internal buffer - each grow touches RawVec's Unique<u8>/NonNull<u8>
    // pointer field, which is where the original nesting bug showed up.
    let mut buf: Vec<u8> = Vec::new();
    for i in 0..64u8 {
        buf.push(i);
    }

    // Also exercise String, since it's a Vec<u8> underneath with the same
    // RawVec-based growth path, reached via a different set of std types.
    let mut s = String::new();
    for i in 0..64u8 {
        s.push(i as char);
    }

    black_box(buf);
    black_box(s);

    //println!("{} {}", buf.len(), s.len());
}
