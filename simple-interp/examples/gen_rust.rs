use simple_interp::SimpleInterp;
use simple_interp::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, InvokeTraitFunc, Print, Return,
    Sequence, Struct, Switch, TraitDecl, TraitImpl,
};
use simple_interp::statement::{
    AssignmentRVal, BStatement, FuncDecl, FuncVal, RVal, Statement, Type,
};

fn run(stmt: Statement) {
    let si = SimpleInterp::new();
    let rw_stmt = si.interp(stmt).unwrap();
    println!("{}", rw_stmt);
}

fn print() {
    let stmt = Print("hello world");
    run(stmt);
}

fn assign_num() {
    let stmt = Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
    run(stmt);
}

fn assign_var() {
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
    run(stmt);
}

fn seq() {
    let stmt_vec = vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))];
    let stmt = Sequence(stmt_vec);
    run(stmt);
}

fn seq_assign() {
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
    run(stmt);
}

fn cond_true() {
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
    run(stmt);
}

fn cond_false() {
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
    run(stmt);
}

fn cond_not() {
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
    run(stmt);
}

fn funcdef() {
    let body = Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ));
    let stmt = FuncDef("foo", false, vec![], None, body.clone());
    run(stmt);
}

fn direct_invoke() {
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "x",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let stmt = Sequence(vec![
        Box::new(FuncDef("foo", false, vec![], None, body)),
        Box::new(InvokeFunc("foo", vec![])),
    ]);
    run(stmt);
}

fn direct_invoke_args() {
    let body = Box::new(Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Print("0")),
            Box::new(Print("1")),
        )),
    ]));

    let stmt = Sequence(vec![
        Box::new(FuncDef(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            body.clone(),
        )),
        Box::new(Assignment(
            "arg",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(InvokeFunc("foo", vec!["arg"])),
    ]);
    run(stmt);
}

fn direct_invoke_ret() {
    let body = Box::new(Return(RVal::Num(5)));
    let stmt = Sequence(vec![
        Box::new(FuncDef(
            "foo",
            false,
            vec![],
            Some(Box::new(Type::Int())),
            body,
        )),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::Statement(Box::new(InvokeFunc(
                "foo",
                vec![],
            )))),
        )),
    ]);
    run(stmt);
}

fn indirect_invoke() {
    let body = Box::new(Sequence(vec![Box::new(Assignment(
        "z",
        Box::new(AssignmentRVal::RVal(RVal::Num(5))),
    ))]));
    let stmt = Sequence(vec![
        Box::new(FuncDef("foo", false, vec![], None, body)),
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(InvokeFunc("x", vec![])),
    ]);
    run(stmt);
}

// start making runnable by adding `main()`

fn indirect_invoke_args() {
    let foo_body = Box::new(Sequence(vec![
        Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(Conditional(
            Box::new(BStatement::Equals(RVal::Var("x"), RVal::Var("y"))),
            Box::new(Print("0")),
            Box::new(Print("1")),
        )),
    ]));
    let main_body = Box::new(Sequence(vec![
        Box::new(Assignment(
            "w",
            Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
        )),
        Box::new(Assignment(
            "arg",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        )),
        Box::new(InvokeFunc("w", vec!["arg"])),
    ]));
    let stmt = Sequence(vec![
        Box::new(FuncDef(
            "foo",
            false,
            vec![("y", Type::Int())],
            None,
            foo_body,
        )),
        Box::new(FuncDef(
            "main",
            false,
            vec![],
            None,
            main_body,
        )),
    ]);
    run(stmt);
}

fn main() {
    //let args: Vec<String> = env::args().collect();
    //if args.len() == 0 {
    //    println!("filename required");
    //    std::process::exit(1);
    //}

    indirect_invoke_args();
}
