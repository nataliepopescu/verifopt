use super::*;
use crate::funcs::Funcs;
use crate::statement::RWStatement as RWS;
use crate::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence, Struct,
    TraitDecl, TraitImpl,
};
use crate::statement::{
    AssignmentRVal, FuncDecl, FuncVal, RWAssignmentRVal, RWFuncVal, Type,
};
use std::collections::HashSet;

#[test]
fn test_print() {
    let stmt = Print("hello");
    let check_stmt = RWS::Print("hello");

    let funcs = Funcs::new();
    let cmap = ConstraintMap::new();
    let sigs = Sigs::new();
    let traits = Traits::new();
    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    assert_eq!(check_stmt, ret.unwrap());
}

#[test]
fn test_assignment() {
    let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
    let check_stmt = RWS::Assignment("x", Box::new(RWAssignmentRVal::RVal(RVal::Num(5))));

    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    let sigs = Sigs::new();
    let traits = Traits::new();

    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    assert_eq!(check_stmt, ret.unwrap());
}

#[test]
fn test_direct_invoke() {
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            body.clone(),
        ))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(InvokeFunc("foo", vec![])),
    ]);

    let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "z",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "foo",
            false,
            vec![],
            None,
            check_body,
        ))),
        Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(RWS::InvokeFunc("foo", vec![])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );
    let mut cmap = ConstraintMap::new();
    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (HashSet::from([RVal::Var("foo")]), HashSet::new()),
        )),
    );
    let mut sigs = Sigs::new();
    let sigval = SigVal::new(vec![], None);
    sigs.sigs.insert(sigval, HashSet::from(["foo"]));

    let traits = Traits::new();
    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    assert_eq!(check_stmt, ret.unwrap());
}

#[test]
fn test_indirect_invoke_single_val() {
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let mut stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            body.clone(),
        ))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );
    let mut cmap = ConstraintMap::new();
    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (HashSet::from([RVal::Var("foo")]), HashSet::new()),
        )),
    );
    let mut sigs = Sigs::new();
    let sigval = SigVal::new(vec![], None);
    sigs.sigs.insert(sigval, HashSet::from(["foo"]));

    let traits = Traits::new();
    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);

    let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "z",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let switch_vec = vec![(RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![])))];
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "foo",
            false,
            vec![],
            None,
            check_body,
        ))),
        Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
    ]);

    assert_eq!(check_stmt, ret.unwrap());
}

#[test]
fn test_indirect_invoke_multiple_val() {
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(6))),
    ))]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            foo_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            None,
            bar_body.clone(),
        ))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
            )),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new("bar", false, vec![], None, bar_body.clone()),
        )],
    );
    let mut cmap = ConstraintMap::new();
    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("bar"), RVal::Var("foo")]),
                HashSet::new(),
            ),
        )),
    );
    let mut sigs = Sigs::new();
    let sigval = SigVal::new(vec![], None);
    sigs.sigs.insert(sigval, HashSet::from(["foo"]));

    let traits = Traits::new();
    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    let check_foo_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "z",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let check_bar_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "z",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(6))),
    ))]));
    let switch_vec = vec![
        (RVal::Var("bar"), Box::new(RWS::InvokeFunc("bar", vec![]))),
        (RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![]))),
    ];
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "foo",
            false,
            vec![],
            None,
            check_foo_body.clone(),
        ))),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "bar",
            false,
            vec![],
            None,
            check_bar_body.clone(),
        ))),
        Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("bar"))),
            )),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
    ]);

    assert_eq!(ret.unwrap(), check_stmt);
}

