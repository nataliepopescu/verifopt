pub mod func_collector;
pub mod interpreter;
pub mod translator;

use crate::func_collector::{Env, FuncCollector};
use crate::interpreter::{Error, Interpreter, Statement, Store};

pub struct SimpleInterp {}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(&self, stmt: Statement) -> Result<(Store, Statement), Error> {
        let fc = FuncCollector::new();
        let fc_res = fc.collect(Env::new(), stmt.clone());

        let unwrapped_fc_res = fc_res.unwrap();
        let interp = Interpreter::new(unwrapped_fc_res.0);
        interp.interp(Store::new(), unwrapped_fc_res.1)
    }
}

#[cfg(test)]
mod tests {
    use super::interpreter::RVal;
    use super::*;

    #[test]
    fn test_print() {
        let stmt = Statement::Print("hello");

        let si = SimpleInterp::new();
        let res = si.interp(stmt.clone());

        assert_eq!(res.unwrap(), (Store::new(), stmt));
    }

    #[test]
    fn test_funcdef() {
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());

        let si = SimpleInterp::new();
        let res = si.interp(stmt.clone());

        assert_eq!(res.unwrap(), (Store::new(), stmt));
    }

    #[test]
    fn test_direct_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::InvokeFunc("foo")),
        ]);

        let si = SimpleInterp::new();
        let res = si.interp(stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_indirect_invoke() {
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);

        let si = SimpleInterp::new();
        let res = si.interp(stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Var("foo")]);
        store.inner.insert("z", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }
}
