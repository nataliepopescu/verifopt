# Novelty Claim for Verifopt

CHA = Class Hierarchy Analysis

|     | Verifopt | [KB96](https://dl.acm.org/doi/pdf/10.1145/331119.331419) | [NoVT](https://ieeexplore.ieee.org/document/9581255) | [SmallEiffel](https://inria.hal.science/inria-00565627/document) | [AH96](https://link.springer.com/chapter/10.1007/BFb0053060) |
| --- | --- | --- | --- | --- | --- |
| program analysis | abstract interpretation | CHA | CHA + opts | | hybrid: CHA + type feedback |
| profiling | no | no | no | | yes |
| program modification(s) | switch-conversion | switch-conversion | switch-conversion | | ? |
| tool location | compiler-mod | source-to-source | compiler-mod | | source-to-source |
| mod(s) correctness | verified | ? | tested | | ? |
| multiple/virtual inheritance | yes | | yes | no | ? |
| vtable fallback | no | yes full CH not available (dynlink) | no | | ? |
| dynamic linking | don't believe so | no? | no | | ? |
| eval'd code | | | | | |

uncertain
- static analysis vs profiling
    - profiling can prune types, but may require falling back to virtual
      dispatch (since profiling cannot model every possible execution)
    - static analysis may be more conservative, but abstract interpretation aids
      in pruning types
- clarify what is meant by "multiple/virtual inheritance" (from NoVT)

verifopt technique benefits
- modified compiler enables frontend to communicate with backend
- abstract interpretation can prune impossible types based on control-flow paths
  that are never taken, while base CHA seems to always include every possible class
  type
    - caveat: NoVT has a Dead Class Identifier optimization
    - might be interesting to actually study how much our abstract
      interpretation technique reduces the set of classes/types compared to CHA

verifopt technique limitations
- dynamic linking?
