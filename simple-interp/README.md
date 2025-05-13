# simple-interp

Undefined behavior:

- nested function definitions

Implementation notes:

- SSA-checks are effectively spread across `func_collector.rs` and 
  `interpreter.rs` (instead of in a separate pass)
