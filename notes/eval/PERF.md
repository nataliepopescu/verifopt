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

## inlining

with various inlining attributes around `wrap_dyn_call`

### `two_variants_bench` (single dynamic dispatch)

using alt 0s and 1s input type

| inlining attribute | OG (ns) | RW (ns) |
| --- | --- | --- |
| never  | 2.4  | 1.78 |
| always | 2.04 | 1.69 |
| none   | 2.00 | 1.68 |
