use super::*;
use crate::funcs::Funcs;
use crate::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence, Struct,
    Switch, TraitDecl, TraitImpl,
};
use crate::statement::{FuncDecl, FuncVal, Type};

#[test]
fn test_merge_none() {
    let vec: Vec<ConstraintMap> = Vec::new();
    assert_eq!(vec.merge(), Ok(None));
}

#[test]
fn test_merge_one() {
    let mut cmap = ConstraintMap::new();
    cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    let vec: Vec<ConstraintMap> = vec![cmap];
    assert_eq!(vec[0].clone(), vec.merge().unwrap().unwrap());
}

#[test]
fn test_merge_two() {
    let mut cmap1 = ConstraintMap::new();
    cmap1.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    let mut cmap2 = ConstraintMap::new();
    cmap2.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );
    let vec: Vec<ConstraintMap> = vec![cmap1, cmap2];

    let mut end_cmap = ConstraintMap::new();
    end_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
        )),
    );
    assert_eq!(end_cmap, vec.merge().unwrap().unwrap());
}

#[test]
fn test_print() {
    let interp = Interpreter::new();
    let stmt = Print("hello");
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, ConstraintMap::new());
}

#[test]
fn test_assign_num() {
    let interp = Interpreter::new();
    let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_seq() {
    let interp = Interpreter::new();
    let stmt_vec = vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))];
    let stmt = Sequence(stmt_vec);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_seq_assign() {
    let interp = Interpreter::new();
    let stmt_vec = vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    ];
    let stmt = Sequence(stmt_vec);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_nested_seq() {
    let interp = Interpreter::new();
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
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_var_undef() {
    let interp = Interpreter::new();
    let stmt = Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ))]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    assert_eq!(res.err(), Some(Error::UndefinedSymbol("y")));
}

#[test]
fn test_assign_var() {
    let interp = Interpreter::new();
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
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_true() {
    let interp = Interpreter::new();
    let stmt = Conditional(
        Box::new(BStatement::True()),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    );
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_false() {
    let interp = Interpreter::new();
    let stmt = Conditional(
        Box::new(BStatement::False()),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    );
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_uncertain() {
    let interp = Interpreter::new();
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
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_not() {
    let interp = Interpreter::new();
    let stmt = Conditional(
        Box::new(BStatement::Not(Box::new(BStatement::True()))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    );
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_equals_num() {
    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(2))),
            )),
        )),
    ]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_equals_func() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
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
            FuncVal::new("bar", false, vec![], None, foo_body.clone()),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    // note: `equals` is _shallow_, which is why it evals to false here
    let interp = Interpreter::new();
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
            foo_body.clone(),
        ))),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("bar"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(2))),
            )),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(2)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_equals_func_ref() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            None,
            foo_body.clone(),
        ))),
        Box::new(Assignment(
            "bar",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("bar"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(2))),
            )),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let foo_type = Type::Func(vec![], None);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Values(
            Box::new(foo_type),
            (HashSet::from([RVal::Var("foo")]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_equals_uncertain() {
    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(4))),
            )),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(2))),
            )),
        )),
    ]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3), RVal::Num(4)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1), RVal::Num(2)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_equals_err() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, foo_body))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("foo"), RVal::Var("x"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(2))),
            )),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    assert_eq!(
        res,
        Err(Error::IncomparableTypes(RVal::Var("foo"), RVal::Num(5)))
    );
}

#[test]
fn test_nested_conditional() {
    let interp = Interpreter::new();
    let stmt = Conditional(
        Box::new(BStatement::True()),
        Box::new(Conditional(
            Box::new(BStatement::True()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        )),
        Box::new(Conditional(
            Box::new(BStatement::True()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(7))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(8))),
            )),
        )),
    );
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_conditional_scope() {
    let interp = Interpreter::new();
    let stmt = Sequence(vec![Box::new(Conditional(
        Box::new(BStatement::True()),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(6))),
        )),
    ))]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_funcdef() {
    let mut funcs = Funcs::new();
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = FuncDef(FuncVal::new("foo", false, vec![], None, body));
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, ConstraintMap::new());
}

