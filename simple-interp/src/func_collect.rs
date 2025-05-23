use crate::{Error, FuncVal, Statement};
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
            Statement::FuncDef(name, body) => {
                self.collect_funcdef(funcs, name, body)
            }
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
        body: &Box<Statement>,
    ) -> Result<(), Error> {
        // FIXME remove duplicate name check (panic)
        match funcs.funcs.get(name) {
            Some(_) => {
                panic!("SSA BUG: funcname {:?} already exists", &name)
            }
            None => {
                funcs.funcs.insert(name, FuncVal::new(body.clone()));
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FuncVal, RVal, Statement};

    #[test]
    fn test_print() {
        let fc = FuncCollector::new();
        let stmt = Statement::Print("hello");
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_assign_num() {
        let fc = FuncCollector::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_seq() {
        let fc = FuncCollector::new();
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_nested_seq() {
        let fc = FuncCollector::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("x", RVal::Num(5)),
            )])),
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("y", RVal::Num(6)),
            )])),
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
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
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
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
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
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert("foo", FuncVal::new(body));
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_assign_funcptr() {
        let fc = FuncCollector::new();
        let body = Box::new(Statement::Assignment("y", RVal::Num(5)));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
        ]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert("foo", FuncVal::new(body.clone()));
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }
}
