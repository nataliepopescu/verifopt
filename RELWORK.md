# Potentially Related Work

## Defs/Concepts

what are verification condition generators?
- ah, these seem to be more so a part of how verification mechanics are
  implemented (given these pre-/post-conditions and this code what are the
  precise things we need to pass to the SMT solver)
    - always use Hoare logic?
- what are verification conditions?
  - logical (boolean) formulas

Hoare logic
- Hoare triple: {P}C{Q}
    - P = precondition
    - C = code/logic/command
    - Q = postcondition
- can only prove partial correctness (no termination)

abstract interpretation vs symbolic execution
- [Abstract Interpretation, Symbolic Execution and
  Constraints](https://drops.dagstuhl.de/storage/01oasics/oasics-vol086-gabbriellis-festschrift/OASIcs.Gabbrielli.7/OASIcs.Gabbrielli.7.pdf)
  - AI: sound over-approx of all possible _states_
  - SE: reachability analysis; all possible _execution paths_
  - both: "implicitly or explicitly – maintains constraints
during execution, in the form of invariants or path conditions."

## Language Internals

### Rust

[no-panic-rust](https://blog.reverberate.org/2025/02/03/no-panic-rust.html#fnref:0)

hints: [enforced integer range](https://www.reddit.com/r/rust/comments/1jafvbe/how_to_inform_the_rust_compiler_of_an_enforced/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)

[The state of Rust trying to catch up with Ada](https://fosdem.org/2025/schedule/event/fosdem-2025-5356-the-state-of-rust-trying-to-catch-up-with-ada/)
- Feb 2025
- apparently Ada has a lot of verification support
- [ferrocene](https://ferrocene.dev/en/)
    - compiler toolchain for safety-critical systems
    - subset of Rust compiler?
- subtypes
- Rust contracts...?
    - in the compiler
    - dynamic or static?
    - [contracts](https://docs.rs/contracts/latest/contracts/)
        - via procmacros
    - [PR](https://github.com/rust-lang/rust/pull/128045)
    - uses Kani
    - being used (where?) to prove some small rust code free of runtime exceptions

### Wasm

[one-pass
verification](https://stackoverflow.com/questions/48638653/can-anyone-help-explain-the-one-pass-verification-process-shows-in-webassembly)
- really validation, not verification ("is this valid wasm")
- validation of _types_ (but also other things)
    - [wasm type checking](https://binji.github.io/posts/webassembly-type-checking/)

[Mechanizing and Verifying the WebAssembly
Specification](https://www.cl.cam.ac.uk/~caw77/papers/mechanising-and-verifying-the-webassembly-specification.pdf)
- only 171 individual opcodes
- "but each of these can be viewed as
different specialisations of the 28 abstract operations spec-
ified in the working group’s paper"
- "explicit goal to eliminate undefined behavior from the specification"

- all memory accesses are bounds checked -> what does this mean for
  "optimizations" that try to remove bounds checks?
- no integer-pointer distinction

- validation == type checking

kind of do need mechanization to do verif
- b/c need to know what the lang actually does
- linter vs formal backing
- [clippy?](https://rust-lang.github.io/rust-clippy/master/index.html#missing_panics_doc)

## Rust Performance

[Towards Understanding the Runtime Performance of
Rust](https://dl.acm.org/doi/pdf/10.1145/3551349.3559494)
- Rust has a ~1.77x perf overhead (vs C)
- compiler-inserted runtime checks
- if these checks are disabled, Rust performance becomes like C
- good citation for why moving from dynamic -> static checking is good for
  performance

## Verification Tools

### Rust Verification

[Flux](https://dl.acm.org/doi/pdf/10.1145/3591283)
- refinement types / liquid inference ?
    - TODO: difference between refinement / dependent / liquid types
    - "synthesizing refinements via liquid type inference"
- type checker
- extends rust types
- quantifier-free (easier to reason about)
    - "using a program logic comes at the cost of complex user-specified universally quantified invariants"
- also uses stacked borrows rules apparently

- declarative type system == plug in to Rust compiler
    - spatial phase: use fxn sigs, program ids -> heap loc (map)
        - location refinements?
        - intermediate refinements still unknown
    - checking phase: refinement type checking (Horn vars for unknown refinements) -> system of Horn constraints
    - inference phase: solve the constraints

- 2 A TOUR OF FLUX

- 4 ALGORITHMIC VERIFICATION
    - flux == compiler plugin; operates on programs that have _already_ been analyzed by the compiler
        - benefits: IR contains inferred type info + assume Rust borrowing rules are satisfied
    - at MIR level; CFG
    - therefore flux doesn't technically pass info to the compiler, but rather the other way around (leverages compiler info)

[Verus: A Practical Foundation for Systems Verification](https://www.andrew.cmu.edu/user/bparno/papers/verus-sys.pdf)
- more focused on making verus practical (systems audience)
- optimizations

[Verus: Verifying Rust Programs Using Linear Ghost
Types](https://dl.acm.org/doi/pdf/10.1145/3586037)
- " In particular, we demonstrate the use of linear ghost
permissions that enable a program to take specific actions on specific resources, such as writing to a
memory location. Since the permissions are linear, they can track the evolving state of a resource in
the same way that separation logic formulas can track the state of a resource. **Since the permissions
are ghost, they exist only during type checking and verification, and do not impose any overhead
on compiled executable code.**"
- 3 modes: specifications, proof, and executable code
    - all are written in Rust (and go through Rust's type checker)
    - specification / proof code are checked for termination
    - proof / executable code are checked for linearity + borrowing
        - why is spec code not checked for linearity / borrowing?
    - ONLY executable is compiled to machine code

- long list of what Verus supports, is the list of what it doesn't support even
  longer? 

- extends the Rust compiler? 
    - "erases all ghost code (all specifications and proofs) before the code 
    is compiled to machine code"
- "driver" that links against the Rust compiler? what does this mean?

RustBelt
- proving unsafe impls encapsulated in well-typed interface

Oxide

Aeneas

Prusti
- heavyweight (according to Flux)
- encodes programs into Viper (a verification IR)

RustHorn
- heavyweight (according to Flux)

Creusot
- heavyweight (according to Flux)

What is "bounded" verification? 

[Kani](https://github.com/model-checking/kani)
- model-checking-based

### LLVM Verification

### Wasm Verification

[Mechanizing and Verifying the WebAssembly
Specification](https://www.cl.cam.ac.uk/~caw77/papers/mechanising-and-verifying-the-webassembly-specification.pdf)
- what does mechanization mean?
    - proof that the specification is correct?
    - in Isabelle
- also verified executable interpreter + type checker
- how does mechanization of the specification help with verification of
  properties?
    - can you verify properties about something written in a language that
      itself isn't proven sound? probably yes but there's the caveat that potential 
      unsoundness will undermine the "correctness" of whatever properties you
      have proved
- "The designers of WebAssembly have made it an explicit
goal to specify the language in a way that makes it amenable
to formal analysis and verification."

[Iris-Wasm](https://dl.acm.org/doi/pdf/10.1145/3591265)
- Conrad!
- mechanized higher-order separation logic
    - what about the framework / specifications is specific to separation logic
      and what does it mean to use it to verify non-seplogic things?
- Coq + Iris (not very automated; in fact not automated at all - interactive)
- emphasis on modules: how do modules exist in compiled code? 

- what is the verification burden?

- "Wasm is embedded within
a host language, which provides several important capabilities not available to core Wasm code,
including a complex, inherently higher-order, instantiation operation in which the declared state of
a WebAssembly module is allocated, the module’s requested imports are satisfied, and the module’s
declared exports are registered for use in satisfying further imports requested during subsequent
instantiations."
    - what host lang is compiled wasm embedded into?

- paper's relwork section
  - apparently shravan's paper (rlbox) has a similar pipeline: C -> wasm -> native
    (goal = sandboxing)
    - [rlbox](https://www.usenix.org/system/files/sec20-narayan.pdf)
    - uses matthew's paper's results: https://dl.acm.org/doi/pdf/10.1145/3498688
      - apparently shows that wasm -> native compilation "obeys a safe calling
        convention and certain isolation properties with respect to the rest of
        the system"
    - what benefits does wasm give rlbox?
  
  - CAP (Certified Assembly Programming) frameworks
    - "focuses on features that are abstracted away by Wasm"

  - potentially for automating Iris verification? : https://dl.acm.org/doi/10.1145/3519939.3523432

[Crocus](https://cs.wellesley.edu/~avh/veri-isle-preprint.pdf)
- Fraser!
- verification for instruction-lowering (from wasm to native code) in cranelift
    - instruction-lowering = instruction selection?
- what about Crocus is specific to these properties? (lowering)
    - operates on cranelift DSL (ISLE)
    - from a glance the syntax at least looks kind of similar to wasm

[Specification and Verification of WebAssembly
Programs](https://oa.upm.es/75802/1/TFM_DAVID_MUNUERA_MAZARRO.pdf)
- David Mazarro master's thesis (June 2023)
    - no other related papers sadly
- how to verify
    - "the VCs are derived from the WebAssembly bytecode and the specification by 
performing a symbolic execution of the WebAssembly code."
    - automatic CFG generation (similar pipeline/workflow to VeriWasm, except
      maybe cross-function)
      - toCFG + simplifyCFG functions
    - SCCs (strongly connected components) -> identify loops in CFG
        - helps w per-loop invariants (if a SCC has a cycle, we know its a loop,
          thus we require assertions)
- VerifiWASM (spec language)
    - apparently specs are written in a different file
    - this could be a benefit for automatically-generated code...
    - no support for global vars / mem management? 
    - "VerifiWASM is not intended to be used to write
specifications manually. The idea is that the HLL you choose that targets WebAssembly also
generates a specification written in VerifiWASM to accompany the WebAssembly bytecode that
is generated"
    - "In this sense, our tool is not batteries-included and we expect WASM-generating libraries and
compilers for HLLs to generate VerifiWASM specifications as part of the
compilation process."
        - does anyone do this...?
        - how hard would this be? i.e. would have to write a specification compiler
            - maybe if targeting certain types of annotations (i.e. refinement
              types) this wouldn't be so bad
    - doesn't use an existing verification IR
    - approach is similar to Dafny
- [wasm-verify](https://github.com/DavidMazarro/wasm-verify)
    - cmdline tool
    - verification condition generator

[Trust but Verify: SFI safety for native-compiled Wasm
](https://www.ndss-symposium.org/wp-content/uploads/ndss2021_5B-3_24078_paper.pdf)
- VeriWasm = static SFI verifier
    - what about it is specific to this context?
    - disassembles natively-compiled Wasm -> CFG
        - how does this work?
        - "As a part of this process, VeriWasm also resolves all
indirect jumps in the control-flow graph... and ensures that all direct and indirect calls target functions
present in the symbol table"
        - verifies each individual function wrt _local_ safety properties
            - potential limitations if trying to prove facts across functions
    - runs verified analysis passes on each function's CFG -> SFI safe?
    - uses abstract interpretation; "Abstract interpretation
is a static analysis technique that infers information about
a program by overapproximating its behavior"
- Lucet compiler: Wasm -> binary
- Yaxpeax: trusted disassembler

[KWASM](https://odr.chalmers.se/server/api/core/bitstreams/a06be182-a12e-46ce-94d3-cff7a5dc42ba/content)
- Hjort's master's thesis
- KWASM = mechanization of Wasm in the K framework -> formal verifiaction
    - K framework seems to be a language-agnostic tool for enabling formal
      verification
    - [wasm-semantics](https://github.com/runtimeverification/wasm-semantics)
- project goal == verify Wasm smart contracts
    - what about the approach is specific to smart contracts, if anything?
        - perhaps the types of properties?
        - embedding in Ethereum?

- K: a language compiler (generates tools for a lang given its spec)
    - auto-gen default lang things (classes, tab-completion, etc)
        - suite of defaults
    - so you can focus on the cool stuff

### Native Verification

[Verification of Safety Properties for Concurrent Assembly Code](https://flint.cs.yale.edu/flint/publications/vsca.pdf)
- Yale (zhong shao)
- Coq

## Compiler Output

[How Should Compilers Explain Problems to
Developers?](https://denaeford.me/papers/compiler-explanations-FSE-2018.pdf)
- from Chasins @ berkeley, about how to best formulate compiler error messages
- [Thesis version](https://static.barik.net/barik/thesis/barik-thesis.pdf)

## Optimization Techniques

[The Program Dependence Graph and Its Use in Optimization](https://www.cs.utexas.edu/~pingali/CS395T/2009fa/papers/ferrante87.pdf)
- 1987

[Equality
Saturation](https://rosstate.org/publications/eqsat/eqsat_tate_popl09.pdf)
- [video](https://rosstate.org/publications/eqsat/)
- POPL 2009

egg
- e-graphs
  - data structure for storing equivalence relations over terms in languages
  - dev'd in 1970s -> automated theorem provers
  - *equality saturation* is a distinct technique that _uses_ e-graphs
- project for building optimixers and synthesizers using e-graphs
- "An e-graph compactly represents many equivalent programs... egg makes
  e-graphs fast and flexible enough for use in program optimization and
  synthesis."
- [egg](https://egraphs-good.github.io/)
- [egg github (rust)](https://github.com/egraphs-good/egg)
- [docs.rs](https://docs.rs/egg/latest/egg/struct.EGraph.html)
- [egg paper](https://dl.acm.org/doi/pdf/10.1145/3434304)
    - POPL 2021

### Stochastic Optimization

[Stochastic Program
Optimization](https://theory.stanford.edu/~aiken/publications/papers/cacm16.pdf)
- CACM 2016
- HPC
- opt short, loop-free, fixed-point asm

- intro
    - comilers generally "Factoring the optimization problem
    into a collection of small subproblems"
    - "In many
    cases, the best possible code can only be obtained through
    the simultaneous consideration of mutually dependent
    issues such as instruction selection, register allocation, and
    target-dependent optimization."
    - seemingly argues that lower-level information is crucial for better
      optimizations
      - VERIFOPT might argue that higher-level information is _also_ crucial for
        better optimizations (dev intention)

    - incomplete search
        - correctness vs performance -> cost function (over all x86_64 loop-free
          instr seqs)
          - how long are these instr seqs? TODO
        - solve cost minimization problem
        - Markov Chain Monte Carlo (MCMC) sampler used to explore the cost
          function

- relwork
    - STOKE
        - 400 x86_64 opcodes
    - [Superoptimizer -- A Look at the Smallest
      Program](https://dl.acm.org/doi/pdf/10.1145/36177.36194)
      - enumerates code seqs of increasing length; chooses first w identical
        behavior (according to _test cases_, not some other equivalence
        measurement -- sketchy)
      - massively restricts the set of enumerable opcodes to 10-15
      - unlikely to scale to many more opcodes
    - [Denali](https://courses.cs.washington.edu/courses/cse501/15sp/papers/joshi.pdf)
      + [Equality
      Saturation](https://rosstate.org/publications/eqsat/eqsat_tate_popl09.pdf)
      - improved scalability; _only_ consider "equivalent" code seqs
      - explore via seccussive application of equality preserving
        transformations
      - goal-directed (TODO what exactly does this mean? is it like "i want to
        optimize this specific code chunk"?)
      - con: rely heavily on expert knowledge
        - TODO how / for what exactly?
      - "...whether a set of expert rules could ever cover the set of all
      possible interesting optimizations."
    - [Peephole
      Superoptimizer](https://www.usenix.org/legacy/event/osdi08/tech/full_papers/bansal/bansal.pdf)
      - auto-enumerates 32-bit x86 optimizations
      - stores in DB for later
      - search happens ONCE offline + no expert knowledge
      - sliding code window
      - local optimizations have their limitations (miss higher-level props)
    - [Sketching](https://people.csail.mit.edu/asolar/papers/asplos06-final.pdf)
      + [Brahma](https://susmitjha.github.io/papers/pldi11.pdf)
      - closely related problem: component-based sequence synthesis
      - "operate on statements in
      simplified bit-vector calculi rather than directly on hard-
      ware instructions... internal representations used by these
      systems preclude them from reasoning about the low-level
      performance properties of the code that they produce."

- cost minimization
    - function = eq(rewrite, target) + perf(rewrite)
        - each term seems to have some weights as well
        - eq == 0 if same
        - perf: lower == better
    - optimization is any rewrite where perf is better and eq == 0
    - search space is highly irregular (what does this mean?) and high
      dimensional (why? b/c many variables?)
    - MCMC sampling == general solution, tractable
        - also used in [code
          breaking](https://math.uchicago.edu/~shmuel/Network-course-readings/MCMCRev.pdf)
    - MCMC sampling
        - draw elems from probability distribution
        - draw more from higher probability areas
        - in cost minimization:
            - most samples are taken from the minimum (optimal) values of the
              func (theoretically / "in the limit")
            - in practice, "functions as an intelligent hill climbing
            method which is robust against irregular functions that are
            dense with local minima."
    - transform cost -> probability density function
        - common approach
            - iteratively finds new rewrites to apply
                - unclear why a proposal may be rejected
                - local acceptance criteria = "Metropolis–Hastings acceptance 
                  probability"
                - this is its own probability distribution
                - "Empirically, the best results
                are obtained for a distribution which makes both local pro-
                posals that make minor modifications to R and global pro-
                posals that induce major changes"
                - if proposed rewrite is better, accept; if its worse, possibly
                  accept (might be a local minima); if same, ...?
            - until computational budget is exhausted
                - what is an example of a computational budget?
            - rewrite proposals must be *ergodic*
                - can transform one code seq into another given some number of
                  mods
                - what is an example of a rewrite that is not ergodic?
            - "in the limit" produces a "sequence of samples distributed in 
              proportion to their cost"

- cost minimzation for x86
    - eq() impl: symbolic validator + binary indicator (0 or 1)

[Stochastic
Superoptimization](https://theory.stanford.edu/~aiken/publications/papers/asplos13.pdf)

[Stochastic Optimization of Floating-Point Programs with Tunable
Precision](https://theory.stanford.edu/~aiken/publications/papers/pldi14a.pdf)

## Using Static Analysis for Optimization

[From Verification to
Optimizations](https://kedar-namjoshi.github.io/papers/Gjomemo-Namjoshi-Phung-Venkatakrishnan-Zuck-VMCAI-2015.pdf)
- link external static analysis tools into compilers
- *propagate source-level info into optimization pipeline*
- VMCAI 2015
- cited by:
  https://scholar.google.com/scholar?cites=15323456212062700795&as_sdt=5,31&sciodt=0,31&hl=en
  - likely most relevant: 
    - [SecureDelivery of Program Properties through Optimizing
      Compilation](https://dl.acm.org/doi/pdf/10.1145/3377555.3377897)
    - [Leveraging Static Analysis Tools for Improving Usability of Memory Error
      Sanitization
      Compilers](https://ieeexplore.ieee.org/stamp/stamp.jsp?arnumber=7589812)

- abstract
  - *compiler static analysis limited by compilation time budgets*
    - is this the main reason?
  - instead, link external static analysis tools into compiler
  - assertions from Frama-C source code analysis -> LLVM
  - improve effectiveness of several optimizations

- intro
  - 'the GCC wiki has as rule 1: ruleDo not add algorithms with quadratic or
  worse behavior, ever."'
  - 'While the
  compile-time cost of employing additional tools may be high, it is often the
  case that runtime improvements in optimization outweigh this additional cost,
  for example, for large frequently used code such as kernels and name servers.'
    - kind of undermines the initial argument if time budgets are the main
      constraint (weakens that statement)
  - make these features optional
  - "modular approach, which decouples analysis from transformation"

  - challenge
    - propagating info known about source code to certain optimization stages
      requires somehow transforming that info (i.e. through opt stages?)
      - e.g. what if variables are renamed
    - refinement "witnesses"
    - proposal = "instrumenting the optimization to produce a refinement relation
      as it transforms a program."
      - check validity of generated relation via SMT
      - valid relation = "witness" to correctness of opt
  - common case seems to mainly rely on range (interval) analysis

- approach by example
  - Frama-C => value analysis (abstract-interp - based) => get domains of
    integers
    - the vibe is that the "verification" part is specific to integer value
      resolution (explains why eval focuses on bounds checks / arithmetic
      overflow checks)
      - note, no loops (interval analysis)

- external invariant usage in LLVM (methodology)
  - refinement relations (optimization validity / target equality to source)
    - target = optimization(source); opt == transformer
    - correct optimization if every possible behavior of T is also in S
    - is S is "transition deterministic" + S and T have identical initial
      states, then S has no "extra" behavrios wrt T
    - refinement: map T states in S states

  - invariant propagation
    - existence of refinement relation == witness to the correctness of an
      optimization
    - winess ALSO is a means for propagating invariants

  - generating witnesses
    - refinement is generally undecidable
    - augment opt w witness generator -> pass candidate witnesses to refinement
      checker
      - TODO does this mean it is incomplete? what exactly now makes this a
        decidable problem?
        - expressed in a special logic?

  - effective manipulation of witnesses
    - " the invariants obtained from Frama-C are of the form ∧
    v∈V lv ≤ v ≤ hv where the lv and hv are integer constants. The
    transformations of of the form VT = E(VS ) where E is a simple arithmetic
    expression over VS . "

- system description

  - LLVM background
    - "By this, we decouple the need for updating the compiler
    frequently as newer or improved program analysis algorithms become
    available,
    as our system is designed to obtain assertions from cutting-edge program
    analy-
    sis tools such as Frama-C"

  - practical challenges

    - source-IR mapping
      - i.e. propagating invariants
      - b/c SSA, every source var => several SSA versions
        - invariants must also be bound to each SSA version
        - TODO what exactly do we mean by an SSA version?
          - a single variable name will expand into one of several SSA
            assignments, each of these is a "version"

        - [SSA
          explanation](https://www.cs.cornell.edu/courses/cs6120/2022sp/lesson/6/)
          - "The idea in SSA is to convert general programs into SSA form, do all
            our optimization there, and then convert back to a standard mutating
            form before we generate backend code."
          - SSA solves the problem of variable name conflicts
          - phi-functions/nodes
            - flow-sensitive copy instrs; determine which version of a variable
              name to use depending on control flow

    - intermediate operations

    - code transformation

  - system architecture
    - source code is annotated w invariants (from static analysis) + compiled
      into LLVM IR
    - IR processed by 2 new passes
        - annotation mapping
            - binds assertions to SSA versions
        - annotation propagation
            - combines assertions
            - propagates them to related intermediate ops
        - run before any other pass

    - opts:
        - array bounds check INSERTION
            - TODO why is this an "optimization"?
                - guess this was a typo; in fact removing the bounds checks
        - integer overflow check REMOVAL
            - written by the authors
            - "safely remove run time checks inserted by
            LLVM when it is invoked with the bounds-checking option"
        - instruction combination
            - modified pass
            - comparison simplifications

    - CIL-based rewriter
        - assertions via input file
        - written in subset of ACSL
        - goal: support many program analysis tools -> assertions
        - thus assertions are encoded as "dummy" strings (to be tool-agnostic)
        - CIL rewriter: injects these assertions into the source code
            - [CIL = C Intermediate
              Language](https://scottmcpeak.com/papers/cil_cc02.pdf)

    - Annotation Mapping
        - uses debug info + loads

    - Annotation Propagation

  - optimizations

    - bounds check REMOVAL
      - "If, however, at compile time, it can be proved that an array access
        will never
        be out of bounds during execution, then the bounds checks on that access
        can
        be removed."
      - modify Safecode -> consult metadata (index range + array size) 
        - TODO but not assertions? or are the assertions/invariants now present
          as "metadata"?

    - overflow check REMOVAL
      - "As shown in Fig. 5, these options transform every oper-
      ation that may result in overow into a procedure call (L3)"
      - pass " checks the possible value range of the result" removes overflow
        checks if overflow is impossible

    - instruction combination
      - e.g. integer comparison instruction -> res in bool
      - can fold comparison into true or false

- eval
  - Frama-C = abstract interpretation
  - apparently tool injects assertions according to Frama-C range analysis
    findings (4.3)
    - what is the rewriting step?


[Perceus: Garbage Free Reference Counting with Reuse](https://dl.acm.org/doi/pdf/10.1145/3453483.3454032)
- from mae

## Optimize + Verify

[VSync: Push-Button Verification and Optimization for Synchronization Primitives on Weak Memory Models](https://dl.acm.org/doi/abs/10.1145/3445814.3446748)
- automatically optimize + verify? how?

[EverCrypt: A Fast, Verified, Cross-Platform Cryptographic
Provider](https://www.microsoft.com/en-us/research/wp-content/uploads/2019/10/evercrypt.pdf)
- verified, high-perf crypto
- "new verified implementations (including hashes,
Curve25519, and AES-GCM) whose performance matches or
exceeds the best unverified implementations"
- builds off of ValeCrypt work
    - [Vale](https://web.eecs.umich.edu/~manosk/assets/papers/vale-sec17.pdf)
    - [A Verified, Efficient Embedding of a Verifiable Assembly
    Language](http://nickgian.github.io/popl19-vale.pdf)
        - "The key idea is to use the computational
        power of a *dependent type system’s type checker* to run a verified *verification-condition generator* during
        type checking. This allows the embedding to *customize the verification condition* sent by the type checker
        to an SMT solver"
- "we show that multiple layers of abstraction, both in the
implementation and (more surprisingly) in the specification of
cryptographic algorithms, naturally leads to agile code while
also improving verification efficiency."
- F* proof assistant

- is there a clear answer/intuition for why their implementations outperform
  pre-existing ones?
  - VII 
    - "With EverCrypt, we aim to obviate the need for such a
choice; thanks to verification, we can offer developers best-in-
class performance and provable security guarantees"
    - VII-A (AES-GCM)
        - "To simplify the task of deciphering OpenSSL’s code
and reconstructing the invariants the original programmer had in
mind, we initially ported the encryption and the authentication
operations separately, proving that each accomplished its goals
individually. We then manually merged the implementations
and proofs into single implementation and proof. In the future,
we hope to develop techniques to automate such merges."
  - VIII-B (perf eval)
    - "value of optimizing for particular platforms" - targeted (not portable)
      versions are those that surpass the perf of preexisting impls (fig 8)
    - "We attribute this to the fact that the latter two
each jointly optimize encryption and authentication together,
whereas libjc optimizes the two primitives separately"
    - "EverCrypt’s combined Low* +Vale implementation" (Section VI-B)

## Verified Optimizers/Compilers

[Verified Tensor-Program Optimization Via High-Level Scheduling Rewrites](https://dl.acm.org/doi/pdf/10.1145/3498717)
- optimizer w verified rewrite rules (Coq proofs)
- "easy" extension (adding new rewrite rules)

[CompCert - A Formally Verified Optimizing Compiler](https://inria.hal.science/hal-01238879v1/document)

[Verified Software Toolchain](https://www.cs.princeton.edu/~appel/papers/vst.pdf)

[Formal Verification of SSA-Based Optimizations for LLVM](https://dl.acm.org/doi/pdf/10.1145/2491956.2462164)

[Verified Peephole Optimizations for CompCert](https://dl.acm.org/doi/pdf/10.1145/2908080.2908109)

## Using Optimization Problems for Verification

unsure what is meant by "verification" and "optimization" in these contexts,
think it's very different

[Thesis: Optimization-Based Methods for Nonlinear and Hybrid Systems Verification](https://thesis.library.caltech.edu/2155/1/thesis.pdf)

[Modeling, Optimization and Computation for Software Verification](https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=1edbc45f98bfa52b43f06e6bcb07b490f7bc127e)

[Optimixation of Lyapunov Invariants in Verification of Software Systems](https://ieeexplore.ieee.org/abstract/document/6416001)
- search for Lyapunov invariants == convex optimization problem

[Optimization-Based Verification and Stability Characterizaation of Piecewise Affine and Hybrid Systems](https://link.springer.com/chapter/10.1007/3-540-46430-1_8)