#[test]
fn test_direct_invoke() {
    let mut funcs = Funcs::new();
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, body))),
        Box::new(InvokeFunc("foo", vec![])),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_funcdef_args_direct() {
    let mut funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();

    let body = Box::new(Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(0))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
        )),
    ]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            body.clone(),
        ))),
        Box::new(Assignment(
            "arg",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(InvokeFunc("foo", vec!["arg"])),
    ]);

    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![("y", Type::Int())], None, body.clone()),
        )],
    );
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "arg",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(0)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_indirect_invoke_simple() {
    let mut funcs = Funcs::new();
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, body.clone()))],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, body))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let foo_type = Type::Func(vec![], None);
    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(foo_type),
            (HashSet::from([RVal::Var("foo")]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_funcdef_args_indirect() {
    let mut funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();

    let body = Box::new(Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(0))),
            )),
            Box::new(Assignment(
                "z",
                Box::new(AssignmentRVal::RVal(RVal::Num(1))),
            )),
        )),
    ]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            body.clone(),
        ))),
        Box::new(Assignment(
            "w",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(Assignment(
            "arg",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(InvokeFunc("w", vec!["arg"])),
    ]);

    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![("y", Type::Int())], None, body.clone()),
        )],
    );
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let foo_type = Type::Func(vec![Type::Int()], None);
    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(0)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "arg",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "w",
        Box::new(VarType::Values(
            Box::new(foo_type),
            (HashSet::from([RVal::Var("foo")]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
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
    funcs.funcs.insert(
        "bar",
        vec![(None, FuncVal::new("bar", false, vec![], None, bar_body))],
    );
    funcs.funcs.insert(
        "foo",
        vec![(None, FuncVal::new("foo", false, vec![], None, foo_body))],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let foo_cmap = ConstraintMap::new();
    let mut bar_cmap = ConstraintMap::new();
    bar_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
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
            FuncVal::new("quz", false, vec![], None, qux_body.clone()),
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
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
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

    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                HashSet::new(),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "baz2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![], None)), baz2_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "qux",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![], None)), qux_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "baz",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![], None)), baz_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "qux2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![], None)), qux2_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_nested_direct_calls_with_args() {
    let bar_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ));
    let foo_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![("y", Type::Int())],
            None,
            bar_body.clone(),
        ))),
        Box::new(InvokeFunc("bar", vec!["y"])),
    ]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            foo_body.clone(),
        ))),
        Box::new(Assignment(
            "arg",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(InvokeFunc("foo", vec!["arg"])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new("bar", false, vec![("y", Type::Int())], None, bar_body),
        )],
    );
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![("y", Type::Int())], None, foo_body),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    let mut bar_cmap = ConstraintMap::new();
    bar_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    bar_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    foo_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "arg",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), bar_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_nested_indirect_calls_with_args() {
    let baz_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ));
    let qux_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ));
    let baz2_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ));
    let qux2_body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
    ));
    let foo_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "baz",
            false,
            vec![("y", Type::Int())],
            None,
            baz_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "qux",
            false,
            vec![("y", Type::Int())],
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
        Box::new(InvokeFunc("x", vec!["y"])),
    ]));
    let bar_body = Box::new(Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "baz2",
            false,
            vec![("y", Type::Int())],
            None,
            baz2_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "qux2",
            false,
            vec![("y", Type::Int())],
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
        Box::new(InvokeFunc("x", vec!["y"])),
    ]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            foo_body.clone(),
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![("y", Type::Int())],
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
        Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(InvokeFunc("x", vec!["y"])),
    ]);

    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new(
                "foo",
                false,
                vec![("y", Type::Int())],
                None,
                foo_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new(
                "bar",
                false,
                vec![("y", Type::Int())],
                None,
                bar_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "baz",
        vec![(
            None,
            FuncVal::new(
                "baz",
                false,
                vec![("y", Type::Int())],
                None,
                baz_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "qux",
        vec![(
            None,
            FuncVal::new(
                "qux",
                false,
                vec![("y", Type::Int())],
                None,
                qux_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "baz2",
        vec![(
            None,
            FuncVal::new(
                "baz2",
                false,
                vec![("y", Type::Int())],
                None,
                baz2_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "qux2",
        vec![(
            None,
            FuncVal::new(
                "qux2",
                false,
                vec![("y", Type::Int())],
                None,
                qux2_body.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
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
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    baz_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    qux_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    qux_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    baz2_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    baz2_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    qux2_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    qux2_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );

    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![Type::Int()], None)),
            (
                HashSet::from([RVal::Var("baz"), RVal::Var("qux")]),
                HashSet::new(),
            ),
        )),
    );
    foo_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    bar_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![Type::Int()], None)),
            (
                HashSet::from([RVal::Var("baz2"), RVal::Var("qux2")]),
                HashSet::new(),
            ),
        )),
    );
    bar_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );

    check_cmap.cmap.insert(
        "baz",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), baz_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), bar_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), foo_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![Type::Int()], None)),
            (
                HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                HashSet::new(),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "qux",
        Box::new(VarType::Scope(
            Some("foo"),
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), qux_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "baz2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), baz2_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "qux2",
        Box::new(VarType::Scope(
            Some("bar"),
            vec![(Box::new(Type::Func(vec![Type::Int()], None)), qux2_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_direct_invoke_uncertain() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(6))),
    ))]));
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
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, foo_body))),
        Box::new(FuncDef(FuncVal::new("bar", false, vec![], None, bar_body))),
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(InvokeFunc("foo", vec![])),
            Box::new(InvokeFunc("bar", vec![])),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    let mut bar_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    bar_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_indirect_invoke_uncertain() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "y",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let bar_body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(6))),
    ))]));
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
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
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
        Box::new(InvokeFunc("x", vec![])),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    let mut bar_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], None)),
            (
                HashSet::from([RVal::Var("foo"), RVal::Var("bar")]),
                HashSet::new(),
            ),
        )),
    );
    foo_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    bar_cmap.cmap.insert(
        "z",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(6)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), bar_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_indirect_invoke_err() {
    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(Assignment(
            "foo",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(InvokeFunc("foo", vec![])),
    ]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    assert_eq!(res, Err(Error::NotAFunction("foo")));
}

#[test]
fn test_switch_err() {
    let interp = Interpreter::new();
    let switch_vec: Vec<(RVal, Box<Statement>)> = vec![];
    let stmt = Switch(RVal::Var("x"), switch_vec);
    let res = interp.interp(
        &Funcs::new(),
        &mut ConstraintMap::new(),
        &mut Traits::new(),
        None,
        &stmt,
    );

    assert_eq!(res, Err(Error::UndefinedSymbol("x")));
}

#[test]
fn test_switch() {
    let interp = Interpreter::new();
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
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Switch(RVal::Var("x"), switch_vec)),
    ]);
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_switch_uncertain() {
    let interp = Interpreter::new();
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
    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "y",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(0), RVal::Num(1)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(4)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_direct_func_return_num() {
    let mut funcs = Funcs::new();
    let body = Box::new(Return(RVal::Num(5)));
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new(
                "foo",
                false,
                vec![],
                Some(Box::new(Type::Int())),
                body.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            Some(Box::new(Type::Int())),
            body,
        ))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "foo",
                vec![],
            )))),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_direct_func_return_set() {
    let mut funcs = Funcs::new();
    let body = Box::new(Sequence(vec![
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        )),
        Box::new(Return(RVal::Var("x"))),
    ]));
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new(
                "foo",
                false,
                vec![],
                Some(Box::new(Type::Int())),
                body.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            Some(Box::new(Type::Int())),
            body,
        ))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "foo",
                vec![],
            )))),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                foo_cmap,
            )],
        )),
    );
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(6)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_direct_func_return_func() {
    let bar_body = Box::new(Return(RVal::Num(0)));
    let baz_body = Box::new(Return(RVal::Num(1)));
    let foo_body = Box::new(Sequence(vec![
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
            )),
        )),
        Box::new(Return(RVal::Var("x"))),
    ]));

    let rettype = Box::new(Type::Int());
    let mut funcs = Funcs::new();
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new(
                "foo",
                false,
                vec![],
                Some(Box::new(Type::Func(vec![], Some(rettype.clone())))),
                foo_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "bar",
        vec![(
            None,
            FuncVal::new(
                "bar",
                false,
                vec![],
                Some(Box::new(Type::Int())),
                bar_body.clone(),
            ),
        )],
    );
    funcs.funcs.insert(
        "baz",
        vec![(
            None,
            FuncVal::new(
                "baz",
                false,
                vec![],
                Some(Box::new(Type::Int())),
                baz_body.clone(),
            ),
        )],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();

    let interp = Interpreter::new();
    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new(
            "foo",
            false,
            vec![],
            Some(Box::new(Type::Func(vec![], Some(Box::new(Type::Int()))))),
            foo_body,
        ))),
        Box::new(FuncDef(FuncVal::new(
            "bar",
            false,
            vec![],
            Some(Box::new(Type::Int())),
            bar_body,
        ))),
        Box::new(FuncDef(FuncVal::new(
            "baz",
            false,
            vec![],
            Some(Box::new(Type::Int())),
            baz_body,
        ))),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "foo",
                vec![],
            )))),
        )),
        Box::new(Assignment(
            "res",
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc("x", vec![])))),
        )),
    ]);
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
            (
                HashSet::from([RVal::Var("bar"), RVal::Var("baz")]),
                HashSet::new(),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(
                    vec![],
                    Some(Box::new(Type::Func(vec![], Some(Box::new(Type::Int()))))),
                )),
                foo_cmap,
            )],
        )),
    );
    check_cmap.cmap.insert(
        "bar",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "baz",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Func(vec![], Some(Box::new(Type::Int())))),
            (
                HashSet::from([RVal::Var("bar"), RVal::Var("baz")]),
                HashSet::new(),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "res",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1), RVal::Num(0)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_negative_constraints_eq() {
    let stmt = Sequence(vec![
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
        )),
        Box::new(Assignment(
            "f",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("f"))),
            Box::new(Assignment(
                "g",
                Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
            )),
            Box::new(Assignment(
                "h",
                Box::new(AssignmentRVal::RVal(RVal::Num(0))),
            )),
        )),
    ]);

    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "f",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "g",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (
                HashSet::from([RVal::Num(5), RVal::Num(3), RVal::Var("f")]),
                HashSet::new(),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "h",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(0)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_negative_constraints_neq() {
    let stmt = Sequence(vec![
        Box::new(Conditional(
            Box::new(BStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(3))),
            )),
        )),
        Box::new(Assignment(
            "f",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Not(Box::new(BStatement::Equals(
                RVal::Var("x"),
                RVal::Var("f"),
            )))),
            Box::new(Assignment(
                "g",
                Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
            )),
            Box::new(Assignment(
                "h",
                Box::new(AssignmentRVal::RVal(RVal::Num(0))),
            )),
        )),
    ]);

    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5), RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "f",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(3)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "g",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (
                HashSet::from([RVal::Num(5), RVal::Num(3)]),
                HashSet::from([RVal::Var("f")]),
            ),
        )),
    );
    check_cmap.cmap.insert(
        "h",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(0)]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_struct() {
    let stmt = Sequence(vec![
        Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
        Box::new(Assignment(
            "edgar",
            Box::new(AssignmentRVal::RVal(RVal::Struct(
                "Cat",
                vec![RVal::Var("9")],
                vec!["age"],
            ))),
        )),
    ]);

    let funcs = Funcs::new();
    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "edgar",
        Box::new(VarType::Values(
            Box::new(Type::Struct("Cat")),
            (
                HashSet::from([RVal::Struct("Cat", vec![RVal::Var("9")], vec!["age"])]),
                HashSet::new(),
            ),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

// TODO test structs as args/retvals

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
    funcs.funcs.insert(
        "speak",
        vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
    );

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    check_cmap.cmap.insert(
        "edgar",
        Box::new(VarType::Values(
            Box::new(Type::Struct("Cat")),
            (
                HashSet::from([RVal::Struct("Cat", vec![], vec![])]),
                HashSet::new(),
            ),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}

#[test]
fn test_dyn_traits_single_impl() {
    let funcdef = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );

    let cat_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(1))),
    ));
    let cat_funcimpl = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let gmaa = Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))]));

    let stmt = Sequence(vec![
        Box::new(TraitDecl("Animal", vec![funcdef.clone()])),
        Box::new(FuncDef(FuncVal::new(
            "giveMeAnAnimal",
            false,
            vec![],
            Some(Box::new(Type::DynTrait("Animal"))),
            gmaa.clone(),
        ))),
        Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
        Box::new(TraitImpl("Animal", "Cat", vec![cat_funcimpl.clone()])),
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
        vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
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
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_traits = Traits::new();
    check_traits.traits.insert("Animal", vec!["Cat"]);

    let mut check_cmap = ConstraintMap::new();
    let mut speak_cmap = ConstraintMap::new();
    speak_cmap.cmap.insert(
        "self",
        Box::new(VarType::Values(
            Box::new(Type::Struct("Cat")),
            (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
        )),
    );
    speak_cmap.cmap.insert(
        "spoken",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(1)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "speak",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![Type::Struct("Cat")], None)),
                speak_cmap,
            )],
        )),
    );
    check_cmap.cmap.insert(
        "giveMeAnAnimal",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::DynTrait("Animal"))))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "specific_animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (HashSet::from([RVal::IdkStruct("Cat")]), HashSet::new()),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
    assert_eq!(traits, check_traits);
}

