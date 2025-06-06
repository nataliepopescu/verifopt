# Novelty Claim for Verifopt

CHA = Class Hierarchy Analysis
- traversing the inheritance tree -> (at least partially) resolve type ambiguity
- inheritance vs interface implementation
    - C++ has both, while Rust just has interface implementation (traits)
    - todo: does this make resolving type ambiguity easier in Rust?
    - [Rust Is Beyond Object-Oriented, Part 3: Inheritance](https://www.thecodedmessage.com/posts/oop-3-inheritance/)

|     | Verifopt | [KB96](https://dl.acm.org/doi/pdf/10.1145/331119.331419) | [NoVT](https://ieeexplore.ieee.org/document/9581255) | [SmallEiffel](https://inria.hal.science/inria-00565627/document) | [AH96](https://link.springer.com/chapter/10.1007/BFb0053060) |
| --- | --- | --- | --- | --- | --- |
| analysis type | abstract interpretation | CHA | CHA + opts | | hybrid: CHA + type feedback |
| profiling | no | no | no | | yes |
| program mod(s) | switch-conversion | switch-conversion | switch-conversion | | |
| tool location | in-compiler | source-to-source | in-compiler | | source-to-source |
| mod(s) correctness | verified | | tested | | |
| multiple/virtual inheritance | yes | yes | yes | no | yes |
| vtable fallback | no | yes if full CH not available (dynlink) | no | | |
| dynamic linking | don't believe so | no? | no | | |
| eval'd code | | | | | |

uncertain
- static analysis vs profiling
    - profiling can prune types, but may require falling back to virtual
      dispatch (since profiling cannot model every possible execution)
    - static analysis may be more conservative, but abstract interpretation aids
      in pruning types
- clarify what is meant by "multiple/virtual inheritance" (from NoVT)
- look more into CHA + its drawbacks

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
