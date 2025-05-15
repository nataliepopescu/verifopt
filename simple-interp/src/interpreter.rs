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
    // TODO replace w match
    Switch(RVal, Vec<(RVal, Box<Statement>)>),
    // no args or retvals for now
    FuncDef(&'static str, Box<Statement>),
    InvokeFunc(&'static str),
    // TODO traits
}

#[derive(Debug, Clone, PartialEq)]
pub enum RVal {
    Num(i32),
    Var(&'static str),
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVal::Num(num) => write!(f, "{}", num),
            RVal::Var(var) => write!(f, "{}", var),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FuncVal {
    body: Box<Statement>,
}

impl FuncVal {
    pub fn new(body: Box<Statement>) -> Self {
        Self { body }
    }
}

/*
#[derive(Debug, Clone, PartialEq)]
pub enum  {
    Num(i32),
    FuncPtr(Box<Statement>),
}

impl From<RVal> for  {
    fn from(item: RVal) -> Self {
        match item {
            RVal::Num(num) => ::Num(num),
            RVal::Var(_) => panic!("cannot turn var name into "),
        }
    }
}

impl fmt::Display for  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ::Num(num) => write!(f, "{:?}", num),
            ::FuncPtr(boxed) => write!(f, "{:?}", boxed),
        }
    }
}
*/

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
    #[error("Symbol {0} is undefined")]
    UndefinedSymbol(&'static str),
    //#[error("Variable {0} is undefined.")]
    //UndefinedVariable(&'static str),
    //#[error("Function {0} is undefined.")]
    //UndefinedFunction(&'static str),
    #[error("{0} cannot be compared to {1}.")]
    IncomparableTypes(RVal, RVal),
    #[error("{0} is not a function.")]
    NotAFunction(&'static str),
    #[error("Variable {0} already exists")]
    VarAlreadyExists(&'static str),
    #[error("Cannot perform merge on Vec with less than two elements")]
    VecSize(),
}

/// Define interpreter state

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    inner: HashMap<&'static str, Vec<RVal>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<&'static str, Vec<RVal>>::new(),
        }
    }
}

pub trait Merge {
    fn merge(&self) -> Result<Store, Error>;
}

impl Merge for Vec<Store> {
    fn merge(&self) -> Result<Store, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().inner.iter_mut() {
                match merged.inner.get_mut(key) {
                    Some(mut mval) => {
                        if val != mval {
                            val.append(&mut mval);
                            merged.inner.insert(key, val.to_vec());
                        }
                    }
                    None => {
                        merged.inner.insert(key, val.to_vec());
                    }
                }
            }
        }
        Ok(merged)
    }
}

pub struct Interpreter {
    funcs: HashMap<&'static str, FuncVal>,
}

/// Implement interpreter

impl Interpreter {
    pub fn new(env: crate::func_collector::Env) -> Self {
        Self { funcs: env.funcs }
    }

