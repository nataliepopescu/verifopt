# MIR pass low-level details/construction

## pre-rewrite MIR

## post-rewrite MIR goal (transmute_cp)

dumped MIR (from `-Z dump-mir=main`):

```
bb14: {
    StorageLive(_22);
    StorageLive(_23);
    StorageLive(_24);
    _24 = &_17;
    _23 = &(*_24);
    _22 = transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(move _23) -> [return: bb15, unwind: bb33];
}

bb17: {
    StorageDead(_31);
    StorageLive(_29);
    _29 = copy (_30.1: *const usize);
    StorageDead(_32);
    StorageDead(_30);
    StorageLive(_33);
    StorageLive(_34);
    _34 = copy _21;
    StorageLive(_35);
    _35 = copy _25;
    _33 = Eq(move _34, move _35);
    switchInt(move _33) -> [0: bb21, otherwise: bb18];
}

bb18: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_36);
    StorageLive(_37);
    StorageLive(_38);
    _56 = const false;
    _38 = move _17;
    _37 = Box::<dyn Animal>::into_raw(move _38) -> [return: bb19, unwind: bb33];
}

bb19: {
    StorageDead(_38);
    _36 = move _37 as *const () (PtrToPtr);
    StorageDead(_37);
    StorageLive(_39);
    StorageLive(_40);
    StorageLive(_41);
    _41 = copy _36;
    _40 = move _41 as &Cat (Transmute);
    _39 = &(*_40);
    StorageDead(_41);
    StorageDead(_40);
    StorageLive(_42);
    StorageLive(_43);
    _43 = &(*_39);
    _42 = <Cat as Animal>::speak(move _43) -> [return: bb20, unwind: bb33];
}

bb20: {
    StorageDead(_43);
    StorageDead(_42);
    _0 = const ();
    StorageDead(_39);
    StorageDead(_36);
    goto -> bb27;
}

bb21: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_44);
    StorageLive(_45);
    _45 = copy _21;
    StorageLive(_46);
    _46 = copy _29;
    _44 = Eq(move _45, move _46);
    switchInt(move _44) -> [0: bb25, otherwise: bb22];
}

bb22: {
    StorageDead(_46);
    StorageDead(_45);
    StorageLive(_47);
    StorageLive(_48);
    StorageLive(_49);
    _56 = const false;
    _49 = move _17;
    _48 = Box::<dyn Animal>::into_raw(move _49) -> [return: bb23, unwind: bb33];
}

bb23: {
    StorageDead(_49);
    _47 = move _48 as *const () (PtrToPtr);
    StorageDead(_48);
    StorageLive(_50);
    StorageLive(_51);
    StorageLive(_52);
    _52 = copy _47;
    _51 = move _52 as &Dog (Transmute);
    _50 = &(*_51);
    StorageDead(_52);
    StorageDead(_51);
    StorageLive(_53);
    StorageLive(_54);
    _54 = &(*_50);
    _53 = <Dog as Animal>::speak(move _54) -> [return: bb24, unwind: bb33];
}

bb24: {
    StorageDead(_54);
    StorageDead(_53);
    _0 = const ();
    StorageDead(_50);
    StorageDead(_47);
    goto -> bb26;
}

bb33 (cleanup): {
    drop(_20) -> [return: bb34, unwind terminate(cleanup)];
}
```

human-readible basic block names:

