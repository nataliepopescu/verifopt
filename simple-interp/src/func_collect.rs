use crate::{Error, FuncVal, Statement, Type};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Funcs {
    pub funcs: HashMap<&'static str, FuncVal>,
}

impl Funcs {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::<&'static str, FuncVal>::new(),
        }
    }
}

pub struct FuncCollector {}

impl FuncCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(
        &self,
        funcs: &mut Funcs,
        stmt: &Statement,
    ) -> Result<(), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.collect_seq(funcs, stmt_vec),
            Statement::FuncDef(name, paramtypes, params, rettype, body) => self
                .collect_funcdef(
                    funcs, name, paramtypes, params, rettype, body,
                ),
            _ => Ok(()),
        }
    }

    pub fn collect_seq(
        &self,
        funcs: &mut Funcs,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter() {
            let res = self.collect(funcs, &*stmt);
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    pub fn collect_funcdef(
        &self,
        funcs: &mut Funcs,
        name: &'static str,
        paramtypes: &Vec<Type>,
        params: &Vec<&'static str>,
        rettype: &Option<Box<Type>>,
        body: &Box<Statement>,
    ) -> Result<(), Error> {
        // FIXME remove duplicate name check (panic)
        match funcs.funcs.get(name) {
            Some(_) => {
                panic!("SSA BUG: funcname {:?} already exists", &name)
            }
            None => {
                funcs.funcs.insert(
                    name,
                    FuncVal::new(
                        paramtypes.clone(),
                        params.clone(),
                        rettype.clone(),
                        body.clone(),
                    ),
                );

                self.collect(funcs, body)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BooleanStatement;
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Sequence,
    };
    use crate::{AssignmentRVal, FuncVal, RVal};

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
        let stmt =
            Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
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
        let stmt = FuncDef("foo", vec![], vec![], None, body.clone());
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body));
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
        let stmt =
            FuncDef("foo", vec![Type::Int()], vec!["y"], None, body.clone());
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "foo",
            FuncVal::new(vec![Type::Int()], vec!["y"], None, body),
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
            Box::new(FuncDef("foo", vec![], vec![], None, body.clone())),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
        ]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body.clone()));
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
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let mut funcs = Funcs::new();
        let fc = FuncCollector::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs
            .funcs
            .insert("bar", FuncVal::new(vec![], vec![], None, bar_body));
        check_funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, foo_body));

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
            Box::new(FuncDef("baz", vec![], vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", vec![], vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
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
            Box::new(FuncDef("baz2", vec![], vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", vec![], vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
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
            Box::new(FuncDef("foo", vec![], vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", vec![], vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
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
            FuncVal::new(vec![], vec![], None, foo_body.clone()),
        );
        check_funcs.funcs.insert(
            "bar",
            FuncVal::new(vec![], vec![], None, bar_body.clone()),
        );
        check_funcs.funcs.insert(
            "baz",
            FuncVal::new(vec![], vec![], None, baz_body.clone()),
        );
        check_funcs.funcs.insert(
            "qux",
            FuncVal::new(vec![], vec![], None, qux_body.clone()),
        );
        check_funcs.funcs.insert(
            "baz2",
            FuncVal::new(vec![], vec![], None, baz2_body.clone()),
        );
        check_funcs.funcs.insert(
            "qux2",
            FuncVal::new(vec![], vec![], None, qux2_body.clone()),
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }
}
