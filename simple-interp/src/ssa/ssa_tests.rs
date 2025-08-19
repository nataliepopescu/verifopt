use super::*;
use crate::error::Error;
use crate::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, Print, Sequence, Struct, Switch,
    TraitDecl, TraitImpl,
};
use crate::statement::{AssignmentRVal, BStatement, FuncDecl, FuncVal, Type};

#[test]
fn test_merge_vars() {
    let mut symb1 = Symbols::new();
    symb1.0.insert("x", None);
    let mut symb2 = Symbols::new();
    symb2.0.insert("x", None);
    let vec: Vec<Symbols> = vec![symb1, symb2];

    let mut end_symb = Symbols::new();
    end_symb.0.insert("x", None);
    assert_eq!(end_symb, vec.merge().unwrap().unwrap());
}

#[test]
fn test_merge_same_funcs() {
    let mut body = Symbols::new();
    body.0.insert("x", None);
    let mut symb1 = Symbols::new();
    symb1.0.insert("foo", Some(Box::new(body.clone())));
    let mut symb2 = Symbols::new();
    symb2.0.insert("foo", Some(Box::new(body.clone())));
    let vec: Vec<Symbols> = vec![symb1, symb2];

    let mut end_symb = Symbols::new();
    end_symb.0.insert("foo", Some(Box::new(body)));
    assert_eq!(end_symb, vec.merge().unwrap().unwrap());
}

#[test]
fn test_merge_err() {
    let mut body = Symbols::new();
    body.0.insert("x", None);
    let mut symb1 = Symbols::new();
    symb1.0.insert("foo", Some(Box::new(body.clone())));
    let mut symb2 = Symbols::new();
    symb2.0.insert("foo", None);
    let vec: Vec<Symbols> = vec![symb1, symb2];

    assert!(vec.merge().is_err());
}

#[test]
fn test_assignment_err() {
    let stmt = Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    ]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    assert_eq!(res, Err(Error::SymbolAlreadyExists("x")));
}

#[test]
fn test_funcdef_args_do_nothing() {
    let body = Box::new(Assignment(
        "y",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = Sequence(vec![Box::new(FuncDef(FuncVal::new(
        "foo",
        false,
        vec![("x", Type::Int())],
        None,
        body.clone(),
    )))]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let _ = sc.check(&mut symbols, &stmt);

    let mut check_symbols = Symbols::new();
    let mut inner_check_symbols = Symbols::new();
    inner_check_symbols.0.insert("x", None);
    inner_check_symbols.0.insert("y", None);
    check_symbols
        .0
        .insert("foo", Some(Box::new(inner_check_symbols)));

    assert_eq!(check_symbols, symbols);
}

#[test]
fn test_funcdef_err() {
    let body = Box::new(Assignment(
        "x",
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
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            body.clone(),
        ))),
    ]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    assert_eq!(res, Err(Error::SymbolAlreadyExists("foo")));
}

#[test]
fn test_nested_funcdefs() {
    let body = Box::new(FuncDef(FuncVal::new(
        "baz",
        false,
        vec![],
        None,
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
    )));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            None,
            body.clone(),
        ))),
    ]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let _ = sc.check(&mut symbols, &stmt);

    let mut check_symbols = Symbols::new();
    let mut body_check_symbols = Symbols::new();
    let mut baz_check_symbols = Symbols::new();
    baz_check_symbols.0.insert("x", None);
    body_check_symbols
        .0
        .insert("baz", Some(Box::new(baz_check_symbols)));
    check_symbols
        .0
        .insert("foo", Some(Box::new(body_check_symbols.clone())));
    check_symbols
        .0
        .insert("bar", Some(Box::new(body_check_symbols.clone())));

    assert_eq!(symbols, check_symbols);
}

#[test]
fn test_conditional_ok() {
    let stmt = Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    );

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    assert_eq!(res.unwrap(), ());
}

#[test]
fn test_switch_ok() {
    let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
        (
            RVal::Num(4),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(0))),
            )),
        ),
        (
            RVal::Num(5),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
        ),
    ];
    let stmt = Sequence(vec![
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(4))),
            )),
        )),
        Box::new(Switch(RVal::Var("x"), switch_vec)),
    ]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    assert_eq!(res.unwrap(), ());
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

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    let mut check_symbols = Symbols::new();
    let mut foo_symbols = Symbols::new();
    let mut bar_symbols = Symbols::new();
    let mut baz_symbols = Symbols::new();
    let mut qux_symbols = Symbols::new();
    let mut baz2_symbols = Symbols::new();
    let mut qux2_symbols = Symbols::new();

    baz_symbols.0.insert("x", None);
    qux_symbols.0.insert("x", None);
    baz2_symbols.0.insert("x", None);
    qux2_symbols.0.insert("x", None);

    foo_symbols.0.insert("baz", Some(Box::new(baz_symbols)));
    foo_symbols.0.insert("qux", Some(Box::new(qux_symbols)));
    foo_symbols.0.insert("x", None);

    bar_symbols.0.insert("baz2", Some(Box::new(baz2_symbols)));
    bar_symbols.0.insert("qux2", Some(Box::new(qux2_symbols)));
    bar_symbols.0.insert("x", None);

    check_symbols.0.insert("foo", Some(Box::new(foo_symbols)));
    check_symbols.0.insert("bar", Some(Box::new(bar_symbols)));
    check_symbols.0.insert("x", None);

    assert_eq!(res.unwrap(), ());
    assert_eq!(check_symbols, symbols);
}

#[test]
fn test_dyn_traits_two_impl() {
    let funcdef = FuncDecl::new(
        "speak",
        true,
        vec![("animal", Type::DynTrait("Animal"))],
        None,
    );

    let cat_speak_body = Box::new(Print("meow"));
    let cat_speak = FuncVal::new(
        "speak",
        true,
        vec![("animal", Type::DynTrait("Animal"))],
        None,
        cat_speak_body.clone(),
    );

    let dog_speak_body = Box::new(Print("woof"));
    let dog_speak = FuncVal::new(
        "speak",
        true,
        vec![("animal", Type::DynTrait("Animal"))],
        None,
        dog_speak_body.clone(),
    );

    let stmt = Sequence(vec![
        Box::new(TraitDecl("Animal", vec![funcdef.clone()])),
        Box::new(Struct("Cat", vec![], vec![])),
        Box::new(Struct("Dog", vec![], vec![])),
        Box::new(TraitImpl("Animal", "Cat", vec![cat_speak.clone()])),
        Box::new(TraitImpl("Animal", "Dog", vec![dog_speak.clone()])),
        Box::new(Assignment(
            "cat",
            Box::new(AssignmentRVal::RVal(RVal::Struct("Cat", vec![], vec![]))),
        )),
        Box::new(InvokeFunc("speak", vec!["cat"])),
    ]);

    let sc = SSAChecker::new();
    let mut symbols = Symbols::new();
    let res = sc.check(&mut symbols, &stmt);

    let mut check_symbols = Symbols::new();
    check_symbols.0.insert("Cat", None);
    check_symbols.0.insert("Dog", None);
    check_symbols.0.insert("cat", None);

    assert_eq!(res.unwrap(), ());
    assert_eq!(check_symbols, symbols);
}