```
bb_transmute: {
    StorageLive(_22);
    StorageLive(_23);
    StorageLive(_24);
    _24 = &_17;
    _23 = &(*_24);
    _22 = transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(move _23) -> [return: bb_get_vtable_ptrs, unwind: bb_unwind];
}

bb_get_vtable_ptrs: {}

bb_first_compare: {
    StorageDead(_31);
    StorageLive(_29);
    _29 = copy (_30.1: *const usize);
    StorageDead(_32);
    StorageDead(_30);
    StorageLive(_33);
    StorageLive(_34);
    _34 = copy _21;
    StorageLive(_35);
    _35 = copy _25;
    _33 = Eq(move _34, move _35);
    switchInt(move _33) -> [0: bb_second_compare, otherwise: bb_cat_into_raw];
}

bb_cat_into_raw: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_36);
    StorageLive(_37);
    StorageLive(_38);
    _56 = const false;
    _38 = move _17;
    _37 = Box::<dyn Animal>::into_raw(move _38) -> [return: bb_cat_speak, unwind: bb_unwind];
}

bb_cat_speak: {
    StorageDead(_38);
    _36 = move _37 as *const () (PtrToPtr);
    StorageDead(_37);
    StorageLive(_39);
    StorageLive(_40);
    StorageLive(_41);
    _41 = copy _36;
    _40 = move _41 as &Cat (Transmute);
    _39 = &(*_40);
    StorageDead(_41);
    StorageDead(_40);
    StorageLive(_42);
    StorageLive(_43);
    _43 = &(*_39);
    _42 = <Cat as Animal>::speak(move _43) -> [return: bb_cat_goto_drop, unwind: bb_unwind];
}

bb_cat_goto_drop: {
    StorageDead(_43);
    StorageDead(_42);
    _0 = const ();
    StorageDead(_39);
    StorageDead(_36);
    goto -> bb27;
}

bb_second_compare: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_44);
    StorageLive(_45);
    _45 = copy _21;
    StorageLive(_46);
    _46 = copy _29;
    _44 = Eq(move _45, move _46);
    switchInt(move _44) -> [0: bb25, otherwise: bb_dog_into_raw];
}

bb_dog_into_raw: {
    StorageDead(_46);
    StorageDead(_45);
    StorageLive(_47);
    StorageLive(_48);
    StorageLive(_49);
    _56 = const false;
    _49 = move _17;
    _48 = Box::<dyn Animal>::into_raw(move _49) -> [return: bb_dog_speak, unwind: bb_unwind];
}

bb_dog_speak: {
    StorageDead(_49);
    _47 = move _48 as *const () (PtrToPtr);
    StorageDead(_48);
    StorageLive(_50);
    StorageLive(_51);
    StorageLive(_52);
    _52 = copy _47;
    _51 = move _52 as &Dog (Transmute);
    _50 = &(*_51);
    StorageDead(_52);
    StorageDead(_51);
    StorageLive(_53);
    StorageLive(_54);
    _54 = &(*_50);
    _53 = <Dog as Animal>::speak(move _54) -> [return: bb_dog_goto_drop, unwind: bb_unwind];
}

bb_dog_goto_drop: {
    StorageDead(_54);
    StorageDead(_53);
    _0 = const ();
    StorageDead(_50);
    StorageDead(_47);
    goto -> bb26;
}

bb_unwind (cleanup): {
    drop(_20) -> [return: bb34, unwind terminate(cleanup)];
}
```

actual MIR construction: 

- what happens if we omit all the StorageDead/Live instructions?

```
bb_transmute {
    StatementKind::Assign(Box<(Place, RValue::Ref(Region::ReErased, BorrowKind::Shared, Place)))
    TerminatorKind::Call {
        func: transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>
        args: [Spanned { node: Move, span: srclines }]
        destination: Place
        target: Some(bb_get_vtable_ptrs)
        unwind: UnwindAction::Cleanup(bbunwind)
        call_source: Callsource::Normal
        fn_span: srclines
    }
}

bb_get_vtable_ptrs {
    // ???
}

bb_first_compare {
    StatementKind::Assign(Box<(Place, RValue::BinaryOp(Eq, Box(Move, Move)))>)
    TerminatorKind::SwitchInt {
        discr: Move(Place),
        SwitchTargets {
            values: [Pu128(0)], // excludes fallback (= otherwise)
            targets: [bb_second_compare, bb_cat_into_raw], // includes fallback
        }
    }
}

bb_cat_into_raw {
    StatementKind::Assign(Box<(Place, RValue::Use(Move(Place)))>)
    TerminatorKind::Call {
        func: Box::<dyn Animal>::into_raw
        args: [Spanned { node: Move, span: srlines }]
        destination: Place,
        target: Some(bb_cat_speak)
        unwind: UnwindAction::Cleanup(bbunwind)
        call_source: CallSource::Normal
        fn_span: srclines
    }
}

bb_cat_speak {
    StatementKind::Assign(Box<(Place, RValue::Cast(
        CastKind::PtrToPtr,
        Move(Place),
        *const ()
    ))>)
    StatementKind::Assign(Box<(Place, RValue::Cast(
        CastKind::Transmute,
        Move(Place),
        &'{erased} Cat
    ))>)
    TerminatorKind::Call {
        func: <Cat as Animal>::speak
        args: [Spanned { node: Move(Place), span: srclines }]
        destination: Place
        target: Some(bbgotodrop)
        unwind: UnwindAction::Cleanup(bbunwind)
        call_source: CallSource::Normal
        fn_span: srclines
    }
}

// the rest (Dog version) is just like the above
```

