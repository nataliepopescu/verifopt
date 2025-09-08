# MIR pass low-level details/construction

## post-rewrite MIR goal (from transmute_cp rewritten source code)

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

simplified dumped MIR:

```
bb_transmute: {
    StorageLive(_22);
    StorageLive(_23);
    StorageLive(_24);
    _24 = &_17;
    _23 = &(*_24);
    _22 = transmute_copy::<Box<dyn Animal>, (*const u8, *const usize)>(move _23) -> [return: bb_get_vtable_ptrs, unwind: bb_unwind];
}

bb_get_vtable_ptrs {}

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

