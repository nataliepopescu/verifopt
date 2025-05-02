use std::collections::HashMap;
use std::fmt;
use std::ops::Not;
use thiserror::Error;

/// Define CFG

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, RVal),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
    // no args or retvals for now
    FuncDef(&'static str, Box<Statement>),
    InvokeFunc(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RVal {
    Var(&'static str),
    Num(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum StoreVal {
    Num(i32),
    FuncPtr(Box<Statement>),
}

impl From<RVal> for StoreVal {
    fn from(item: RVal) -> Self {
        match item {
            RVal::Num(num) => StoreVal::Num(num),
            RVal::Var(_) => panic!("cannot turn var name into StoreVal"),
        }
    }
}

impl fmt::Display for StoreVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoreVal::Num(num) => write!(f, "{:?}", num),
            StoreVal::FuncPtr(boxed) => write!(f, "{:?}", boxed),
        }
    }
}

// intentionally skipping Or, And, Xor, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanStatement {
    True(),
    False(),
    TrueOrFalse(),
    Not(Box<BooleanStatement>),
    Equals(RVal, RVal),
}

impl Not for BooleanStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BooleanStatement::True() => BooleanStatement::False(),
            BooleanStatement::False() => BooleanStatement::True(),
            BooleanStatement::TrueOrFalse() => BooleanStatement::TrueOrFalse(),
            BooleanStatement::Not(_) | BooleanStatement::Equals(_, _) => {
                panic!("not implemented yet")
            }
        }
    }
}

impl Not for &BooleanStatement {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            BooleanStatement::True() => &BooleanStatement::False(),
            BooleanStatement::False() => &BooleanStatement::True(),
            BooleanStatement::TrueOrFalse() => &BooleanStatement::TrueOrFalse(),
            BooleanStatement::Not(_) | BooleanStatement::Equals(_, _) => {
                panic!("not implemented yet (&)")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Variable {0} is undefined.")]
    UndefinedVariable(&'static str),
    #[error("Function {0} is undefined.")]
    UndefinedFunction(&'static str),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(StoreVal, StoreVal),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
}

/// Define interpreter state

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    inner: HashMap<&'static str, StoreVal>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<&'static str, StoreVal>::new(),
        }
    }

    pub fn insert(&mut self, key: &'static str, val: StoreVal) {
        self.inner.insert(key, val);
    }

    pub fn get(&self, key: &'static str) -> Option<&StoreVal> {
        self.inner.get(key)
    }
}

pub struct Interpreter {}

