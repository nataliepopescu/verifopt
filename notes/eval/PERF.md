# some performance number sketches

## branch predictor

kinda trying to play w the branch predictor on different patterns of input (that
determine which branch the `two_variants` example goes down)

### `two_variants_bench` (single dynamic dispatch)

using `#[inline(never)]` around `wrap_dyn_call`

| input type | OG (ns) | RW (ns) |
| --- | --- | --- |
| all 0s        | 1.95 | 1.78 |
| alt 0s and 1s | 2.4  | 1.78 |
| rand          | 4.78 | 1.76 |

the rand OG performance is weird, given anja's research it should perform
between the `all` and `alt` types.... TODO look into
- i guess it depends on the branch predictor heuristic, i.e. it is not
  necessarily of  doing the naive thing which is: predict whatever the last
  result is
- so rand could trigger another heuristic that makes it perform even worse than
  the naive?

### `visitor_one_variant_bench` (two dynamic dispatch, one w two variants, one w one variant)

no inlining annotations

| input type | OG (ns) | RW (ns) |
| --- | --- | --- |
| all 0s (all)        | 3.47 | 3.20 |
| alt 0s and 1s (alt) | 3.53 | 3.09 |
| rand                | 5.95 | 3.20 |
| 00,01,10,11 (vis1)  | 3.50 | 3.05 |
| 00,10,01,11 (vis2)  | 3.52 | 3.18 |

verifopt seems to make code much less susceptible to branch mispredictions

note that this is _not_ optimized to the performance of a single static call,
but rather roughly that of two static calls, since that is the best-case
scenario here. 

FIXME re-run actually, these numbers are slightly too small?







### `visitor_two_variants_bench` (two dynamic dispatch, both w two variants)

no inlining annotations

| input type | OG (ns) | RW (ns) |
| --- | --- | --- |
| all 0s (all)        | 3.44 | 3.43 |
| alt 0s and 1s (alt) | 3.43 | 3.41 |
| rand                | 9.26 | 3.43 |
| 00,01,10,11 (vis1)  | 3.69 | 3.44 |
| 00,10,01,11 (vis2)  | 3.76 | 3.42 |

## inlining

with various inlining attributes around `wrap_dyn_call` (or other wrapper
functions that contain dynamic dispatch, e.g. `visit`)

### `two_variants_bench` (single dynamic dispatch)

using alt 0s and 1s input type

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 2.4  | 1.78 |
| always | 2.04 | 1.69 |
| none   | 2.00 | 1.68 |

### `visitor_one_variant_bench` (two dynamic dispatch, one w two variants, one w one variant)

#### alt input

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 3.72 | 3.76 |
| always | 3.53 | 2.92 | ?
| none   | 3.53 | 3.09 |

#### rand input

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 6.01 | 6.13 |
| always | 6.09 | 2.96 | ?
| none   | 5.95 | 3.20 |

it is actually pretty interesting to see here that if we disable inlining for
the rewritten functions, verifopt's performance benefits kind of go away, which
suggests that the verifopt-enabled inlining does count for a lot of the
performance improvement (this is easier to observe with random input, but can
also be seen in the above alt input case)

### `visitor_two_variants_bench` (two dynamic dispatch, both w two variants)

inlining is currently difficult to measure in a reliable way b/c inlining
changes the indices of the vtable ptr locations, so things need to be manually
adjusted (unlike non-inlined functions which always have the necessary vtable
ptrs at a fixed offset)

automating the vtable ptr shim in the future will fix this

#### vis1 input

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 4.51 | 3.66 |
| always | 3.71 | 3.33 |
| none   | 3.69 | 3.44 |

#### vis2 input

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 4.63 | 3.67 |
| always | 3.71 | 3.43 |
| none   | 3.76 | 3.42 |

#### rand input

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 9.56 | 6.07 |
| always | 9.15 | 9.02 | ?
| none   | 9.26 | 3.43 |



