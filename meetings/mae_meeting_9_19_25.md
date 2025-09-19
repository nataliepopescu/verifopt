# notes

TODO what does enabling the `ptr_metadata` feature do?
- introduce unsafety or runtime overhead? (usually one of those two)

does the extra MIR flexibility make analysis much harder? (vs source code, with many more cases)
- still certain important invariants @ MIR
 - referential integrity (which vars are live + what they point to); vs C++
  - i.e. type safety + lifetime analysis

test Storage events need
- tight loop w lots of storage events (run vallgrind)

drop vs StorageDead?

look at post-pass MIR and see if its "efficient enough"

vtable lookup
- api?
- search data structure?
- worst case: look at LLVM