/// Implement interpreter

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(
        &self,
        stores: Vec<Store>,
        stmt: Statement,
    ) -> Result<Vec<Store>, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.interp_seq(stores, stmt_vec),
            Statement::Assignment(var, value) => {
                self.interp_assignment(stores, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok(stores)
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.interp_conditional(
                    stores,
                    *condition,
                    *true_branch,
                    *false_branch,
                )
            }
            Statement::FuncDef(name, body) => {
                self.interp_funcdef(stores, name, body)
            }
            Statement::InvokeFunc(name) => self.interp_invoke(stores, name),
        }
    }

    pub fn interp_seq(
        &self,
        stores: Vec<Store>,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<Vec<Store>, Error> {
        let mut cur_stores = stores;
        for stmt in stmt_vec.iter() {
            let res = self.interp(cur_stores, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_stores = res.unwrap();
        }
        Ok(cur_stores)
    }

    pub fn interp_assignment(
        &self,
        stores: Vec<Store>,
        var: &'static str,
        value: RVal,
    ) -> Result<Vec<Store>, Error> {
        let mut new_stores = stores.clone();
        for new_store in new_stores.iter_mut() {
            match value {
                RVal::Num(_) => {
                    new_store.insert(var, value.clone().into());
                }
                RVal::Var(varname) => {
                    match new_store.get(varname) {
                        Some(val) => new_store.insert(var, val.clone()),
                        None => return Err(Error::UndefinedVariable(var)),
                    };
                }
            }
        }
        Ok(new_stores)
    }

    pub fn interp_print(&self, var: &'static str) {
        println!("{:#?}", var);
    }

    pub fn interp_bool(
        &self,
        store: Store,
        b_stmt: BooleanStatement,
    ) -> Result<(Store, BooleanStatement), Error> {
        match b_stmt {
            BooleanStatement::True()
            | BooleanStatement::False()
            | BooleanStatement::TrueOrFalse() => Ok((store.clone(), b_stmt)),
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(store, *inner_b_stmt)
            }
            BooleanStatement::Equals(lhs, rhs) => {
                self.interp_equals(store, lhs, rhs)
            }
        }
    }

    pub fn interp_not(
        &self,
        store: Store,
        b_stmt: BooleanStatement,
    ) -> Result<(Store, BooleanStatement), Error> {
        let res = self.interp_bool(store, b_stmt);
        match res {
            Ok((new_stores, b_res)) => Ok((new_stores, !b_res)),
            Err(_) => res,
        }
    }

    pub fn interp_rval(
        &self,
        store: &Store,
        rval: RVal,
    ) -> Result<StoreVal, Error> {
        match rval {
            RVal::Num(num) => Ok(StoreVal::Num(num)),
            RVal::Var(var) => match store.get(var) {
                Some(val) => Ok(val.clone()),
                None => return Err(Error::UndefinedVariable(var)),
            },
        }
    }

    pub fn interp_equals(
        &self,
        store: Store,
        lhs: RVal,
        rhs: RVal,
    ) -> Result<(Store, BooleanStatement), Error> {
        let lhs_res = self.interp_rval(&store, lhs);
        if lhs_res.is_err() {
            return Err(lhs_res.err().unwrap());
        }
        let rhs_res = self.interp_rval(&store, rhs);
        if rhs_res.is_err() {
            return Err(rhs_res.err().unwrap());
        }

        // TODO when to return TrueOrFalse here?
        match (lhs_res.as_ref().unwrap(), rhs_res.as_ref().unwrap()) {
            (StoreVal::Num(lnum), StoreVal::Num(rnum)) => {
                if lnum == rnum {
                    return Ok((store, BooleanStatement::True()));
                } else {
                    return Ok((store, BooleanStatement::False()));
                }
            }
            (StoreVal::FuncPtr(lfp), StoreVal::FuncPtr(rfp)) => {
                if lfp == rfp {
                    return Ok((store, BooleanStatement::True()));
                } else {
                    return Ok((store, BooleanStatement::False()));
                }
            }
            (_, _) => {
                return Err(Error::IncomparableTypes(
                    lhs_res.unwrap(),
                    rhs_res.unwrap(),
                ));
            }
        }
    }

    pub fn interp_conditional(
        &self,
        stores: Vec<Store>,
        condition: BooleanStatement,
        true_branch: Statement,
        false_branch: Statement,
    ) -> Result<Vec<Store>, Error> {
        let mut res_stores: Vec<Store> = vec![];
        for store in stores.iter() {
            let c_res = self.interp_bool(store.clone(), condition.clone());
            match c_res {
                Ok((c_store, bool_res)) => {
                    if self.possible(&bool_res) {
                        let true_res = self
                            .interp(vec![c_store.clone()], true_branch.clone());
                        match true_res {
                            Ok(mut true_stores) => {
                                res_stores.append(&mut true_stores)
                            }
                            Err(_) => return true_res,
                        }
                    }
                    if self.possible(!&bool_res) {
                        let false_res =
                            self.interp(vec![c_store], false_branch.clone());
                        match false_res {
                            Ok(mut false_stores) => {
                                res_stores.append(&mut false_stores)
                            }
                            Err(_) => return false_res,
                        }
                    }
                }
                Err(c_err) => return Err(c_err),
            }
        }

        Ok(res_stores)
    }

    pub fn possible(&self, possible_b: &BooleanStatement) -> bool {
        match possible_b {
            BooleanStatement::True() => true,
            BooleanStatement::False() => false,
            BooleanStatement::TrueOrFalse() => true,
            _ => panic!("boolean statement not fully evaluated"),
        }
    }

    pub fn interp_funcdef(
        &self,
        stores: Vec<Store>,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<Vec<Store>, Error> {
        let mut new_stores = stores.clone();
        for new_store in &mut new_stores {
            new_store.insert(name, StoreVal::FuncPtr(body.clone()));
        }
        Ok(new_stores)
    }

    pub fn interp_invoke(
        &self,
        stores: Vec<Store>,
        name: &'static str,
    ) -> Result<Vec<Store>, Error> {
        let mut res_stores: Vec<Store> = vec![];
        for store in stores.iter() {
            match store.clone().get(name) {
                Some(StoreVal::FuncPtr(boxed_body)) => {
                    let res =
                        self.interp(vec![store.clone()], *boxed_body.clone());
                    match res {
                        Ok(mut new_stores) => {
                            res_stores.append(&mut new_stores)
                        }
                        Err(_) => return res,
                    }
                }
                Some(_) => return Err(Error::NotAFunction(name)),
                None => return Err(Error::UndefinedFunction(name)),
            }
        }
        Ok(res_stores)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Statement::Print("hello");
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);
        assert_eq!(res.as_ref().unwrap()[0], Store::new());
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Num(6))),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("x", RVal::Num(5)),
            )])),
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("y", RVal::Num(6)),
            )])),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
        ))]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assign_var() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::False()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_conditional_uncertain() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 2);

        let mut store1 = Store::new();
        store1.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store1);

        let mut store2 = Store::new();
        store2.insert("x", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[1], store2);
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Not(Box::new(BooleanStatement::True()))),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_conditional_equals() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Assignment("y", RVal::Num(3))),
            Box::new(Statement::Assignment("z", RVal::Num(0))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(3));
        store.insert("y", StoreVal::Num(3));
        store.insert("z", StoreVal::Num(1));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(6))),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Statement::Assignment("x", RVal::Num(7))),
                Box::new(Statement::Assignment("x", RVal::Num(8))),
            )),
        );
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::True()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(6))),
            )),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_funcdef() {
        let interp = Interpreter::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(body));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_invoke() {
        let interp = Interpreter::new();
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 1);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(body));
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store);
    }

    #[test]
    fn test_invoke_uncertain() {
        let interp = Interpreter::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(6)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::FuncDef("bar", bar_body.clone())),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::InvokeFunc("foo")),
                Box::new(Statement::InvokeFunc("bar")),
            )),
        ]);
        let res = interp.interp(vec![Store::new()], stmt);

        assert_eq!(res.as_ref().unwrap().len(), 2);

        let mut store1 = Store::new();
        store1.insert("foo", StoreVal::FuncPtr(foo_body.clone()));
        store1.insert("bar", StoreVal::FuncPtr(bar_body.clone()));
        store1.insert("x", StoreVal::Num(5));
        assert_eq!(res.as_ref().unwrap()[0], store1);

        let mut store2 = Store::new();
        store2.insert("foo", StoreVal::FuncPtr(foo_body));
        store2.insert("bar", StoreVal::FuncPtr(bar_body));
        store2.insert("x", StoreVal::Num(6));
        assert_eq!(res.as_ref().unwrap()[1], store2);
    }
}
