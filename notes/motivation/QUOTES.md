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

/// A type that encapsulates the selection of a prefilter algorithm from a 
/// sequence of needles.
/// 
/// The existence of this type is a little tricky, because we don't (currently)
/// use it for performing a search. Instead, we really only consume it by 
/// converting the underlying prefilter into a trait object, whether that be 
/// `dyn PrefilterI` or `dyn Strategy` (for the meta regex engine). In order 
/// to avoid re-copying the prefilter selection logic, we isolate it here, and 
/// then force anything downstream that wants to convert it to a trait object 
/// to do trivial case analysis on it.
/// 
/// One wonders whether we *should* use an enum instead of a trait object.
/// At time of writing, I chose trait objects based on instinct because 1) I 
/// knew I wasn't going to inline anything and 2) there would potentially be 
/// many different choices. However, as of time of writing, I haven't actually
/// compared the trait object approach to the enum approach. That probably
/// should be litigated, but I ran out of steam.
/// 
/// Note that if the `alloc` feature is disabled, then values of this type 
/// are (and should) never be constructed. Also, in practice, for any of the 
/// prefilters to be selected, you'll need at least one of the `perf-literal-*`
/// features enabled.
- regex-automata-0.4.13/src/util/prefilter/mod.rs: 523
