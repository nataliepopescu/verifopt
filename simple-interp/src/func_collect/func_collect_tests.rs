use super::*;
use crate::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence, Struct,
    TraitDecl, TraitImpl,
};
use crate::statement::{AssignmentRVal, BStatement, FuncDecl, FuncVal, RVal};

#[test]
fn test_print() {
    let fc = FuncCollector::new();
    let stmt = Print("hello");
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_assign_num() {
    let fc = FuncCollector::new();
    let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_seq() {
    let fc = FuncCollector::new();
    let stmt_vec = vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))];
    let stmt = Sequence(stmt_vec);
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_nested_seq() {
    let fc = FuncCollector::new();
    let stmt = Sequence(vec![
        Box::new(Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))])),
        Box::new(Sequence(vec![Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        ))])),
    ]);
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_var_undef() {
    let fc = FuncCollector::new();
    let stmt = Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ))]);
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_assign_var() {
    let fc = FuncCollector::new();
    let stmt = Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
        )),
    ]);
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let check_funcs = Funcs::new();
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_funcdef() {
    let fc = FuncCollector::new();
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = FuncDef(FuncVal::new("foo", false, vec![], None, body.clone()));
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body))],
    );
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_funcdef_args_do_nothing() {
    let fc = FuncCollector::new();
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = FuncDef(FuncVal::new(
        "foo",
        false,
        vec![("y", Type::Int())],
        None,
        body.clone(),
    ));
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![("y", Type::Int())], None, body),
        )],
    );
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_assign_funcptr() {
    let fc = FuncCollector::new();
    let body = Box::new(Assignment(
        "y",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
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
    ]);
    let mut funcs = Funcs::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );
    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
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

    let mut funcs = Funcs::new();
    let fc = FuncCollector::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "bar",
        vec![(None, FuncVal::new("bar", false, vec![], None, bar_body))],
    );
    check_funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, foo_body))],
    );

    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
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
    let fc = FuncCollector::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );
    check_funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new("bar", false, vec![], None, bar_body.clone()),
        )],
    );
    check_funcs.funcs.insert(
        "baz",
        vec![(
            None,
            FuncVal::new("baz", false, vec![], None, baz_body.clone()),
        )],
    );
    check_funcs.funcs.insert(
        "qux",
        vec![(
            None,
            FuncVal::new("qux", false, vec![], None, qux_body.clone()),
        )],
    );
    check_funcs.funcs.insert(
        "baz2",
        vec![(
            None,
            FuncVal::new("baz2", false, vec![], None, baz2_body.clone()),
        )],
    );
    check_funcs.funcs.insert(
        "qux2",
        vec![(
            None,
            FuncVal::new("qux2", false, vec![], None, qux2_body.clone()),
        )],
    );

    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}

#[test]
fn test_trait_impl() {
    let cat_speak_body = Box::new(Sequence(vec![Box::new(Print("meow"))]));

    let funcdef = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );
    let cat_funcimpl = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let stmt = Sequence(vec![
        Box::new(TraitDecl("Animal", vec![funcdef.clone()])),
        Box::new(Struct("Cat", vec![], vec![])),
        Box::new(TraitImpl("Animal", "Cat", vec![cat_funcimpl.clone()])),
        Box::new(Assignment(
            "edgar",
            Box::new(AssignmentRVal::RVal(RVal::Struct("Cat", vec![], vec![]))),
        )),
    ]);

    let mut funcs = Funcs::new();
    let fc = FuncCollector::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "speak",
        vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
    );

    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
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
    let fc = FuncCollector::new();
    let res = fc.collect(&mut funcs, &stmt);

    let mut check_funcs = Funcs::new();
    check_funcs.funcs.insert(
        "speak",
        vec![
            (Some(("Animal", "Cat")), cat_speak.clone()),
            (Some(("Animal", "Dog")), dog_speak.clone()),
        ],
    );
    check_funcs.funcs.insert(
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

    assert_eq!(res.unwrap(), ());
    assert_eq!(funcs, check_funcs);
}
