use std::collections::HashMap;
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

// intentionally skipping Or, And, Xor, Equals, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanStatement {
    Literal(bool),
    Not(Box<BooleanStatement>),
}

/// Define return types

// FIXME better way to do this?
#[derive(Debug)]
pub enum PossibleBoolVals {
    True(),
    False(),
    // rename -> Unknown() or Either() ?
    TrueOrFalse(),
}

impl Not for PossibleBoolVals {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PossibleBoolVals::True() => PossibleBoolVals::False(),
            PossibleBoolVals::False() => PossibleBoolVals::True(),
            PossibleBoolVals::TrueOrFalse() => PossibleBoolVals::TrueOrFalse(),
        }
    }
}

impl Not for &PossibleBoolVals {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PossibleBoolVals::True() => &PossibleBoolVals::False(),
            PossibleBoolVals::False() => &PossibleBoolVals::True(),
            PossibleBoolVals::TrueOrFalse() => &PossibleBoolVals::TrueOrFalse(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Variable {0} is undefined.")]
    UndefinedVariable(&'static str),
    //#[error("Boolean interpretation failed")]
    //BooleanInterpFailed(),
}

/// Define interpreter state

#[derive(Debug, Clone)]
pub struct Store {
    inner: HashMap<&'static str, StoreVal>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<&'static str, StoreVal>::new(),
        }
    }
}

pub struct Interpreter {}

