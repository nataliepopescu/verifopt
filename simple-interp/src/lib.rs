use std::collections::HashMap;
use std::ops::Not;
use thiserror::Error;

/// Define CFG

#[derive(Debug, Clone)]
pub enum Statement {
    Sequence(Vec<Box<Statement>>),
    Assignment(&'static str, i32),
    Var(&'static str),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
}

// intentionally skipping Or, And, Xor, Equals, and GreaterThan for simplicity
#[derive(Debug, Clone)]
pub enum BooleanStatement {
    Literal(bool),
    Not(Box<BooleanStatement>),
}

/// Define return types

pub enum Retval {
    // i32
    // functions
    // bools?
}

// FIXME better way to do this?
#[derive(Debug)]
pub enum PossibleBooleanValues {
    True(),
    False(),
    // rename -> Unknown() or Either() ?
    TrueOrFalse(),
}

impl Not for PossibleBooleanValues {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PossibleBooleanValues::True() => PossibleBooleanValues::False(),
            PossibleBooleanValues::False() => PossibleBooleanValues::True(),
            PossibleBooleanValues::TrueOrFalse() => {
                PossibleBooleanValues::TrueOrFalse()
            }
        }
    }
}

impl Not for &PossibleBooleanValues {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            PossibleBooleanValues::True() => &PossibleBooleanValues::False(),
            PossibleBooleanValues::False() => &PossibleBooleanValues::True(),
            PossibleBooleanValues::TrueOrFalse() => {
                &PossibleBooleanValues::TrueOrFalse()
            }
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
pub struct State {
    inner: HashMap<&'static str, i32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            inner: HashMap::<&'static str, i32>::new(),
        }
    }
}

pub struct Interpreter {}

/// Implement interpreter

impl Interpreter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn interp(&self, mem: State, stmt: Statement) -> Result<(State,
    Option<i32>), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => {
                self.interp_seq(mem, stmt_vec)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(mem, var, value)
            }
            Statement::Var(var) => self.interp_var(mem, var),
            Statement::Print(var) => self.interp_print(mem, var),
            Statement::Conditional(condition, if_branch, else_branch) => self
                .interp_conditional(mem, *condition, *if_branch, *else_branch),
        }
    }

    pub fn interp_seq(
        &self,
        mem: State,
        stmt_vec: Vec<Box<Statement>>,
    ) -> Result<(State, Option<i32>), Error> {
        let mut cur_mem = mem;
        for stmt in stmt_vec.iter() {
            let res = self.interp(cur_mem, *stmt.clone());
            if res.is_err() {
                return res;
            }
            cur_mem = res.unwrap().0;
        }
        Ok((cur_mem, None))
    }

    pub fn interp_assignment(
        &self,
        mem: State,
        var: &'static str,
        value: i32,
    ) -> Result<(State, Option<i32>), Error> {
        let mut new_mem = mem.clone();
        let _ = new_mem.inner.insert(var, value);
        Ok((new_mem, None))
    }

    pub fn interp_var(
        &self,
        mem: State,
        var: &'static str,
    ) -> Result<(State, Option<i32>), Error> {
        match mem.inner.clone().get(var) {
            Some(val) => return Ok((mem, Some(*val))),
            None => return Err(Error::UndefinedVariable(var)),
        }
    }

    pub fn interp_print(
        &self,
        mem: State,
        var: &'static str,
    ) -> Result<(State, Option<i32>), Error> {
        match mem.inner.get(var) {
            Some(val) => {
                println!("{}", val);
                Ok((mem, None))
            }
            None => return Err(Error::UndefinedVariable(var)),
        }
    }

    pub fn interp_bool(
        &self,
        mem: State,
        b_stmt: BooleanStatement,
    ) -> (State, PossibleBooleanValues) {
        // TODO when to return TrueOrFalse?
        match b_stmt {
            BooleanStatement::Literal(b) => {
                if b {
                    return (mem.clone(), PossibleBooleanValues::True());
                } else {
                    return (mem.clone(), PossibleBooleanValues::False());
                }
            }
            BooleanStatement::Not(inner_b_stmt) => {
                self.interp_not(mem, *inner_b_stmt)
            }
        }
    }

    pub fn interp_not(
        &self,
        mem: State,
        b_stmt: BooleanStatement,
    ) -> (State, PossibleBooleanValues) {
        let (new_mem, ret) = self.interp_bool(mem, b_stmt);
        (new_mem, !ret)
    }

    pub fn interp_conditional(
        &self,
        mem: State,
        condition: BooleanStatement,
        if_branch: Statement,
        else_branch: Statement,
    ) -> Result<(State, Option<i32>), Error> {
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
        Ok((mem, None))
    }

    pub fn possible(
        &self,
        _mem: &State,
        possible_b: &PossibleBooleanValues,
    ) -> bool {
        // FIXME somehow use mem
        match possible_b {
            PossibleBooleanValues::True() => true,
            PossibleBooleanValues::False() => false,
            PossibleBooleanValues::TrueOrFalse() => true,
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
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", 5);
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.unwrap().0.inner.get("x"), Some(&5));
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Print("x")),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.unwrap().0.inner.get("x"), Some(&5));
    }
    #[test]
    fn test_seq_assign() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Assignment("y", 6)),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.as_ref().unwrap().0.inner.get("x"), Some(&5));
        assert_eq!(res.as_ref().unwrap().0.inner.get("y"), Some(&6));
    }

    #[test]
    fn test_seq_print_undef() {
        let interp = Interpreter::new();
        let stmt_vec = vec![
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Print("y")),
        ];
        let stmt = Statement::Sequence(stmt_vec);
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.clone().err(), Some(Error::UndefinedVariable("y")));
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Sequence(vec![
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Print("x")),
            ])),
            Box::new(Statement::Sequence(vec![
                Box::new(Statement::Assignment("y", 6)),
                Box::new(Statement::Print("y")),
            ])),
        ]);
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.as_ref().unwrap().0.inner.get("x"), Some(&5));
        assert_eq!(res.as_ref().unwrap().0.inner.get("y"), Some(&6));
    }

    #[test]
    fn test_var_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Var("x");
        let res = interp.interp(State::new(), stmt);
        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_var() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Var("x"))
        ]);
        let res = interp.interp(State::new(), stmt);
        assert!(res.is_ok());
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Literal(true)),
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Assignment("x", 6)),
        );
        let res = interp.interp(State::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::Literal(false)),
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Assignment("x", 6)),
        );
        let res = interp.interp(State::new(), stmt);
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
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Assignment("x", 6)),
        );
        let res = interp.interp(State::new(), stmt);
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
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Assignment("x", 6)),
            )),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment("x", 7)),
                Box::new(Statement::Assignment("x", 8)),
            )),
        );
        let res = interp.interp(State::new(), stmt);
        assert!(res.is_ok());
        // FIXME how to check values set in conditional scopes
    }

    #[test]
    fn test_conditional_scope() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", 3)),
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Assignment("x", 6)),
            )),
        ]);
        let res = interp.interp(State::new(), stmt);
        assert!(res.is_ok());
        // FIXME want x == 5
    }
}
