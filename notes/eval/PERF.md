# some initial performance numbers

## branch predictor

kinda trying to play w the branch predictor on different patterns of input (that
determine which branch the `two_variants` example goes down)

### `two_variants_bench` (single dynamic dispatch)

| input type | OG (ns) | RW (ns) |
| --- | --- | --- |
| all 0s | 1.95 | 1.78 |
| alt 0s and 1s | 2.4 | 1.78 |
| rand | 4.78 | 1.76 |

the rand OG performance is weird, given anja's research it should perform
between the `all` and `alt` types

