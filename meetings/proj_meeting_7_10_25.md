# meeting

## agenda

- [ ] high-level
    - current goal: eliminate indirect function calls via flow-sensitive
      analysis

- [ ] what is implemented?
    - general
        - simplified program CFG
        - types: `Int`, `Struct`, `Func`, `DynTrait`, `Idk`
        - values: `Num`, `IdkNum`, `Struct`, `IdkStruct`, `Var`, `IdkVar`

    - indirect calls
        - CHA
        - flow-sensitive analysis
        - func rewriting

    - trait calls
        - CHA

- [ ] currently implementing
    - trait calls
        - flow-sensitive analysis
        - func rewriting

- [ ] what implementation is left before moving into Rust compiler?
    - function summaries

## notes


