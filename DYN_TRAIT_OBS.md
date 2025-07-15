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

from flow-insensitive tool, in the top 50 (most downloaded) crates or so, 
see one of:
1. (23) no `dyn Trait`s
2. (15) `dyn Trait`s where the `Trait`s are _not_ implemented in the current crate
    - would probably make more sense to look at full _projects_ rather than
      individual crates
3. (5) `dyn Trait`s where each `Trait` is only implemented _once_ in the current crate
4. (7) `dyn Trait`s where at least one `Trait` is implemented more than once in
  the current crate

Group 1: 23

Group 2: 15

Group 3: 5

Group 4: 7
- `rand-0.9.1`: 7 impls of `RngCore`
- `indexmap-2.10.0`: 20 impls of `Iterator`
- `itertools-0.14.0`: 56 impls of `Iterator`
- `log-0.4.27`: 4 impls for `Log`, 4 impls for `VisitSource`, 8 impls for `Source`
- `aho-corasick-1.1.3`: 1 impl for `AcAutomaton`, 8 impls for `PrefilterI`,
  4 impls for `SearcherT`
- `bytes-1.10.1`: 7 impls for `Buf`, 4 impls for `BufMut`
- `regex-automata-0.4.9`: 5 impls for `PrefilterI`, 4 impls for `Strategy`

double check `syn` results (`for` as a trait == wrong)

