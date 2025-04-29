use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Not;
use thiserror::Error;

/// Define CFG

pub enum Statement {
    Sequence(Box<Statement>, Box<Statement>),
    Assignment(&'static str, i32),
    Print(&'static str),
    Conditional(Box<BooleanStatement>, Box<Statement>, Box<Statement>),
}

// intentionally skipping And, Xor, and GreaterThan for simplicity
pub enum BooleanStatement {
    Literal(bool),
    Not(Box<BooleanStatement>),
    //Or(Box<BooleanStatement>),
    //Equals(Box<BooleanStatement>, Box<BooleanStatement>),
}

/// Define return types

// FIXME better way to do this?
pub enum PossibleBooleanValues {
    True(),
    False(),
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

#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("Variable {0} is undefined.")]
    UndefinedVariable(&'static str),
    //#[error("Boolean interpretation failed")]
    //BooleanInterpFailed(),
}

/// Define interpreter state

#[derive(Debug, Clone)]
pub struct Interpreter {
    mem: RefCell<HashMap<&'static str, i32>>,
}

/// Implement interpreter

impl Interpreter {
    pub fn new() -> Self {
        Self {
            mem: HashMap::new().into(),
        }
    }

    pub fn interp(&self, stmt: Statement) -> Result<(), Error> {
        match stmt {
            Statement::Sequence(first_stmt, second_stmt) => {
                self.interp_seq(*first_stmt, *second_stmt)
            }
            Statement::Assignment(var, value) => {
                self.interp_assignment(var, value)
            }
            Statement::Print(var) => self.interp_print(var),
            Statement::Conditional(condition, if_branch, else_branch) => {
                self.interp_conditional(*condition, *if_branch, *else_branch)
            }
        }
    }

    pub fn interp_seq(
        &self,
        stmt1: Statement,
        stmt2: Statement,
    ) -> Result<(), Error> {
        let res1 = self.interp(stmt1);
        if res1.is_err() {
            return res1;
        }
        let res2 = self.interp(stmt2);
        if res2.is_err() {
            return res2;
        }
        Ok(())
    }

    pub fn interp_assignment(
        &self,
        var: &'static str,
        value: i32,
    ) -> Result<(), Error> {
        let _ = self.mem.borrow_mut().insert(var, value);
        Ok(())
    }

    pub fn interp_print(&self, var: &'static str) -> Result<(), Error> {
        match self.mem.borrow().get(var) {
            Some(val) => {
                println!("{}", val);
                Ok(())
            }
            None => return Err(Error::UndefinedVariable(var)),
        }
    }

    pub fn interp_bool(
        &self,
        b_stmt: BooleanStatement,
    ) -> PossibleBooleanValues {
        // TODO when to return TrueOrFalse?
        match b_stmt {
            BooleanStatement::Literal(b) => {
                if b {
                    return PossibleBooleanValues::True();
                } else {
                    return PossibleBooleanValues::False();
                }
            }
            BooleanStatement::Not(inner_b_stmt) => self.interp_not(*inner_b_stmt),
        }
    }

    pub fn interp_not(&self, b_stmt: BooleanStatement) -> PossibleBooleanValues {
        !self.interp_bool(b_stmt)
    }

    pub fn interp_conditional(
        &self,
        condition: BooleanStatement,
        if_branch: Statement,
        else_branch: Statement,
    ) -> Result<(), Error> {
        let b_res = self.interp_bool(condition);

        if self.possible(&b_res) {
            let if_res = self.interp(if_branch);
            if if_res.is_err() {
                return if_res;
            }
        }
        if self.possible(!&b_res) {
            let else_res = self.interp(else_branch);
            if else_res.is_err() {
                return else_res;
            }
        }
        Ok(())
    }

    pub fn possible(&self, possible_b: &PossibleBooleanValues) -> bool {
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
        let res = interp.interp(stmt);
        assert_eq!(res.err(), Some(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", 5);
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&5));
    }

    #[test]
    fn test_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Print("x")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&5));
    }

    #[test]
    fn test_seq_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Print("y")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res.err(), Some(Error::UndefinedVariable("y")));
        assert_eq!(interp.mem.borrow().get("x"), Some(&5));
    }

    #[test]
    fn test_nested_seq() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Sequence(
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Print("x")),
            )),
            Box::new(Statement::Sequence(
                Box::new(Statement::Assignment("y", 6)),
                Box::new(Statement::Print("y")),
            )),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&5));
        assert_eq!(interp.mem.borrow().get("y"), Some(&6));
    }

    #[test]
    fn test_conditional_true() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(true)),
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Assignment("x", 6)),
            )),
            Box::new(Statement::Print("x")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&5));
    }

    #[test]
    fn test_conditional_false() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Literal(false)),
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Assignment("x", 6)),
            )),
            Box::new(Statement::Print("x")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&6));
    }

    #[test]
    fn test_conditional_not() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Conditional(
                Box::new(BooleanStatement::Not(
                    Box::new(BooleanStatement::Literal(true))
                )),
                Box::new(Statement::Assignment("x", 5)),
                Box::new(Statement::Assignment("x", 6)),
            )),
            Box::new(Statement::Print("x")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
        assert_eq!(interp.mem.borrow().get("x"), Some(&6));
    }
}
