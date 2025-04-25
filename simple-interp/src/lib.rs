use std::cell::RefCell;
use std::collections::HashMap;
use thiserror::Error;

/// Define CFG

pub enum Statement {
    Sequence(Box<Statement>, Box<Statement>),
    Assignment(&'static str, i32),
    Print(&'static str),
}

/// Define error type

#[derive(Debug, PartialEq, Error)]
pub enum Error {
    #[error("Variable {0} is undefined.")]
    UndefinedVariable(&'static str),
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Print("x");
        let res = interp.interp(stmt);
        assert_eq!(res, Err(Error::UndefinedVariable("x")));
    }

    #[test]
    fn test_assignment() {
        let interp = Interpreter::new();
        let stmt = Statement::Assignment("x", 5);
        let res = interp.interp(stmt);
        assert_eq!(res, Ok(()));
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
    }

    #[test]
    fn test_seq_undef() {
        let interp = Interpreter::new();
        let stmt = Statement::Sequence(
            Box::new(Statement::Assignment("x", 5)),
            Box::new(Statement::Print("y")),
        );
        let res = interp.interp(stmt);
        assert_eq!(res, Err(Error::UndefinedVariable("y")));
    }
}