    pub fn interp(
        &self,
        store: Store,
        stmt: Statement,
    ) -> Result<(Store, Statement), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.interp_seq(store, stmt_vec),
            Statement::Assignment(var, value) => {
                self.interp_assignment(store, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok((store, stmt))
            }
            Statement::Conditional(condition, true_branch, false_branch) => {
                self.interp_conditional(
                    store,
                    *condition,
                    *true_branch,
                    *false_branch,
                )
            }
            Statement::Switch(val, vec) => self.interp_switch(store, val, vec),
            Statement::FuncDef(_, _) => Ok((store, stmt)),
            Statement::InvokeFunc(name) => {
                self.interp_invoke(store, name)
            }
        }
    }

    pub fn interp_seq(
        &self,
        store: Store,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<(Store, Statement), Error> {
        let mut cur_store = store;
        for stmt in stmt_vec.iter() {
            let res = self.interp(cur_store, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_store = res.unwrap().0;
        }
        Ok((cur_store, Statement::Sequence(stmt_vec)))
    }

    pub fn interp_assignment(
        &self,
        store: Store,
        var: &'static str,
        value: RVal,
    ) -> Result<(Store, Statement), Error> {
        let mut new_store = store.clone();
        if new_store.inner.get(var).is_some() {
            return Err(Error::VarAlreadyExists(var));
        }

        match value {
            RVal::Num(_) => {
                new_store.inner.insert(var, vec![value.clone().into()]);
            }
            ref varval @ RVal::Var(varname) => {
                match new_store.inner.get(varname) {
                    Some(val) => new_store.inner.insert(var, val.clone()),
                    None => match self.funcs.get(varname) {
                        Some(_) => {
                            new_store.inner.insert(var, vec![varval.clone()])
                        }
                        None => return Err(Error::UndefinedSymbol(varname)),
                    },
                };
            }
        }
        Ok((new_store, Statement::Assignment(var, value)))
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
        match self.interp_bool(store, b_stmt) {
            Ok((new_store, b_res)) => Ok((new_store, !b_res)),
            err @ Err(_) => err,
        }
    }

    pub fn interp_rval(
        &self,
        store: &Store,
        rval: RVal,
    ) -> Result<Vec<RVal>, Error> {
        match rval {
            num @ RVal::Num(_) => Ok(vec![num]),
            varval @ RVal::Var(var) => match store.inner.get(var) {
                Some(val) => Ok(val.clone()),
                None => match self.funcs.get(var) {
                    Some(_) => Ok(vec![varval]),
                    None => Err(Error::UndefinedSymbol(var)),
                },
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

        let lhs_vec = lhs_res.as_ref().unwrap();
        let rhs_vec = rhs_res.as_ref().unwrap();
        if lhs_vec.len() == 1 && rhs_vec.len() == 1 {
            match (lhs_vec[0].clone(), rhs_vec[0].clone()) {
                (RVal::Num(lnum), RVal::Num(rnum)) => {
                    if lnum == rnum {
                        return Ok((store, BooleanStatement::True()));
                    } else {
                        return Ok((store, BooleanStatement::False()));
                    }
                }
                (RVal::Var(lfp), RVal::Var(rfp)) => {
                    if lfp == rfp {
                        return Ok((store, BooleanStatement::True()));
                    } else {
                        return Ok((store, BooleanStatement::False()));
                    }
                }
                (_, _) => {
                    return Err(Error::IncomparableTypes(
                        lhs_vec[0].clone(),
                        rhs_vec[0].clone(),
                    ));
                }
            }
        } else {
            return Ok((store, BooleanStatement::TrueOrFalse()));
        }
    }

    pub fn interp_conditional(
        &self,
        store: Store,
        condition: BooleanStatement,
        true_branch: Statement,
        false_branch: Statement,
    ) -> Result<(Store, Statement), Error> {
        let mut res_stores: Vec<Store> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(store.clone(), condition.clone()) {
            Ok((c_store, bool_res)) => {
                if self.possible(&bool_res) {
                    match self.interp(c_store.clone(), true_branch.clone()) {
                        Ok((true_store, _)) => res_stores.push(true_store),
                        err @ Err(_) => return err,
                    }
                }
                if self.possible(!&bool_res) {
                    match self.interp(c_store, false_branch.clone()) {
                        Ok((false_store, _)) => res_stores.push(false_store),
                        err @ Err(_) => return err,
                    }
                }
            }
            Err(c_err) => return Err(c_err),
        }

        match res_stores.merge() {
            Ok(store) => Ok((
                store,
                Statement::Conditional(
                    Box::new(condition),
                    Box::new(true_branch),
                    Box::new(false_branch),
                ),
            )),
            Err(err) => Err(err),
        }
    }

    pub fn possible(&self, possible_b: &BooleanStatement) -> bool {
        match possible_b {
            BooleanStatement::True() => true,
            BooleanStatement::False() => false,
            BooleanStatement::TrueOrFalse() => true,
            _ => panic!("boolean statement not fully evaluated"),
        }
    }

    pub fn interp_switch(
        &self,
        store: Store,
        val: RVal,
        vec: Vec<(RVal, Box<Statement>)>,
    ) -> Result<(Store, Statement), Error> {
        // FIXME mod store if have effects
        let resolved_vals = match val {
            ref num @ RVal::Num(_) => vec![num.clone()],
            RVal::Var(varname) => match store.inner.get(varname) {
                Some(possible_vals) => possible_vals.to_vec(),
                None => return Err(Error::UndefinedSymbol(varname)),
            },
        };

        // grab all vec elems where vec_val is in store_vals
        let matching_vals: Vec<_> = vec
            .clone()
            .into_iter()
            .filter(|(vec_val, _)| resolved_vals.contains(vec_val))
            .collect();

        // loop to interp all such elems
        let mut res_stores: Vec<Store> = Vec::new();
        for (_, vec_stmt) in matching_vals.iter() {
            match self.interp(store.clone(), *vec_stmt.clone()) {
                Ok((new_store, _)) => res_stores.push(new_store),
                err @ Err(_) => return err,
            }
        }

        match res_stores.merge() {
            Ok(store) => Ok((store, Statement::Switch(val, vec))),
            Err(err) => Err(err),
        }
    }

    pub fn interp_indirect_invoke_helper(
        &self,
        store: Store,
        name: &'static str,
        val: &RVal,
    ) -> Result<(Store, Statement), Error> {
        match val {
            RVal::Var(name) => match self.funcs.get(name) {
                Some(func_val) => {
                    match self.interp(store, *func_val.body.clone()) {
                        Ok(new_store) => return Ok(new_store),
                        err @ Err(_) => return err,
                    }
                }
                None => return Err(Error::UndefinedSymbol(name)),
            },
            _ => return Err(Error::NotAFunction(name)),
        }
    }

    pub fn interp_indirect_invoke(
        &self,
        store: Store,
        name: &'static str,
        vec: &Vec<RVal>,
    ) -> Result<(Store, Statement), Error> {
        let mut res_stores: Vec<Store> = vec![];
        for val in vec.iter() {
            match self.interp_indirect_invoke_helper(store.clone(), name, val) {
                Ok((new_store, _)) => res_stores.push(new_store),
                err @ Err(_) => return err,
            }
        }
        match res_stores.merge() {
            Ok(store) => Ok((store, Statement::InvokeFunc(name))),
            Err(err) => Err(err),
        }
    }

    pub fn interp_direct_invoke(
        &self,
        store: Store,
        name: &'static str,
    ) -> Result<(Store, Statement), Error> {
        match self.funcs.get(name) {
            Some(func) => match self.interp(store, *func.body.clone()) {
                Ok((new_store, _)) => {
                    return Ok((new_store, Statement::InvokeFunc(name)));
                }
                err @ Err(_) => return err,
            },
            None => return Err(Error::UndefinedSymbol(name)),
        }
    }

    pub fn interp_invoke(
        &self,
        store: Store,
        name: &'static str,
    ) -> Result<(Store, Statement), Error> {
        match store.clone().inner.get(name) {
            Some(vec) => self.interp_indirect_invoke(store.clone(), name, vec),
            None => self.interp_direct_invoke(store.clone(), name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::func_collector::Env;

    #[test]
    fn test_merge_none() {
        let vec: Vec<Store> = Vec::new();
        assert_eq!(vec.merge(), Err(Error::VecSize()));
    }

    #[test]
    fn test_merge_one() {
        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        let vec: Vec<Store> = vec![store];
        assert_eq!(vec[0].clone(), vec.merge().unwrap());
    }

    #[test]
    fn test_merge_two() {
        let mut store1 = Store::new();
        store1.inner.insert("x", vec![RVal::Num(5)]);
        let mut store2 = Store::new();
        store2.inner.insert("x", vec![RVal::Num(6)]);
        let vec: Vec<Store> = vec![store1, store2];

        let mut end_store = Store::new();
        end_store
            .inner
            .insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(end_store, vec.merge().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Print("hello");
        let res = interp.interp(Store::new(), stmt.clone());

        assert_eq!(res.unwrap(), (Store::new(), stmt));
    }

    #[test]
    fn test_assign_num() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new(Env::new());
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new(Env::new());
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Num(6))),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        store.inner.insert("y", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("x", RVal::Num(5)),
            )])),
            Box::new(Statement::Sequence(vec![Box::new(
                Statement::Assignment("y", RVal::Num(6)),
            )])),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        store.inner.insert("y", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
        ))]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res.err(), Some(Error::UndefinedSymbol("y")));
    }

    #[test]
    fn test_assign_var() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        store.inner.insert("y", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::False()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_uncertain() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Not(Box::new(BooleanStatement::True()))),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(6)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_equals_num() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Assignment("y", RVal::Num(3))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(3)]);
        store.inner.insert("y", vec![RVal::Num(3)]);
        store.inner.insert("z", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_equals_func() {
        let mut env = Env::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        env.funcs.insert("bar", FuncVal::new(foo_body.clone()));

        // note: `equals` is _shallow_, which is why it evals to false here
        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::FuncDef("bar", foo_body.clone())),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("z", vec![RVal::Num(2)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_equals_func_ref() {
        let mut env = Env::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
            Box::new(Statement::Assignment("bar", RVal::Var("foo"))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("bar"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("bar", vec![RVal::Var("foo")]);
        store.inner.insert("z", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_equals_uncertain() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(3))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("y", RVal::Num(3))),
                Box::new(Statement::Assignment("y", RVal::Num(4))),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("x"),
                    RVal::Var("y"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(3)]);
        store.inner.insert("y", vec![RVal::Num(4), RVal::Num(3)]);
        store.inner.insert("z", vec![RVal::Num(2), RVal::Num(1)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_equals_err() {
        let mut env = Env::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Equals(
                    RVal::Var("foo"),
                    RVal::Var("x"),
                )),
                Box::new(Statement::Assignment("z", RVal::Num(1))),
                Box::new(Statement::Assignment("z", RVal::Num(2))),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(
            res,
            Err(Error::IncomparableTypes(RVal::Var("foo"), RVal::Num(5)))
        );
    }

    // skipped in collect
    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new(Env::new());
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
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    // skipped in collect
    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![Box::new(Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        ))]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_funcdef() {
        let mut env = Env::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        env.funcs.insert("foo", FuncVal::new(body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::FuncDef("foo", body);
        let res = interp.interp(Store::new(), stmt.clone());

        assert_eq!(res.unwrap(), (Store::new(), stmt));
    }

    // skipped in collect (rest of tests)

    #[test]
    fn test_direct_invoke() {
        let mut env = Env::new();
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        env.funcs.insert("foo", FuncVal::new(body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_indirect_invoke() {
        let mut env = Env::new();
        let body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        env.funcs.insert("foo", FuncVal::new(body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body)),
            Box::new(Statement::Assignment("x", RVal::Var("foo"))),
            Box::new(Statement::InvokeFunc("x")),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Var("foo")]);
        store.inner.insert("z", vec![RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_direct_invoke_uncertain() {
        let mut env = Env::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(6)),
        )]));
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        env.funcs.insert("bar", FuncVal::new(bar_body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::FuncDef("bar", bar_body)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::InvokeFunc("foo")),
                Box::new(Statement::InvokeFunc("bar")),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(6), RVal::Num(5)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_indirect_invoke_uncertain() {
        let mut env = Env::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(5)),
        )]));
        let bar_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("z", RVal::Num(6)),
        )]));
        env.funcs.insert("foo", FuncVal::new(foo_body.clone()));
        env.funcs.insert("bar", FuncVal::new(bar_body.clone()));

        let interp = Interpreter::new(env);
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body)),
            Box::new(Statement::FuncDef("bar", bar_body)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Var("foo"))),
                Box::new(Statement::Assignment("x", RVal::Var("bar"))),
            )),
            Box::new(Statement::InvokeFunc("x")),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store
            .inner
            .insert("x", vec![RVal::Var("bar"), RVal::Var("foo")]);
        store.inner.insert("z", vec![RVal::Num(5), RVal::Num(6)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_indirect_invoke_err() {
        let interp = Interpreter::new(Env::new());
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("foo", RVal::Num(5))),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res, Err(Error::NotAFunction("foo")));
    }

    #[test]
    fn test_switch_err() {
        let interp = Interpreter::new(Env::new());
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![];
        let stmt = Statement::Switch(RVal::Var("x"), switch_vec);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res, Err(Error::UndefinedSymbol("x")));
    }

    #[test]
    fn test_switch() {
        let interp = Interpreter::new(Env::new());
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0))),
            ),
            (
                RVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1))),
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(5)]);
        store.inner.insert("y", vec![RVal::Num(1)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }

    #[test]
    fn test_switch_uncertain() {
        let interp = Interpreter::new(Env::new());
        let switch_vec: Vec<(RVal, Box<Statement>)> = vec![
            (
                RVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0))),
            ),
            (
                RVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1))),
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(4))),
            )),
            Box::new(Statement::Switch(RVal::Var("x"), switch_vec)),
        ]);
        let res = interp.interp(Store::new(), stmt.clone());

        let mut store = Store::new();
        store.inner.insert("x", vec![RVal::Num(4), RVal::Num(5)]);
        store.inner.insert("y", vec![RVal::Num(1), RVal::Num(0)]);
        assert_eq!(res.unwrap(), (store, stmt));
    }
}