## post-rewrite MIR goal (ptr_metadata)

dumped MIR (from `-Z dump-mir=main`):

```
bb14: {
    StorageLive(_21);
    StorageLive(_22);
    StorageLive(_23);
    _51 = copy ((_17.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    _23 = &(*_51);
    _22 = &raw const (*_23);
    _21 = std::ptr::metadata::<dyn Animal>(move _22) -> [return: bb15, unwind: bb33];
}

bb17: {
    StorageDead(_28);
    StorageDead(_29);
    StorageLive(_30);
    StorageLive(_31);
    StorageLive(_32);
    _50 = const false;
    _32 = move _17;
    _31 = Box::<dyn Animal>::into_raw(move _32) -> [return: bb18, unwind: bb33];
}

bb18: {
    StorageDead(_32);
    _30 = move _31 as *const () (PtrToPtr);
    StorageDead(_31);
    StorageLive(_33);
    StorageLive(_34);
    _34 = &_21;
    StorageLive(_35);
    _35 = &_24;
    _33 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _34, move _35) -> [return: bb19, unwind: bb33];
}

bb19: {
    switchInt(move _33) -> [0: bb22, otherwise: bb20];
}

bb20: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_36);
    StorageLive(_37);
    StorageLive(_38);
    _38 = copy _30;
    _37 = move _38 as &Cat (Transmute);
    _36 = &(*_37);
    StorageDead(_38);
    StorageDead(_37);
    StorageLive(_39);
    StorageLive(_40);
    _40 = &(*_36);
    _39 = <Cat as Animal>::speak(move _40) -> [return: bb21, unwind: bb33];
}

bb21: {
    StorageDead(_40);
    StorageDead(_39);
    _0 = const ();
    StorageDead(_36);
    goto -> bb28;
}

bb22: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_41);
    StorageLive(_42);
    _42 = &_21;
    StorageLive(_43);
    _43 = &_27;
    _41 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _42, move _43) -> [return: bb23, unwind: bb33];
}

bb23: {
    switchInt(move _41) -> [0: bb26, otherwise: bb24];
}

bb24: {
    StorageDead(_43);
    StorageDead(_42);
    StorageLive(_44);
    StorageLive(_45);
    StorageLive(_46);
    _46 = copy _30;
    _45 = move _46 as &Dog (Transmute);
    _44 = &(*_45);
    StorageDead(_46);
    StorageDead(_45);
    StorageLive(_47);
    StorageLive(_48);
    _48 = &(*_44);
    _47 = <Dog as Animal>::speak(move _48) -> [return: bb25, unwind: bb33];
}

bb25: {
    StorageDead(_48);
    StorageDead(_47);
    _0 = const ();
    StorageDead(_44);
    goto -> bb27;
}

bb26: {
    StorageDead(_43);
    StorageDead(_42);
    _0 = const ();
    goto -> bb27;
}

bb33 (cleanup): {
    drop(_20) -> [return: bb34, unwind terminate(cleanup)];
}
```

human-readible basic block names:

