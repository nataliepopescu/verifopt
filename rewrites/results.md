| Dynamic Dispatch Calls                                                                    | # Dynamic Dispatch Calls |
| ----------------------------------------------------------------------------------------- | ------------------------ |
| `run_animal_dispatch(animal: Box<dyn Animal>, v: Box<Visitor1>)`                          | 2 dynamic calls          |
| `run_visitor_dispatch(animal: &Dog, v: Box<dyn AnimalVisitor>)`                           | 1 dynamic calls          |
| `visitor0sf_2::run_not_rw(animal: Box<dyn Animal>, dc: &Visitor1)`                        | 2 dynamic calls          |
| `double_visitor0sf::run_full_not_rw(animal: Box<dyn Animal>, dc: Box<dyn AnimalVisitor>)` | 2 dynamic calls          |



| Branch Misprediction Effects                                                              | random                                                                                                           | always 1                                   | always 2                | alternating                                                                                       |
| ----------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ----------------------- | ------------------------------------------------------------------------------------------------- |
| `double_visitor0sf::run_animal_dispatch(animal: Box<dyn Animal>, v: Box<Visitor1>)`       | **High**: animal indirect target flips (Cat/Dog). Visitor call inside `visit`is stable (Visitor1).               | **Low**: animal target stable (Cat).       |                         | **High → Very High**: alternating Cat/Dog often worse than random for indirect-target predictors. |
| `double_visitor0sf::run_visitor_dispatch(animal: &Dog, v: Box<dyn AnimalVisitor>)`        | **Moderate/High**: visitor call target flips (Visitor1/Visitor2).                                                | **Low**: visitor target stable (Visitor1). |                         | **Moderate/High**: alternating Visitor1/2 stresses indirect prediction (often worse than random). |
| `double_visitor0sf::run_full_not_rw(animal: Box<dyn Animal>, dc: Box<dyn AnimalVisitor>)` | **Very High**: Animal::visit flips (Cat/Dog) and visitor method flips (V1/V2) → more target entropy than others. | **Moderate**_(only one pinned)_.           | **Low** _(both pinned)_ | **Very High**: alternation in one or both dimensions.                                             |





| Metric            | **fknown** | **frandom** | Δ (random / known) |
| ----------------- | ---------- | ----------- | ------------------ |
| Cycles (u)        | 9,345,746  | 26,064,635  | **2.79×**          |
| Instructions (u)  | 25,461,544 | 25,467,203  | ~1.00×             |
| Branches (u)      | 4,968,527  | 4,969,839   | ~1.00×             |
| Branch Misses (u) | 19,197     | 617,651     | **32.2×**          |
| Branch Miss Rate  | 0.39%      | 12.43%      | —                  |
| MPKI              | 0.75       | 24.25       | **32.3×**          |
| CPI               | 0.37       | 1.02        | **2.8×**           |
| Elapsed Time (s)  | 0.00864    | 0.01290     | **1.49×**          |