/// Implement interpreter

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(&self, mem: Store, stmt: Statement) -> Result<Store, Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.interp_seq(mem, stmt_vec),
            Statement::Assignment(var, value) => {
                self.interp_assignment(mem, var, value)
            }
            Statement::Print(var) => self.interp_print(mem, var),
            Statement::Conditional(condition, if_branch, else_branch) => self
                .interp_conditional(mem, *condition, *if_branch, *else_branch),
            Statement::FuncDef(name, body) => {
                self.interp_funcdef(mem, name, body)
            }
        }
    }

    pub fn interp_seq(
        &self,
        mem: Store,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<Store, Error> {
        let mut cur_mem = mem;
        for stmt in stmt_vec.iter() {
            let res = self.interp(cur_mem, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_mem = res.unwrap();
        }
        Ok(cur_mem)
    }

    pub fn interp_assignment(
        &self,
        mem: Store,
        var: &'static str,
        value: RVal,
    ) -> Result<Store, Error> {
        let mut new_mem = mem.clone();
        match value {
            RVal::Num(_) => {
                new_mem.inner.insert(var, value.into());
            },
            RVal::Var(varname) => {
                match new_mem.inner.get(varname) {
                    Some(val) => new_mem.inner.insert(var, val.clone()),
                    None => return Err(Error::UndefinedVariable(var)),
                };
            }
        }
        Ok(new_mem)
    }

    pub fn interp_print(
        &self,
        mem: Store,
        var: &'static str,
    ) -> Result<Store, Error> {
        match mem.inner.get(var) {
            Some(val) => {
                println!("{:#?}", val);
                Ok(mem)
            }
            None => return Err(Error::UndefinedVariable(var)),
        }
    }

    pub fn interp_bool(
        &self,
        mem: Store,
        b_stmt: BooleanStatement,
    ) -> (Store, PossibleBoolVals) {
        // TODO when to return TrueOrFalse?
        match b_stmt {
            BooleanStatement::Literal(b) => {
                if b {
                    return (mem.clone(), PossibleBoolVals::True());
                } else {
                    return (mem.clone(), PossibleBoolVals::False());
                }
            }
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(mem, *inner_b_stmt)
            }
        }
    }

    pub fn interp_not(
        &self,
        mem: Store,
        b_stmt: BooleanStatement,
    ) -> (Store, PossibleBoolVals) {
        let (new_mem, ret) = self.interp_bool(mem, b_stmt);
        (new_mem, !ret)
    }

    pub fn interp_conditional(
        &self,
        mem: Store,
        condition: BooleanStatement,
        if_branch: Statement,
        else_branch: Statement,
    ) -> Result<Store, Error> {
        let (base_mem, b_res) = self.interp_bool(mem.clone(), condition);
        println!("base_mem: {:?}", &base_mem);

        if self.possible(&base_mem, &b_res) {
            println!("if_branch: {:?}", &if_branch);
            let if_res = self.interp(base_mem.clone(), if_branch);
            if if_res.is_err() {
                return if_res;
            }
            println!("if_res: {:?}", if_res.unwrap());
            // FIXME the new state just gets dropped...
        }
        if self.possible(&base_mem, !&b_res) {
            let else_res = self.interp(base_mem, else_branch);
            if else_res.is_err() {
                return else_res;
            }
            println!("else_res: {:?}", else_res.unwrap());
            // FIXME the new state just gets dropped...
        }
        // returning mem, not base_mem, because considering the
        // if clause as a new scope
        Ok(mem)
    }

    pub fn interp_funcdef(
        &self,
        mem: Store,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<Store, Error> {
        let mut new_mem = mem.clone();
        new_mem.inner.insert(name, StoreVal::FuncPtr(body));
        Ok(new_mem)
    }
    pub fn possible(
        &self,
        _mem: &Store,
        possible_b: &PossibleBoolVals,
    ) -> bool {
        // FIXME somehow use mem
        match possible_b {
            PossibleBoolVals::True() => true,
            PossibleBoolVals::False() => false,
            PossibleBoolVals::TrueOrFalse() => true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Print("x");
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", RVal::Num(5));
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(res.unwrap().inner.get("x"), Some(&StoreVal::Num(5)));
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Print("x")),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(res.unwrap().inner.get("x"), Some(&StoreVal::Num(5)));
    }
    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Assignment(
                "y",
                RVal::Num(6),
            )),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(
            res.as_ref().unwrap().inner.get("x"),
            Some(&StoreVal::Num(5))
        );
        assert_eq!(
            res.as_ref().unwrap().inner.get("y"),
            Some(&StoreVal::Num(6))
        );
    }

    #[test]
    fn test_seq_print_undef() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Print("y")),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(res.clone().err(), Some(Error::UndefinedVariable("y")));
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(5),
                )),
                Box::new(Statement::Print("x")),
            ])),
            Box::new(Statement::Sequence(vec![
                Box::new(Statement::Assignment(
                    "y",
                    RVal::Num(6),
                )),
                Box::new(Statement::Print("y")),
            ])),
        ]);
        let res = interp.interp(Store::new(), stmt);
        assert_eq!(
            res.as_ref().unwrap().inner.get("x"),
            Some(&StoreVal::Num(5))
        );
        assert_eq!(
            res.as_ref().unwrap().inner.get("y"),
            Some(&StoreVal::Num(6))
        );
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
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Assignment("y", RVal::Var("x"))),
        ]);
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        assert_eq!(
            res.as_ref().unwrap().inner.get("x"),
            res.as_ref().unwrap().inner.get("y")
        );
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Literal(true)),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(6),
            )),
        );
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Literal(false)),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(6),
            )),
        );
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Not(Box::new(
                BooleanStatement::Literal(true),
            ))),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            )),
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(6),
            )),
        );
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_nested_conditional() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Literal(true)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(5),
                )),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(6),
                )),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(7),
                )),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(8),
                )),
            )),
        );
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(3),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(5),
                )),
                Box::new(Statement::Assignment(
                    "x",
                    RVal::Num(6),
                )),
            )),
        ]);
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        // FIXME want x == 5
    }

    #[test]
    fn test_funcdef() {
        let interp = Interpreter::new();
        let stmt = Statement::FuncDef(
            "foo",
            Box::new(Statement::Assignment(
                "x",
                RVal::Num(5),
            ))
        );
        let res = interp.interp(Store::new(), stmt);
        assert!(res.is_ok());
        assert!(res.unwrap().inner.get("foo").is_some());
    }
}