#[test]
fn test_nested_indirect_calls_no_args() {
    let baz_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(1))),
    ));
    let qux_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(2))),
    ));
    let baz2_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(3))),
    ));
    let qux2_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(4))),
    ));
    let foo_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "baz",
            false,
            vec![],
            None,
            baz_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "qux",
            false,
            vec![],
            None,
            qux_body.clone(),
        ))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
            )),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]));
    let bar_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "baz2",
            false,
            vec![],
            None,
            baz2_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "qux2",
            false,
            vec![],
            None,
            qux2_body.clone(),
        ))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
            )),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            foo_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            None,
            bar_body.clone(),
        ))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
            )),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new("bar", false, vec![], None, bar_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "baz",
        vec![(
            None,
            FuncVal::new("baz", false, vec![], None, baz_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "qux",
        vec![(
            None,
            FuncVal::new("qux", false, vec![], None, qux_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "baz2",
        vec![(
            None,
            FuncVal::new("baz2", false, vec![], None, baz2_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "qux2",
        vec![(
            None,
            FuncVal::new("qux2", false, vec![], None, qux2_body.clone()),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    let mut bar_cmap = ConstraintMap::new();
    let mut baz_cmap = ConstraintMap::new();
    let mut qux_cmap = ConstraintMap::new();
    let mut baz2_cmap = ConstraintMap::new();
    let mut qux2_cmap = ConstraintMap::new();

    baz_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );
    qux_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(2)]), HashSet::new()),
        )),
    );
    baz2_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    qux2_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(4)]), HashSet::new()),
        )),
    );

    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("baz"), RVal::Var("qux")]),
                HashSet::new(),
            ),
        )),
    );
    bar_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("baz2"), RVal::Var("qux2")]),
                HashSet::new(),
            ),
        )),
    );

    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                HashSet::new(),
            ),
        )),
    );
    cmap.cmap.insert(
        "baz2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![], None)), baz2_cmap)],
        )),
    );
    cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );
    cmap.cmap.insert(
        "qux",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![], None)), qux_cmap)],
        )),
    );
    cmap.cmap.insert(
        "baz",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![], None)), baz_cmap)],
        )),
    );
    cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
        )),
    );
    cmap.cmap.insert(
        "qux2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![], None)), qux2_cmap)],
        )),
    );
    let mut sigs = Sigs::new();
    let sigval = SigVal::new(vec![], None);
    sigs.sigs.insert(
        sigval,
        HashSet::from(["foo", "bar", "baz", "qux", "baz2", "qux2"]),
    );

    let traits = Traits::new();
    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    let check_baz_body = Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(1))),
    ));
    let check_qux_body = Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(2))),
    ));
    let check_baz2_body = Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(3))),
    ));
    let check_qux2_body = Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(4))),
    ));

    let foo_switch_vec = vec![
        (RVal::Var("baz"), Box::new(RWS::InvokeFunc("baz", vec![]))),
        (RVal::Var("qux"), Box::new(RWS::InvokeFunc("qux", vec![]))),
    ];
    let check_foo_body = Box::new(RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "baz",
            false,
            vec![],
            None,
            check_baz_body.clone(),
        ))),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "qux",
            false,
            vec![],
            None,
            check_qux_body.clone(),
        ))),
        Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("baz"))),
            )),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("qux"))),
            )),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), foo_switch_vec)),
    ]));
    let bar_switch_vec = vec![
        (RVal::Var("baz2"), Box::new(RWS::InvokeFunc("baz2", vec![]))),
        (RVal::Var("qux2"), Box::new(RWS::InvokeFunc("qux2", vec![]))),
    ];
    let check_bar_body = Box::new(RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "baz2",
            false,
            vec![],
            None,
            check_baz2_body.clone(),
        ))),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "qux2",
            false,
            vec![],
            None,
            check_qux2_body.clone(),
        ))),
        Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("baz2"))),
            )),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("qux2"))),
            )),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), bar_switch_vec)),
    ]));
    let switch_vec = vec![
        (RVal::Var("bar"), Box::new(RWS::InvokeFunc("bar", vec![]))),
        (RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![]))),
    ];
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "foo",
            false,
            vec![],
            None,
            check_foo_body.clone(),
        ))),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "bar",
            false,
            vec![],
            None,
            check_bar_body.clone(),
        ))),
        Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
            )),
            Box::new(RWS::Assignment(
                "x",
                Box::new(RWAssignmentRVal::RVal(RVal::Var("bar"))),
            )),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
    ]);

    assert_eq!(ret.unwrap(), check_stmt);
}

