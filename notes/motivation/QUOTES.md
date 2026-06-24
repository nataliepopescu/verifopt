# Quotes

"
/// This uses dynamic dispatch to reduce the amount of code generated, but it is
/// eliminated by LLVM optimizations."
- https://docs.rs/hashbrown/0.16.1/src/hashbrown/raw/mod.rs.html#2075

"
/// A trait that effectively gives us practical dynamic dispatch over anything
/// that impls `Automaton`, but without needing to add a bunch of bounds to
/// the core `Automaton` trait. Basically, we provide all of the marker traits
/// that our automatons have, in addition to `Debug` impls and requiring that
/// there is no borrowed data. Without these, the main `AhoCorasick` type would
/// not be able to meaningfully impl `Debug` or the marker traits without also
/// requiring that all impls of `Automaton` do so, which would be not great.
"
- aho-corasick-1.1.3/src/ahocorasick.rs: 2636
