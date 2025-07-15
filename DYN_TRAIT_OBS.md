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

### initial results

from flow-insensitive tool, in the top 50 (most downloaded) crates or so, 
see one of:
- no `dyn Trait`s
- `dyn Trait`s where the `Trait` is not implemented in the current crate
    - would probably make more sense to look at full _projects_ rather than
      individual crates
- `dyn Trait`s where each `Trait` is only implemented once in the current crate

exceptions (TODO confirm manually)
- `rand-0.9.1`: 7 impls of `RngCore`
- `indexmap-2.10.0`: 20 impls of `Iterator`
- `itertools-0.14.0`: ~40-50 impls of `Iterator`
- `log-0.4.27`: 4 impls for `Log`, 4 impls for `VisitSource`, 8 impls for `Source`
- `aho-corasick-1.1.3`: 1 impl for `AcAutomaton`, 8 impls for `PrefilterI`,
  4 impls for `SearcherT`
- `bytes-1.10.1`: 7 impls for `Buf`, 4 impls for `BufMut`

TODO `impl Trait for Generic` pattern?

tool failed on `regex-automata`, `unicode-ident` (fix)

double check `syn` results (`for` as a trait == wrong)