#[test]
fn test_dyn_traits_two_impl() {
    let funcdecl = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );

    let cat_speak_body = Box::new(Print("meow"));
    let cat_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let dog_speak_body = Box::new(Print("woof"));
    let dog_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        dog_speak_body.clone(),
    );

    let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
            "Cat",
            vec![],
            vec![],
        )))])),
        Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
            "Dog",
            vec![],
            vec![],
        )))])),
    ))]));

    let stmt = Sequence(vec![
        Box::new(TraitDecl("Animal", vec![funcdecl.clone()])),
        Box::new(FuncDef(FuncVal::new(
            "giveMeAnAnimal",
            false,
            vec![],
            Some(Box::new(Type::DynTrait("Animal"))),
            gmaa.clone(),
        ))),
        Box::new(Struct("Cat", vec![], vec![])),
        Box::new(Struct("Dog", vec![], vec![])),
        Box::new(TraitImpl("Animal", "Cat", vec![cat_speak.clone()])),
        Box::new(TraitImpl("Animal", "Dog", vec![dog_speak.clone()])),
        Box::new(Assignment(
            "specific_animal",
            Box::new(AssignmentRVal::Statement(Box::new(Statement::InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(InvokeFunc("speak", vec!["specific_animal"])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "speak",
        vec![
            (Some(("Animal", "Dog")), dog_speak.clone()),
            (Some(("Animal", "Cat")), cat_speak.clone()),
        ],
    );
    funcs.funcs.insert(
        "giveMeAnAnimal",
        vec![(
            None,
            FuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut speak_cmap = ConstraintMap::new();
    speak_cmap.cmap.insert(
        "animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (
                HashSet::from([
                    RVal::Struct("Cat", vec![], vec![]),
                    RVal::Struct("Dog", vec![], vec![]),
                ]),
                HashSet::new(),
            ),
        )),
    );
    cmap.cmap.insert(
        "speak",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![Type::DynTrait("Animal")], None)),
                speak_cmap,
            )],
        )),
    );
    cmap.cmap.insert(
        "giveMeAnAnimal",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::DynTrait("Animal"))))),
                ConstraintMap::new(),
            )],
        )),
    );
    cmap.cmap.insert(
        "specific_animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (
                HashSet::from([
                    RVal::Struct("Cat", vec![], vec![]),
                    RVal::Struct("Dog", vec![], vec![]),
                ]),
                HashSet::new(),
            ),
        )),
    );

    let mut sigs = Sigs::new();
    sigs.sigs
        .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));
    sigs.sigs.insert(
        SigVal::new(vec![], Some(Box::new(Type::DynTrait("Animal")))),
        HashSet::from(["giveMeAnAnimal"]),
    );

    let mut traits = Traits::new();
    traits.traits.insert("Animal", vec!["Cat", "Dog"]);

    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    let check_cat_speak_body = Box::new(RWS::Print("meow"));
    let check_cat_speak = RWFuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        check_cat_speak_body.clone(),
    );

    let check_dog_speak_body = Box::new(RWS::Print("woof"));
    let check_dog_speak = RWFuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        check_dog_speak_body.clone(),
    );

    let check_gmaa = Box::new(RWS::Sequence(vec![Box::new(RWS::Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
            "Cat",
            vec![],
            vec![],
        )))])),
        Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
            "Dog",
            vec![],
            vec![],
        )))])),
    ))]));

    let switch_vec = vec![
        (
            RVal::Var("Cat"),
            Box::new(RWS::InvokeTraitFunc(
                "speak",
                ("Animal", "Cat"),
                vec!["specific_animal"],
            )),
        ),
        (
            RVal::Var("Dog"),
            Box::new(RWS::InvokeTraitFunc(
                "speak",
                ("Animal", "Dog"),
                vec!["specific_animal"],
            )),
        ),
    ];

    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::TraitDecl("Animal", vec![funcdecl.clone()])),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "giveMeAnAnimal",
            false,
            vec![],
            Some(Box::new(Type::DynTrait("Animal"))),
            check_gmaa,
        ))),
        Box::new(RWS::Struct("Cat", vec![], vec![])),
        Box::new(RWS::Struct("Dog", vec![], vec![])),
        Box::new(RWS::TraitImpl(
            "Animal",
            "Cat",
            vec![check_cat_speak.clone()],
        )),
        Box::new(RWS::TraitImpl(
            "Animal",
            "Dog",
            vec![check_dog_speak.clone()],
        )),
        Box::new(RWS::Assignment(
            "specific_animal",
            Box::new(RWAssignmentRVal::Statement(Box::new(RWS::InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(RWS::TraitSwitch(RVal::Var("specific_animal"), switch_vec)),
    ]);

    assert_eq!(ret.unwrap(), check_stmt);
}

#[test]
fn test_dyn_traits_three_impl_two_used() {
    let funcdecl = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );

    let bird_speak_body = Box::new(Print("chirp"));
    let bird_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Bird"))],
        None,
        bird_speak_body.clone(),
    );

    let cat_speak_body = Box::new(Print("meow"));
    let cat_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let dog_speak_body = Box::new(Print("woof"));
    let dog_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        dog_speak_body.clone(),
    );

    let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
            "Cat",
            vec![],
            vec![],
        )))])),
        Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
            "Dog",
            vec![],
            vec![],
        )))])),
    ))]));

    let stmt = Sequence(vec![
        Box::new(TraitDecl("Animal", vec![funcdecl.clone()])),
        Box::new(FuncDef(FuncVal::new(
            "giveMeAnAnimal",
            false,
            vec![],
            Some(Box::new(Type::DynTrait("Animal"))),
            gmaa.clone(),
        ))),
        Box::new(Struct("Bird", vec![], vec![])),
        Box::new(Struct("Cat", vec![], vec![])),
        Box::new(Struct("Dog", vec![], vec![])),
        Box::new(TraitImpl("Animal", "Bird", vec![bird_speak.clone()])),
        Box::new(TraitImpl("Animal", "Cat", vec![cat_speak.clone()])),
        Box::new(TraitImpl("Animal", "Dog", vec![dog_speak.clone()])),
        Box::new(Assignment(
            "specific_animal",
            Box::new(AssignmentRVal::Statement(Box::new(Statement::InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(InvokeFunc("speak", vec!["specific_animal"])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "speak",
        vec![
            (Some(("Animal", "Dog")), dog_speak.clone()),
            (Some(("Animal", "Cat")), cat_speak.clone()),
            (Some(("Animal", "Bird")), bird_speak.clone()),
        ],
    );
    funcs.funcs.insert(
        "giveMeAnAnimal",
        vec![(
            None,
            FuncVal::new(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut cat_speak_cmap = ConstraintMap::new();
    let mut dog_speak_cmap = ConstraintMap::new();

    dog_speak_cmap.cmap.insert(
        "self",
        Box::new(VarType::Values(
            Box::new(Type::Struct("Dog")),
            (HashSet::from([RVal::IdkStruct("Dog")]), HashSet::new()),
        )),
    );
    dog_speak_cmap.cmap.insert(
        "spoken",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(2)]), HashSet::new()),
        )),
    );

    cat_speak_cmap.cmap.insert(
        "self",
        Box::new(VarType::Values(
            Box::new(Type::Struct("Cat")),
            (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
        )),
    );
    cat_speak_cmap.cmap.insert(
        "spoken",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );

    cmap.cmap.insert(
        "speak",
        Box::new(VarType::Scope(
            None,
            vec![
                (
                    Box::new(Type::Func(vec![Type::Struct("Dog")], None)),
                    dog_speak_cmap,
                ),
                (
                    Box::new(Type::Func(vec![Type::Struct("Cat")], None)),
                    cat_speak_cmap,
                ),
            ],
        )),
    );
    cmap.cmap.insert(
        "giveMeAnAnimal",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::DynTrait("Animal"))))),
                ConstraintMap::new(),
            )],
        )),
    );
    cmap.cmap.insert(
        "specific_animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (
                HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                HashSet::new(),
            ),
        )),
    );

    let mut sigs = Sigs::new();
    sigs.sigs
        .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));
    sigs.sigs.insert(
        SigVal::new(vec![], Some(Box::new(Type::DynTrait("Animal")))),
        HashSet::from(["giveMeAnAnimal"]),
    );

    let mut traits = Traits::new();
    traits.traits.insert("Animal", vec!["Bird", "Cat", "Dog"]);

    let rw = Rewriter::new();
    let ret = rw.rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true);

    let check_bird_speak_body = Box::new(RWS::Print("chirp"));
    let check_bird_speak = RWFuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Bird"))],
        None,
        check_bird_speak_body.clone(),
    );

    let check_cat_speak_body = Box::new(RWS::Print("meow"));
    let check_cat_speak = RWFuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        check_cat_speak_body.clone(),
    );

    let check_dog_speak_body = Box::new(RWS::Print("woof"));
    let check_dog_speak = RWFuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        check_dog_speak_body.clone(),
    );

    let check_gmaa = Box::new(RWS::Sequence(vec![Box::new(RWS::Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
            "Cat",
            vec![],
            vec![],
        )))])),
        Box::new(RWS::Sequence(vec![Box::new(RWS::Return(RVal::Struct(
            "Dog",
            vec![],
            vec![],
        )))])),
    ))]));

    let switch_vec = vec![
        (
            RVal::Var("Dog"),
            Box::new(RWS::InvokeTraitFunc(
                "speak",
                ("Animal", "Dog"),
                vec!["specific_animal"],
            )),
        ),
        (
            RVal::Var("Cat"),
            Box::new(RWS::InvokeTraitFunc(
                "speak",
                ("Animal", "Cat"),
                vec!["specific_animal"],
            )),
        ),
    ];

    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::TraitDecl("Animal", vec![funcdecl.clone()])),
        Box::new(RWS::FuncDef(RWFuncVal::new(
            "giveMeAnAnimal",
            false,
            vec![],
            Some(Box::new(Type::DynTrait("Animal"))),
            check_gmaa,
        ))),
        Box::new(RWS::Struct("Bird", vec![], vec![])),
        Box::new(RWS::Struct("Cat", vec![], vec![])),
        Box::new(RWS::Struct("Dog", vec![], vec![])),
        Box::new(RWS::TraitImpl(
            "Animal",
            "Bird",
            vec![check_bird_speak.clone()],
        )),
        Box::new(RWS::TraitImpl(
            "Animal",
            "Cat",
            vec![check_cat_speak.clone()],
        )),
        Box::new(RWS::TraitImpl(
            "Animal",
            "Dog",
            vec![check_dog_speak.clone()],
        )),
        Box::new(RWS::Assignment(
            "specific_animal",
            Box::new(RWAssignmentRVal::Statement(Box::new(RWS::InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(RWS::TraitSwitch(RVal::Var("specific_animal"), switch_vec)),
    ]);

    assert_eq!(ret.unwrap(), check_stmt);
}
