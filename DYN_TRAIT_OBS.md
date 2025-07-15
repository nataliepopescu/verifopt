# Dyn Trait Observations

Goal: gain empirical confidence for the potential effectiveness of verifopt's
technique on real world code

## grep `dyn` in top 250 most-downloaded crates on crates.io

```sh
$ grep -rn 'dyn ' .
$ grep -rn 'dyn ' ./src
$ grep -rn '[/][/].*dyn ' ./src | wc -l # num dyns in comments/docs
```

2835 usages total (including tests, benchmarks, comments, etc)

lets look at one crate at a time

```sh
wget https://crates.op/api/v1/crates/<name>/<version>/download
tar -C <dirname> -xf download
rm -f download
```

table that counts the numbers of `dyn`s per crate:

| crate | total | in `src/` | in uncommented code |
| --- | --- | --- | --- |
| syn | 18 | 10 | 4 |
| hashnrown | 9 | 9 | 5 |

### [syn-2.0.104](https://crates.io/crates/syn)

#### `src/punctuated.rs` (4)

### [hashbrown-0.15.4](https://crates.io/crates/hashbrown)

#### `src/raw/mod.rs` (5)


## potential grep/regex tool(s)

flow-insensitive
1. grep for all `dyn` + remember which trait name the `dyn` is bound to
2. then grep for all `impl <trait-name> for...` instances to get a CHA count
(total number of possible implementations)

flow-sensitive
- would need to happen in the compiler, i'd think...
- essentially very similar to the structure of the interpreter we are building,
  except no rewriting, just accumulating info




# Old/Mistaken

## grep `dyn` in top 250 most-downloaded crates on crates.io

using spider from
[bencher_scrape](https://github.com/nataliepopescu/bencher_scrape):

```sh
scrapy crawl -a category=top -a x=1 get-crates
```

<name>-<version> (<num `dyn` instances>)

15 crates: 
- prost-derive-0.14.1       (1)
- digest-0.11.0-rc.0        (5)
- tracing-subscriber-0.3.19 (1)
- async-trait-0.1.88        (1)
- env_logger-0.11.8         (1)
- clap_builder-4.5.41       (1)
- log-0.4.27                (1)
- autocfg-1.5.0             (3)
- tokio-macros-2.5.0        (1)
- syn-2.0.104               (2)
- anyhow-1.0.98             (1)
- tracing-core-0.1.34       (4)
- crossbeam-channel-0.5.15  (1)
- rustc-demangle-0.1.25     (1)
- thiserror-impl-2.0.12     (2)

jk, somehow my grep didn't catch a lot of `dyn`s (`grep 'dyn '` misses many more 
than `grep 'dyn'` (without the trailing space), but also many valid ones??)
- `grep -rn 'dyn ' .` instead
- the `-w` is problematic (don't fully understand)

### [prost-derive-0.14.1](https://crates.io/crates/prost-derive)

crate description:
- encoding/decoding for Rust types annotated with `prost`
- [prost](https://crates.io/crates/prost) = protocol buffers impl for Rust

short example of usage:
[snazzy](https://github.com/danburkert/snazzy/blob/master/src/lib.rs)
- proto-derive is not directly interacted with, but this example simply shows 
  where its functionality comes into play (`encode`/`decode` calls)

dependencies with `dyn` (at least in top 250): 
- anyhow-1.0.1
- syn-2

#### `dyn` location: `src/field/map.rs:326`

pattern:
`struct DebugWrapperName<'a>(&'a dyn Debug);`
- what does this mean? what implements the `Debug` trait? ah yes, typically
  derived (`#[derive(Debug)]`)
- so, `DebugWrapperName` is a wrapper around any (?) type that implements
  `Debug`, to improve debugging output

in `debug()` of `impl Field`
- wraps "map" for nicer Debug

within a `quote!` block
- [quote](https://crates.io/crates/quote)
    - turns rust syntax tree data structures (ASTs) into tokens of source code
    - `quote!` == procedural macro (token stream -> code/manipulation -> token stream)

considerations
- `dyn` is within a proc macro block: when are proc macros evaluated? (would make
  sense for this to be very early, so not likely a concern, but good to confirm)
    - would actually be crazy for them to not be evaluated/resolved/expanded
      before type checking, so I think this is a safe assumption
    - however, this does further motivate at least a Rust front-end tool
    - confirmed, macro expansion happens around the lexing/parsing compiler steps
- this crate's `dyn` is within a debugging function
    - could still be interesting in terms of performance overhead, but less
      motivating in terms of genuine use cases (i.e. do we really care about
      optimizing debugging in this way? unlikely)