```
bb_transmute: {
    StorageLive(_21);
    StorageLive(_22);
    StorageLive(_23);
    _51 = copy ((_17.0: std::ptr::Unique<dyn Animal>).0: std::ptr::NonNull<dyn Animal>) as *const dyn Animal (Transmute);
    _23 = &(*_51);
    _22 = &raw const (*_23);
    _21 = std::ptr::metadata::<dyn Animal>(move _22) -> [return: bb_get_vtable_ptrs, unwind: bb_unwind];
}

bb_get_vtable_ptrs: {}

bb_animal_into_raw: {
    StorageDead(_28);
    StorageDead(_29);
    StorageLive(_30);
    StorageLive(_31);
    StorageLive(_32);
    _50 = const false;
    _32 = move _17;
    _31 = Box::<dyn Animal>::into_raw(move _32) -> [return: bb_first_compare, unwind: bb_unwind];
}

bb_first_compare: {
    StorageDead(_32);
    _30 = move _31 as *const () (PtrToPtr);
    StorageDead(_31);
    StorageLive(_33);
    StorageLive(_34);
    _34 = &_21;
    StorageLive(_35);
    _35 = &_24;
    _33 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _34, move _35) -> [return: bb_first_switch, unwind: bb_unwind];
}

bb_first_switch: {
    switchInt(move _33) -> [0: bb_second_compare, otherwise: bb_cat_speak];
}

bb_cat_speak: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_36);
    StorageLive(_37);
    StorageLive(_38);
    _38 = copy _30;
    _37 = move _38 as &Cat (Transmute);
    _36 = &(*_37);
    StorageDead(_38);
    StorageDead(_37);
    StorageLive(_39);
    StorageLive(_40);
    _40 = &(*_36);
    _39 = <Cat as Animal>::speak(move _40) -> [return: bb_post_cat_goto, unwind: bb_unwind];
}

bb_post_cat_goto: {
    StorageDead(_40);
    StorageDead(_39);
    _0 = const ();
    StorageDead(_36);
    goto -> bb28;
}

bb_second_compare: {
    StorageDead(_35);
    StorageDead(_34);
    StorageLive(_41);
    StorageLive(_42);
    _42 = &_21;
    StorageLive(_43);
    _43 = &_27;
    _41 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _42, move _43) -> [return: bb_second_switch, unwind: bb_unwind];
}

bb_second_switch: {
    switchInt(move _41) -> [0: bb_goto, otherwise: bb_dog_speak];
}

bb_dog_speak: {
    StorageDead(_43);
    StorageDead(_42);
    StorageLive(_44);
    StorageLive(_45);
    StorageLive(_46);
    _46 = copy _30;
    _45 = move _46 as &Dog (Transmute);
    _44 = &(*_45);
    StorageDead(_46);
    StorageDead(_45);
    StorageLive(_47);
    StorageLive(_48);
    _48 = &(*_44);
    _47 = <Dog as Animal>::speak(move _48) -> [return: bb_post_dog_goto, unwind: bb_unwind];
}

bb_post_dog_goto: {
    StorageDead(_48);
    StorageDead(_47);
    _0 = const ();
    StorageDead(_44);
    goto -> bb27;
}

bb_goto: {
    StorageDead(_43);
    StorageDead(_42);
    _0 = const ();
    goto -> bb27;
}

bb_unwind (cleanup): {
    drop(_20) -> [return: bb34, unwind terminate(cleanup)];
}
```

actual MIR construction: 