#[test]
fn test_dyn_traits_two_impl() {
    let funcdecl = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );

    let cat_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(1))),
    ));
    let cat_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let dog_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(2))),
    ));
    let dog_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        dog_speak_body.clone(),
    );

    let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))])),
        Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Dog")))])),
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
        Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
        Box::new(Struct("Dog", vec![Type::Int()], vec!["age"])),
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
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_traits = Traits::new();
    check_traits.traits.insert("Animal", vec!["Cat", "Dog"]);

    let mut check_cmap = ConstraintMap::new();
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

    check_cmap.cmap.insert(
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
    check_cmap.cmap.insert(
        "giveMeAnAnimal",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::DynTrait("Animal"))))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "specific_animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (
                HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                HashSet::new(),
            ),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
    assert_eq!(traits, check_traits);
}

#[test]
fn test_dyn_traits_three_impl_two_used() {
    let funcdecl = FuncDecl::new(
        "speak",
        true,
        vec![("self", Type::DynTrait("Animal"))],
        None,
    );

    let bird_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(0))),
    ));
    let bird_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Bird"))],
        None,
        bird_speak_body.clone(),
    );

    let cat_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(1))),
    ));
    let cat_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Cat"))],
        None,
        cat_speak_body.clone(),
    );

    let dog_speak_body = Box::new(Assignment(
        "spoken",
        Box::new(AssignmentRVal::RVal(RVal::Num(2))),
    ));
    let dog_speak = FuncVal::new(
        "speak",
        true,
        vec![("self", Type::Struct("Dog"))],
        None,
        dog_speak_body.clone(),
    );

    let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
        Box::new(BStatement::TrueOrFalse()),
        Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Cat")))])),
        Box::new(Sequence(vec![Box::new(Return(RVal::IdkStruct("Dog")))])),
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
        Box::new(Struct("Bird", vec![Type::Int()], vec!["age"])),
        Box::new(Struct("Cat", vec![Type::Int()], vec!["age"])),
        Box::new(Struct("Dog", vec![Type::Int()], vec!["age"])),
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
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_traits = Traits::new();
    check_traits
        .traits
        .insert("Animal", vec!["Bird", "Cat", "Dog"]);

    let mut check_cmap = ConstraintMap::new();
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

    check_cmap.cmap.insert(
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
    check_cmap.cmap.insert(
        "giveMeAnAnimal",
        Box::new(VarType::Scope(
            None,
            vec![(
                Box::new(Type::Func(vec![], Some(Box::new(Type::DynTrait("Animal"))))),
                ConstraintMap::new(),
            )],
        )),
    );
    check_cmap.cmap.insert(
        "specific_animal",
        Box::new(VarType::Values(
            Box::new(Type::DynTrait("Animal")),
            (
                HashSet::from([RVal::IdkStruct("Cat"), RVal::IdkStruct("Dog")]),
                HashSet::new(),
            ),
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
    assert_eq!(traits, check_traits);
}

#[test]
fn use_main() {
    let mut funcs = Funcs::new();
    let foo_body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let main_body = Box::new(InvokeFunc("foo", vec![]));
    funcs.funcs.insert(
        "foo",
        vec![(
            None,
            FuncVal::new("foo", false, vec![], None, foo_body.clone()),
        )],
    );
    funcs.funcs.insert(
        "main",
        vec![(
            None,
            FuncVal::new("main", false, vec![], None, main_body.clone()),
        )],
    );

    let stmt = Sequence(vec![
        Box::new(FuncDef(FuncVal::new("foo", false, vec![], None, foo_body))),
        Box::new(FuncDef(FuncVal::new(
            "main",
            false,
            vec![],
            None,
            main_body,
        ))),
        Box::new(InvokeFunc("main", vec![])),
    ]);

    let mut cmap = ConstraintMap::new();
    let mut traits = Traits::new();
    let interp = Interpreter::new();
    let res = interp.interp(&funcs, &mut cmap, &mut traits, None, &stmt);

    let mut check_cmap = ConstraintMap::new();
    let mut foo_cmap = ConstraintMap::new();
    let main_cmap = ConstraintMap::new();
    foo_cmap.cmap.insert(
        "x",
        Box::new(VarType::Values(
            Box::new(Type::Int()),
            (HashSet::from([RVal::Num(5)]), HashSet::new()),
        )),
    );
    check_cmap.cmap.insert(
        "foo",
        Box::new(VarType::Scope(
            Some("main"),
            vec![(Box::new(Type::Func(vec![], None)), foo_cmap)],
        )),
    );
    check_cmap.cmap.insert(
        "main",
        Box::new(VarType::Scope(
            None,
            vec![(Box::new(Type::Func(vec![], None)), main_cmap)],
        )),
    );

    assert_eq!(res.unwrap(), None);
    assert_eq!(cmap, check_cmap);
}
