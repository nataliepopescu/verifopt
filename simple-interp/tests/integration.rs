use simple_interp::SimpleInterp;
use simple_interp::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, InvokeTraitFunc, Print, Return,
    Sequence, Struct, Switch, TraitDecl, TraitImpl,
};
use simple_interp::statement::{
    AssignmentRVal, BStatement, FuncDecl, FuncVal, RVal, Type,
};

#[test]
fn test_print() {
    let stmt = Print("hello");

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt.clone()).unwrap();

    assert_eq!(rw_stmt, stmt);
}

#[test]
fn test_funcdef() {
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = FuncDef(FuncVal::new("foo", false, vec![], None, body.clone()));

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt.clone()).unwrap();

    assert_eq!(rw_stmt, stmt);
}

#[test]
fn test_direct_invoke() {
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, body))),
        Box::new(InvokeFunc("foo", vec![])),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt.clone()).unwrap();

    assert_eq!(rw_stmt, stmt);
}

#[test]
fn test_indirect_invoke() {
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
        Box::new(InvokeFunc("x", vec![])),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    let switch_vec = vec![(RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![])))];
    let check_stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, body))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(Switch(RVal::Var("x"), switch_vec)),
    ]);

    assert_eq!(rw_stmt, check_stmt);
}

#[test]
fn test_direct_invoke_uncertain() {
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(6))),
    ))]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, foo_body))),
        Box::new(FuncDef(FuncVal::new("bar", false, vec![], None, bar_body))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(InvokeFunc("foo", vec![])),
            Box::new(InvokeFunc("bar", vec![])),
        )),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt.clone()).unwrap();

    assert_eq!(rw_stmt, stmt);
}

#[test]
fn test_indirect_invoke_uncertain() {
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "y",
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

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    let switch_vec = vec![
        (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
        (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
    ];
    let check_stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, foo_body))),
        Box::new(FuncDef(FuncVal::new("bar", false, vec![], None, bar_body))),
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
        Box::new(Switch(RVal::Var("x"), switch_vec)),
    ]);

    assert_eq!(rw_stmt, check_stmt);
}

#[test]
fn test_nested_direct_calls_no_args() {
    let bar_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let foo_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            None,
            bar_body.clone(),
        ))),
        Box::new(InvokeFunc("bar", vec![])),
    ]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            foo_body.clone(),
        ))),
        Box::new(InvokeFunc("foo", vec![])),
    ]);
    let check_stmt = stmt.clone();

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
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

    let foo_switch_vec = vec![
        (RVal::Var("baz"), Box::new(InvokeFunc("baz", vec![]))),
        (RVal::Var("qux"), Box::new(InvokeFunc("qux", vec![]))),
    ];
    let check_foo_body = Box::new(Sequence(vec![
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
        Box::new(Switch(RVal::Var("x"), foo_switch_vec)),
    ]));
    let bar_switch_vec = vec![
        (RVal::Var("baz2"), Box::new(InvokeFunc("baz2", vec![]))),
        (RVal::Var("qux2"), Box::new(InvokeFunc("qux2", vec![]))),
    ];
    let check_bar_body = Box::new(Sequence(vec![
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
        Box::new(Switch(RVal::Var("x"), bar_switch_vec)),
    ]));
    let switch_vec = vec![
        (RVal::Var("bar"), Box::new(InvokeFunc("bar", vec![]))),
        (RVal::Var("foo"), Box::new(InvokeFunc("foo", vec![]))),
    ];
    let check_stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            check_foo_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            None,
            check_bar_body.clone(),
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
        Box::new(Switch(RVal::Var("x"), switch_vec)),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
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
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(InvokeFunc("speak", vec!["specific_animal"])),
    ]);

    let switch_vec = vec![
        (
            RVal::Var("Cat"),
            Box::new(InvokeTraitFunc(
                "speak",
                ("Animal", "Cat"),
                vec!["specific_animal"],
            )),
        ),
        (
            RVal::Var("Dog"),
            Box::new(InvokeTraitFunc(
                "speak",
                ("Animal", "Dog"),
                vec!["specific_animal"],
            )),
        ),
    ];

    let check_stmt = Sequence(vec![
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
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(Switch(RVal::Var("specific_animal"), switch_vec)),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
}