```
bb_transmute: {
    StatementKind::StorageLive(_21);
    StatementKind::StorageLive(_22);
    StatementKind::StorageLive(_23);
    StatementKind::Assign(Box<(_51, Rvalue::Cast(
        CastKind::Transmute,
        Copy(_17.0),
        Ty: *const dyn [Binder { value: Trait(Animal), bound_vars: [] }] + '{erased}
    ))>)
    StatementKind::Assign(Box<_23, Rvalue::Ref(Region::ReErased, BorrowKind::Shared, _51)>)
    StatementKind::Assign(Box<_22, Rvalue::RawPtr(RawPtrKind::Const, _23)>)
    TerminatorKind::Call {
        func: std::ptr::metadata::<dyn Animal>
        args: [Spanned { node: Move(_22), span: srclines }]
        destination: _21
        target: Some(bb_get_vtable_ptrs)
        unwind: Cleanup(bb_unwind)
        call_source: Normal
        fn_span: srclines
    }
}

bb_get_vtable_ptrs: {}

bb_animal_into_raw: {
    StatementKind::StorageDead(_28);
    StatementKind::StorageDead(_29);
    StatementKind::StorageLive(_30);
    StatementKind::StorageLive(_31);
    StatementKind::StorageLive(_32);
    StatementKind::Assign(Box<_50, Rvalue::Use(Constant(false))>)
    StatementKind::Assign(Box<_32, Rvalue::Use(Move(_17)>)
    TerminatorKind::Call {
        func: Box::<dyn Animal>::into_raw
        args: [Spanned { node: Move(_32), span: srclines }]
        destination: _31
        target: Some(bb_first_compare)
        unwind: Some(bb_unwind)
        call_source: Normal
        fn_span: srclines
    }
}

bb_first_compare: {
    StatementKind::StorageDead(_32);
    StatementKind::Assign(Box<(_30, Rvalue::Cast(
        CastKind::PtrToPtr,
        Move(_31),
        Ty: *const (),
    ))>)
    StatementKind::StorageDead(_31);
    StatementKind::StorageLive(_33);
    StatementKind::StorageLive(_34);
    StatementKind::Assign(Box<(_34, Rvalue::Ref(Region::ReErased, BorrowKind::Shared, _21))>)
    StatementKind::StorageLive(_35);
    StatementKind::Assign(Box<(_35, Rvalue::Ref(Region::ReErased, BorrowKind::Shared, _24))>)
//    _33 = <DynMetadata<dyn Animal> as PartialEq>::eq(move _34, move _35) -> [return: bb_first_switch, unwind: bb_unwind];
    TerminatorKind::Call {
        func: <DynMetadata<dyn Animal> as PartialEq>::eq
        args: [Spanned {
            node: Move(_34),
            span: srclines,
        }, Spanned {
            node: Move(_35),
            span: srclines,
        }]
        destination: _33
        target: Some(bb_first_switch)
        unwind: Some(bb_unwind)
        call_source: OverloadedOperator
        fn_span: srclines
    }
}

bb_first_switch: {
    TerminatorKind::SwitchInt {
        discr: Move(_33)
        targets: {
            values: [Pu128(0)]
            targets: [bb_second_compare, bb_cat_speak]
        }
    }
}

bb_cat_speak: {
    StatementKind::StorageDead(_35);
    StatementKind::StorageDead(_34);
    StatementKind::StorageLive(_36);
    StatementKind::StorageLive(_37);
    StatementKind::StorageLive(_38);
    StatementKind::Assign(Box<(_38, Rvalue::Use(Copy(_30)))>)
    StatementKind::Assign(Box<(_37, Rvalue::Cast(
        CastKind::Transmute,
        Move(_38),
        Ty: &'{erased} Cat
    ))>)
    StatementKind::Assign(Box<(_36, Rvalue::Ref(Region::ReErased, BorrowKind::Shared, _37))>)
    StatementKind::StorageDead(_38);
    StatementKind::StorageDead(_37);
    StatementKind::StorageLive(_39);
    StatementKind::StorageLive(_40);
    StatementKind::Assign(Box<(_40, Rvalue::Ref(Region::ReErased, BorrowKind::Shared, _36))>)
    TerminatorKind::Call {
        func: <Cat as Animal>::speak
        args: [Spanned { node: Move(_40), span: srclines }]
        destination: _39
        target: Some(bb_post_cat_goto)
        unwind: Cleanup(bb_unwind)
        call_source: Normal
        fn_span: srclines
    }
}

bb_post_cat_goto: {
    StatementKind::StorageDead(_40);
    StatementKind::StorageDead(_39);
    StatementKind::Assign(Box<(_0, Rvalue::Use(Constant( () ))>)
    StatementKind::StorageDead(_36);
    TerminatorKind::Goto(bb28)
}

// dog blocks are essentially the same

bb_goto: {
    StatementKind::StorageDead(_43);
    StatementKind::StorageDead(_42);
    StatementKind::Assign(Box<(_0, Rvalue::Use(Constant( () ))>)
    TerminatorKind::Goto(bb27)
}

bb_unwind (cleanup): {
    TerminatorKind::Drop {
        place: _20
        target: bb29
        unwind: Cleanup(bb34)
        replace: false
        drop: None
        async_fut: None
    }
}
```

