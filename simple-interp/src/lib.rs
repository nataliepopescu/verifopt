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
    Switch(RVal, Vec<(StoreVal, Box<Statement>)>),
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
    #[error("Variable {0} already exists")]
    VarAlreadyExists(&'static str),
    #[error("Cannot perform merge on Vec with less than two elements")]
    VecSize(),
}

/// Define interpreter state

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    inner: HashMap<&'static str, Vec<StoreVal>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<&'static str, Vec<StoreVal>>::new(),
        }
    }

    pub fn insert(&mut self, key: &'static str, val: StoreVal) {
        self.inner.insert(key, vec![val]);
    }

    pub fn insert_vec(&mut self, key: &'static str, val: Vec<StoreVal>) {
        self.inner.insert(key, val);
    }

    pub fn get(&self, key: &'static str) -> Option<&Vec<StoreVal>> {
        self.inner.get(key)
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
                            merged.insert_vec(key, val.to_vec());
                        }
                    }
                    None => {
                        merged.insert_vec(key, val.to_vec());
                    }
                }
            }
        }
        Ok(merged)
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
        store: Store,
        stmt: Statement,
    ) -> Result<Store, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.interp_seq(store, stmt_vec),
            Statement::Assignment(var, value) => {
                self.interp_assignment(store, var, value)
            }
            Statement::Print(var) => {
                self.interp_print(var);
                Ok(store)
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
            Statement::FuncDef(name, body) => {
                self.interp_funcdef(store, name, body)
            }
            Statement::InvokeFunc(name) => self.interp_invoke(store, name),
        }
    }

    pub fn interp_seq(
        &self,
        store: Store,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<Store, Error> {
        let mut cur_store = store;
        for stmt in stmt_vec.iter() {
            let res = self.interp(cur_store, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_store = res.unwrap();
        }
        Ok(cur_store)
    }

    pub fn interp_assignment(
        &self,
        store: Store,
        var: &'static str,
        value: RVal,
    ) -> Result<Store, Error> {
        let mut new_store = store.clone();
        match value {
            RVal::Num(_) => {
                new_store.insert(var, value.clone().into());
            }
            RVal::Var(varname) => {
                match new_store.get(varname) {
                    Some(val) => new_store.insert_vec(var, val.clone()),
                    None => return Err(Error::UndefinedVariable(var)),
                };
            }
        }
        Ok(new_store)
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
            e @ Err(_) => e,
        }
    }

    pub fn interp_rval(
        &self,
        store: &Store,
        rval: RVal,
    ) -> Result<Vec<StoreVal>, Error> {
        match rval {
            RVal::Num(num) => Ok(vec![StoreVal::Num(num)]),
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

        let lhs_vec = lhs_res.as_ref().unwrap();
        let rhs_vec = rhs_res.as_ref().unwrap();
        if lhs_vec.len() == 1 && rhs_vec.len() == 1 {
            match (lhs_vec[0].clone(), rhs_vec[0].clone()) {
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
    ) -> Result<Store, Error> {
        let mut res_stores: Vec<Store> = vec![];

        // FIXME mod store if have effects
        match self.interp_bool(store.clone(), condition.clone()) {
            Ok((c_store, bool_res)) => {
                if self.possible(&bool_res) {
                    match self.interp(c_store.clone(), true_branch.clone()) {
                        Ok(true_store) => res_stores.push(true_store),
                        e @ Err(_) => return e,
                    }
                }
                if self.possible(!&bool_res) {
                    match self.interp(c_store, false_branch.clone()) {
                        Ok(false_store) => res_stores.push(false_store),
                        e @ Err(_) => return e,
                    }
                }
            }
            Err(c_err) => return Err(c_err),
        }

        res_stores.merge()
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
        vec: Vec<(StoreVal, Box<Statement>)>,
    ) -> Result<Store, Error> {
        // FIXME mod store if have effects
        let store_vals = match val {
            RVal::Num(num) => vec![StoreVal::Num(num)],
            RVal::Var(varname) => match store.get(varname) {
                Some(possible_storevals) => possible_storevals.to_vec(),
                None => return Err(Error::UndefinedVariable(varname)),
            },
        };

        // grab all vec elems where vec_val is in store_vals
        let matching_vals: Vec<_> = vec
            .into_iter()
            .filter(|(vec_val, _)| store_vals.contains(vec_val))
            .collect();

        // loop to interp all such elems
        let mut res_stores: Vec<Store> = Vec::new();
        for (_, vec_stmt) in matching_vals.iter() {
            match self.interp(store.clone(), *vec_stmt.clone()) {
                Ok(new_store) => res_stores.push(new_store),
                e @ Err(_) => return e,
            }
        }

        res_stores.merge()
    }

    pub fn interp_funcdef(
        &self,
        store: Store,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<Store, Error> {
        match store.get(name) {
            Some(_) => {
                return Err(Error::VarAlreadyExists(name));
            }
            None => {
                let mut new_store = store.clone();
                new_store.insert(name, StoreVal::FuncPtr(body.clone()));
                Ok(new_store)
            }
        }
    }

    pub fn interp_invoke_helper(
        &self,
        store: Store,
        name: &'static str,
        val: &StoreVal,
    ) -> Result<Store, Error> {
        match val {
            StoreVal::FuncPtr(boxed_body) => {
                match self.interp(store, *boxed_body.clone()) {
                    Ok(new_store) => return Ok(new_store),
                    e @ Err(_) => return e,
                }
            }
            _ => return Err(Error::NotAFunction(name)),
        }
    }

    pub fn interp_invoke(
        &self,
        store: Store,
        name: &'static str,
    ) -> Result<Store, Error> {
        match store.clone().get(name) {
            Some(vec) => {
                let mut res_stores: Vec<Store> = vec![];
                for val in vec.iter() {
                    match self.interp_invoke_helper(store.clone(), name, val) {
                        Ok(new_store) => res_stores.push(new_store),
                        e @ Err(_) => return e,
                    }
                }
                return res_stores.merge();
            }
            None => return Err(Error::UndefinedFunction(name)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_none() {
        let vec: Vec<Store> = Vec::new();
        assert_eq!(vec.merge(), Err(Error::VecSize()));
    }

    #[test]
    fn test_merge_one() {
        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        let vec: Vec<Store> = vec![store];
        assert_eq!(vec[0].clone(), vec.merge().unwrap());
    }

    #[test]
    fn test_merge_two() {
        let mut store1 = Store::new();
        store1.insert("x", StoreVal::Num(5));
        let mut store2 = Store::new();
        store2.insert("x", StoreVal::Num(6));
        let vec: Vec<Store> = vec![store1, store2];

        let mut end_store = Store::new();
        end_store.insert_vec("x", vec![StoreVal::Num(6), StoreVal::Num(5)]);
        assert_eq!(end_store, vec.merge().unwrap());
    }

    #[test]
    fn test_print() {
        let interp = Interpreter::new();
        let stmt = Statement::Print("hello");
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res.unwrap(), Store::new());
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![Box::new(Statement::Assignment("x", RVal::Num(5)))];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Num(6))),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(6));
        assert_eq!(res.unwrap(), store);
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(6));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![Box::new(Statement::Assignment(
            "x",
            RVal::Var("y"),
        ))]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assign_var() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::True()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::False()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(6));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_uncertain() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert_vec("x", vec![StoreVal::Num(6), StoreVal::Num(5)]);
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Not(Box::new(BooleanStatement::True()))),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(6));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_equals_num() {
        let interp = Interpreter::new();
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(3));
        store.insert("y", StoreVal::Num(3));
        store.insert("z", StoreVal::Num(1));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_equals_func() {
        let interp = Interpreter::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(foo_body.clone()));
        store.insert("bar", StoreVal::FuncPtr(foo_body.clone()));
        store.insert("z", StoreVal::Num(1));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_equals_func_ref() {
        let interp = Interpreter::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(foo_body.clone()));
        store.insert("bar", StoreVal::FuncPtr(foo_body.clone()));
        store.insert("z", StoreVal::Num(1));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_equals_uncertain() {
        let interp = Interpreter::new();
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(3));
        store.insert_vec("y", vec![StoreVal::Num(4), StoreVal::Num(3)]);
        store.insert_vec("z", vec![StoreVal::Num(2), StoreVal::Num(1)]);
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_conditional_equals_err() {
        let interp = Interpreter::new();
        let foo_body = Box::new(Statement::Sequence(vec![Box::new(
            Statement::Assignment("x", RVal::Num(5)),
        )]));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", foo_body.clone())),
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
            Err(Error::IncomparableTypes(
                StoreVal::FuncPtr(foo_body),
                StoreVal::Num(5)
            ))
        );
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_funcdef() {
        let interp = Interpreter::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(body));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_funcdef_err() {
        let interp = Interpreter::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::FuncDef("foo", body.clone())),
        ]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res, Err(Error::VarAlreadyExists("foo")));
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(body));
        store.insert("x", StoreVal::Num(5));
        assert_eq!(res.unwrap(), store);
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
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("foo", StoreVal::FuncPtr(foo_body));
        store.insert("bar", StoreVal::FuncPtr(bar_body));
        store.insert_vec("x", vec![StoreVal::Num(6), StoreVal::Num(5)]);
        assert_eq!(res.clone().unwrap(), store);
    }

    #[test]
    fn test_invoke_err() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("foo", RVal::Num(5))),
            Box::new(Statement::InvokeFunc("foo")),
        ]);
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res, Err(Error::NotAFunction("foo")));
    }

    #[test]
    fn test_switch_err() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(StoreVal, Box<Statement>)> = vec![];
        let stmt = Statement::Switch(
            RVal::Var("x"),
            switch_vec,
        );
        let res = interp.interp(Store::new(), stmt);

        assert_eq!(res, Err(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_switch() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(StoreVal, Box<Statement>)> = vec![
            (
                StoreVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0)))
            ),
            (
                StoreVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1)))
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Switch(
                RVal::Var("x"),
                switch_vec
            )),
        ]);
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert("x", StoreVal::Num(5));
        store.insert("y", StoreVal::Num(1));
        assert_eq!(res.unwrap(), store);
    }

    #[test]
    fn test_switch_uncertain() {
        let interp = Interpreter::new();
        let switch_vec: Vec<(StoreVal, Box<Statement>)> = vec![
            (
                StoreVal::Num(4),
                Box::new(Statement::Assignment("y", RVal::Num(0)))
            ),
            (
                StoreVal::Num(5),
                Box::new(Statement::Assignment("y", RVal::Num(1)))
            ),
        ];
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Statement::Assignment("x", RVal::Num(5))),
                Box::new(Statement::Assignment("x", RVal::Num(4))),
            )),
            Box::new(Statement::Switch(
                RVal::Var("x"),
                switch_vec
            )),
        ]);
        let res = interp.interp(Store::new(), stmt);

        let mut store = Store::new();
        store.insert_vec("x", vec![StoreVal::Num(4), StoreVal::Num(5)]);
        store.insert_vec("y", vec![StoreVal::Num(1), StoreVal::Num(0)]);
        assert_eq!(res.unwrap(), store);
    }
}
