# Dyn Trait Observations

Goal: gain empirical confidence for the potential effectiveness of verifopt's
technique on real world code

## potential grep/regex tool(s)

flow-insensitive
1. grep for all `dyn` + remember which trait name the `dyn` is bound to
2. then grep for all `impl <trait-name> for...` instances to get a CHA count
(total number of possible implementations)

flow-sensitive
- would need to happen in the compiler, i'd think...
- essentially very similar to the structure of the interpreter we are building,
  except no rewriting, just accumulating info

### initial results

from flow-insensitive tool, in the top 50 (most downloaded) crates, we see one of:

Group 1: (23) no `dyn Trait`s

Group 2: (15) `dyn Trait`s where the `Trait`s are _not_ implemented in the current crate
- would probably make more sense to look at full _projects_ rather than 
  individual crates

Group 3: (5) `dyn Trait`s where each `Trait` is only implemented _once_ in the current crate

Group 4: (7) `dyn Trait`s where at least one `Trait` is implemented more than once in
  the current crate
- `rand-0.9.1`: 7 impls of `RngCore`
- `indexmap-2.10.0`: 20 impls of `Iterator`
- `itertools-0.14.0`: 56 impls of `Iterator`
- `log-0.4.27`: 4 impls for `Log`, 4 impls for `VisitSource`, 8 impls for `Source`
- `aho-corasick-1.1.3`: 1 impl for `AcAutomaton`, 8 impls for `PrefilterI`,
  4 impls for `SearcherT`
- `bytes-1.10.1`: 7 impls for `Buf`, 4 impls for `BufMut`
- `regex-automata-0.4.9`: 5 impls for `PrefilterI`, 4 impls for `Strategy`

double check `syn` results (`for` as a trait == wrong)

### individual uses

TODO actually don't want `dyn` return types (unless double counting `dyn` arg
types)?

#### Group 2

valid: 
- [x] serde_derive (1)
    - return type
    - `dyn Iterator`
    - retval is chained w iterator methods (.all(), .filter_map(), etc)
    - `attrs` field is also accessed on the retval
        - i believe this is irrelevant? since not going through vtable ptr, but
          instead going through struct pointer
- [x] hashbrown (5)
    - are these irrelevant/"invalid" for us since in unsafe funcs? TODO
    - 1-2
        - arg type
        - `dyn FnMut`
        - unsafe method
        - struct impl block (`RawTableInner`)
    - 3-5
        - arg type
        - `dyn Fn`
        - unsafe method
        - struct impl block (`RawTableInner`)
- [x] proc-macro2 (1)
    - in function body
    - `type PanicHook = dyn Fn(&PanicInfo) + Sync + Send + 'static;`
    - does not seem to be called, but rather used in pointer comparison?
- [x] digest (3)
    - 1
        - return type
        - `Box<dyn DynDigest>`
        - in `DynDigest` trait decl block
    - 2
        - return type
        - `Box<dyn DynDigest>`
        - in `DynDigest` trait impl block
    - 3
        - trait impl for dyn trait (`impl Clone for Box<dyn DynDigest> {...}`)
- [x] time (13)
    - 1-9
        - return type
        - `Box<dyn Iterator<Item = Self>>`
        - in impl trait for struct
    - 10-13
        - return type
        - `Option<&(dyn std::error::Error + 'static)>`
        - in impl trait for enum
- [x] rand_chacha (1)
    - type cast (`let rng2 = &mut rng1.clone() as &mut dyn CryptoRng;`)
- [x] once_cell (2)
    - 1-2
        - function arg
- [x] thiserror-impl (3 -> 2)
    - 1
        - in an error message (can maybe update tool for this)
    - 2-3
        - return type
        - in `quote!` block
        - in (non-assoc) function
- [x] parking_lot (1)
    - method arg
    - `#[cold]` attribute
    - in struct impl block
- [x] serde_json (6)
    - 1
        - return type
        - impl Trait for Struct
    - 2-3
        - arg type
        - impl Trait for Struct
    - 4
        - struct method arg type
        - struct impl block
    - 5
        - arg type
        - impl Trait for Struct
        - `#[cold]` attribute
    - 6
        - arg type
        - struct impl block
        - `#[cold]` attribute
- [x] base64 (2 -> 1)
    - 1
        - type
        - eventually used as arg to func (from another crate/stdlib)
        - also in examples
    - 2
        - return type
        - in impl Trait for Enum block
- [x] clap (1)
    - func return type

invalid: 
- [x] regex-syntax (1 -> 0)
    - line starts with '#' / is an example, not part of the implementation
- [x] regex (1 -> 0)
    - line starts with '#' / is an example, not part of the implementation
- [x] memchr (3 -> 0)
    - 1-3
        - in `src/tests`

#### Group 3


#### Group 4

### categories

note some intersections between want/don't want filings

valid arg types: 
- Group 2: 5

valid return types: 
- Group 2: 21

#### want

- struct method arg type
    - Group 2 (8 - 7 = 1)
        - hashbrown 1-5 (unsafe)
        - parking_lot (cold)
        - *serde_json 4*, 6 (6=cold)
    - Group 3
    - Group 4

- struct method return types
    - Group 2 (1)
        - *serde_derive*
    - Group 3
    - Group 4

- function arg type
    - Group 2 (2)
        - *once_cell 1-2*
    - Group 3
    - Group 4

- function return type
    - Group 2 (3)
        - *thiserror-impl 2-3*
        - *clap*
    - Group 3
    - Group 4

- trait decl/impl arg types
    - Group 2 (3 - 1 = 2)
        - *serde_json 2-3*, 5 (5=cold)
    - Group 3
    - Group 4

- trait decl/impl return types
    - Group 2 (18 - 1 = 17)
        - *digest 1-2*
        - *time 1-13*
        - serde_json 1 (unsafe)
        - *base64 2*
    - Group 3
    - Group 4

- trait impl for dyn trait decl
    - Group 2 (1)
        - digest 3
    - Group 3
    - Group 4

#### don't want

- in unsafe struct method
    - Group 2 (6)
        - hashbrown 1-5
        - serde_json 1
    - Group 3
    - Group 4

- `#[cold]`
    - Group 2 (3)
        - parking_lot
        - serde_json 5-6
    - Group 3
    - Group 4

- as a type / type cast
    - Group 2 (3)
        - proc-macro2
        - rand_chacha
        - base64 1
    - Group 3
    - Group 4









