use simple_interp::SimpleInterp;
use simple_interp::statement::RWStatement as RWS;
use simple_interp::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, Print, Return,
    Sequence, Struct, TraitDecl, TraitImpl,
};
use simple_interp::statement::{
    RWAssignmentRVal, AssignmentRVal, BStatement, FuncDecl, RWFuncVal, FuncVal, RVal, Type,
};

#[test]
fn test_print() {
    let stmt = Print("hello");
    let check_stmt = RWS::Print("hello");

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
}

#[test]
fn test_funcdef() {
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = FuncDef(FuncVal::new("foo", false, vec![], None, body.clone()));

    let check_body = Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ));
    let check_stmt = RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_body.clone()));

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
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

    let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_body))),
        Box::new(RWS::InvokeFunc("foo", vec![])),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
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

    let check_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "z",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let switch_vec = vec![(RVal::Var("foo"), Box::new(RWS::InvokeFunc("foo", vec![])))];
    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_body))),
        Box::new(RWS::Assignment(
            "x",
            Box::new(RWAssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(RWS::Switch(RVal::Var("x"), switch_vec)),
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

    let check_foo_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let check_bar_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "x",
        Box::new(RWAssignmentRVal::RVal(RVal::Num(6))),
    ))]));

    let check_stmt = RWS::Sequence(vec![
        Box::new(RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_foo_body))),
        Box::new(RWS::FuncDef(RWFuncVal::new("bar", false, vec![], None, check_bar_body))),
        Box::new(RWS::Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(RWS::InvokeFunc("foo", vec![])),
            Box::new(RWS::InvokeFunc("bar", vec![])),
        )),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
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

    let check_foo_body = Box::new(RWS::Sequence(vec![Box::new(RWS::Assignment(
        "y",
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
        Box::new(RWS::FuncDef(RWFuncVal::new("foo", false, vec![], None, check_foo_body))),
        Box::new(RWS::FuncDef(RWFuncVal::new("bar", false, vec![], None, check_bar_body))),
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
        Box::new(RWS::Struct("Bird", vec![], vec![])),
        Box::new(RWS::Struct("Cat", vec![], vec![])),
        Box::new(RWS::Struct("Dog", vec![], vec![])),
        Box::new(RWS::TraitImpl("Animal", "Bird", vec![check_bird_speak.clone()])),
        Box::new(RWS::TraitImpl("Animal", "Cat", vec![check_cat_speak.clone()])),
        Box::new(RWS::TraitImpl("Animal", "Dog", vec![check_dog_speak.clone()])),
        Box::new(RWS::Assignment(
            "specific_animal",
            Box::new(RWAssignmentRVal::Statement(Box::new(RWS::InvokeFunc(
                "giveMeAnAnimal",
                vec![],
            )))),
        )),
        Box::new(RWS::TraitSwitch(RVal::Var("specific_animal"), switch_vec)),
    ]);

    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();

    assert_eq!(rw_stmt, check_stmt);
}
