# Debugging RipGrep Perf

something gets really really slow

noticing
- very large constraints (disjuncts in the millions) - no bueno

when constraints are first converted, how big are they?
- first converted i.e. from CONVERTER
- often copying/moving/wrapping w stuff, so this is not usually the FIRST
  instance
  - that might be base_constraints

then, generally, when do they start getting big?
- they get big quickly, but "big" is the internal structure (hierarchical) of
  the constraint, not necessarily the top-level number of distinct constraints
- still dont know if addressing this is where the real bottleneck is though

trying a timing approach
- more granular info, within basic block interpretation
- execution does in slower over time *in particular places*, which maps to the
  idea that _some_ places may be accumulating massive constraints while others
  might not

next step: where exactly are these places and what is happening in them?

took a break from this and tried instrumenting all the different blocks of code
(getting gradually more granular for those that were expensive). top (worst)
performers are:
- TermMergeWtosUnion
    - a loop around an ImHashMap union() operation
- TermMergeRefsUnion
    - a single ImHashMap union() operation

then:
- TermMergeMapvalsMerge
    - wraps the merge_mapvals operation
- TermMergePerKeyMapvals
    - merging loop containing ImHashMap insert() operations (in addition to
      TermMergeMapvalsMerge logic)


## first "big" disjunct

_19 = Use(Copy((_8.1: core::num::niche_types::UsizeNoHighBit)))
- disjuncts = 106

_8 = Use(Move(((_11 as variant#0).0: alloc::raw_vec::RawVecInner)))
- disjuncts = 55

_11 = direct call to try_allocate_in
- disjuncts = 68

_0 in try_allocate_in
- Aggregate(Adt(AdtDef(DefId { id: 30807, name: "std::result::Result" }), VariantIdx(0, ThreadLocalIndex), GenericArgs([Type(Ty { id: 5787, kind: RigidTy(Adt(AdtDef(DefId { id        : 55390, name: "alloc::raw_vec::RawVecInner" }), GenericArgs([Type(Ty { id: 178, kind: RigidTy(Adt(AdtDef(DefId { id: 39166, name: "std::alloc::Global" }), GenericArgs([]))) })]))        ) }), Type(Ty { id: 522, kind: RigidTy(Adt(AdtDef(DefId { id: 29642, name: "std::collections::TryReserveError" }), GenericArgs([]))) })]), None, None), [Move(_10)])
    - disjuncts = 23
- Aggregate(Adt(AdtDef(DefId { id: 30807, name: "std::result::Result" }), VariantIdx(0, ThreadLocalIndex), GenericArgs([Type(Ty { id: 5787, kind: RigidTy(Adt(AdtDef(DefId { id: 55390, name: "alloc::raw_vec::RawVecInner" }), Ge        nericArgs([Type(Ty { id: 178, kind: RigidTy(Adt(AdtDef(DefId { id: 39166, name: "std::alloc::Global" }), GenericArgs([]))) })]))) }), Type(Ty { id: 522, kind: RigidTy(Adt(AdtDef(DefId { id: 29642, name: "std::collections::TryReser        veError" }), GenericArgs([]))) })]), None, None), [Move(_22)]) 
    - disjuncts = 34
- Aggregate(Adt(AdtDef(DefId { id: 30807, name: "std::result::Result" }), VariantIdx(1, ThreadLocalIndex), GenericArgs([Type(Ty { id: 5787, kind: RigidTy(Adt(AdtDef(DefId { id: 55390, name: "alloc::raw_vec::RawVecInner" }), Ge        nericArgs([Type(Ty { id: 178, kind: RigidTy(Adt(AdtDef(DefId { id: 39166, name: "std::alloc::Global" }), GenericArgs([]))) })]))) }), Type(Ty { id: 522, kind: RigidTy(Adt(AdtDef(DefId { id: 29642, name: "std::collections::TryReser        veError" }), GenericArgs([]))) })]), None, None), [Move(_20)])
    - disjuncts = 8
- Aggregate(Adt(AdtDef(DefId { id: 30807, name: "std::result::Result" }), VariantIdx(1, ThreadLocalIndex), GenericArgs([Type(Ty { id: 5787, kind: RigidTy(Adt(AdtDef(DefId { id: 55390, name: "alloc::raw_vec::RawVecInner" }), Ge        nericArgs([Type(Ty { id: 178, kind: RigidTy(Adt(AdtDef(DefId { id: 39166, name: "std::alloc::Global" }), GenericArgs([]))) })]))) }), Type(Ty { id: 522, kind: RigidTy(Adt(AdtDef(DefId { id: 29642, name: "std::collections::TryReser        veError" }), GenericArgs([]))) })]), None, None), [Constant(ConstOperand { span: Span { id: 6, repr: "no-location" }, user_ty: None, const_: MirConst { kind: Allocated(Allocation { bytes: [Some(0), Some(0), Some(0), Some(0), Some(        0), Some(0), Some(0), Some(0), None, None, None, None, None, None, None, None], provenance: ProvenanceMap { ptrs: [] }, align: 8, mutability: Not }), ty: Ty { id: 522, kind: RigidTy(Adt(AdtDef(DefId { id: 29642, name: "std::collec        tions::TryReserveError" }), GenericArgs([]))) }, id: MirConstId(173, ThreadLocalIndex) } })])
    - disjuncts = 3

3 + 8 + 34 + 23 = 68

this is seemingly in the second call to try_allocate_in

first call is just for summary-building? 
- this is bloating the constraints
- maybe worth looking at a smaller example
- oh, i missed a call


## using perf results + spikes: look at first "spike" (Parser::next)

in statements:
- SetScoped + WriteFields spike

what happens in SetScoped?
- scoped_replace OR scoped_update

what happens in WriteFields?

actually, looking at the aggregate numbers, terminators are more expensive than
ALL the statements (per basic block) _combined_ (hence total_ms) - this seems important
- whats happening at terminators
    - instrument differences:
        - direct calls
        - indirect calls
        - switches
        - returns
        - other

- most of the time direct calls dominate

- currently looking at Parser::next() return spike (likely due to recursion)
    - source code definitely has recursion
    - now instrumenting perf for fast path (in-queue stuff) vs slow path
      (reinterp_recursion + visit_body again)
        - each of slow path is being measured

- yeah, the return spike is due to recursion in the first two cases
- another case later, theres a return spike but no recursion - check that scope

