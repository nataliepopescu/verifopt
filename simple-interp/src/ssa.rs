use crate::{Error, RVal, Statement};

use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq)]
pub struct Symbols(HashSet<&'static str>);

impl Symbols {
    pub fn new() -> Self {
        Self(HashSet::<&'static str>::new())
    }
}

impl FromIterator<&'static str> for Symbols {
    fn from_iter<I: IntoIterator<Item = &'static str>>(iter: I) -> Self {
        let mut s = Symbols::new();

        for i in iter {
            s.0.insert(i);
        }

        s
    }
}

pub trait Merge {
    fn merge(&self) -> Result<Symbols, Error>;
}

impl Merge for Vec<Symbols> {
    fn merge(&self) -> Result<Symbols, Error> {
        if self.len() == 0 {
            return Err(Error::VecSize());
        }
        if self.len() == 1 {
            return Ok(self[0].clone());
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            merged = merged.0.union(&self[i].0).map(|x| *x).collect();
        }

        Ok(merged)
    }
}

pub struct SSAChecker {}

impl SSAChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(
        &self,
        symbols: &mut Symbols,
        stmt: &Statement,
    ) -> Result<(), Error> {
        match stmt {
            Statement::Print(_) => Ok(()),
            Statement::InvokeFunc(_) => Ok(()),
            Statement::Assignment(name, _) => {
                self.check_assignment(symbols, name)
            }
            Statement::Sequence(stmt_vec) => self.check_seq(symbols, stmt_vec),
            Statement::Conditional(_, true_branch, false_branch) => self
                .check_conditional(symbols, &(*true_branch), &(*false_branch)),
            Statement::Switch(_, vec) => self.check_switch(symbols, vec),
            Statement::FuncDef(name, body) => {
                self.check_funcdef(symbols, name, body)
            }
        }
    }

    pub fn check_assignment(
        &self,
        symbols: &mut Symbols,
        name: &'static str,
    ) -> Result<(), Error> {
        if symbols.0.get(name).is_some() {
            return Err(Error::SymbolAlreadyExists(name));
        }
        symbols.0.insert(name);

        Ok(())
    }

    pub fn check_seq(
        &self,
        symbols: &mut Symbols,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter() {
            let res = self.check(symbols, &(*stmt));
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    pub fn check_conditional(
        &self,
        symbols: &mut Symbols,
        true_branch: &Statement,
        false_branch: &Statement,
    ) -> Result<(), Error> {
        let mut res_symbols = vec![];
        let mut symbols_clone = symbols.clone();

        let res_true = self.check(symbols, true_branch);
        if res_true.is_err() {
            return res_true;
        }
        res_symbols.push(symbols.clone());

        let res_false = self.check(&mut symbols_clone, false_branch);
        if res_false.is_err() {
            return res_false;
        }
        res_symbols.push(symbols_clone);

        match res_symbols.merge() {
            Ok(new_symbols) => {
                *symbols = new_symbols;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn check_switch(
        &self,
        symbols: &mut Symbols,
        vec: &Vec<(RVal, Box<Statement>)>,
    ) -> Result<(), Error> {
        let mut res_scopes = vec![];
        for (_, switch_stmt) in vec.iter() {
            let mut scoped_symbols = symbols.clone();
            let res = self.check(&mut scoped_symbols, &(*switch_stmt));
            if res.is_err() {
                return res;
            }
            res_scopes.push(scoped_symbols);
        }

        match res_scopes.merge() {
            Ok(new_symbols) => {
                *symbols = new_symbols;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn check_funcdef(
        &self,
        symbols: &mut Symbols,
        name: &'static str,
        body: &Box<Statement>,
    ) -> Result<(), Error> {
        if symbols.0.get(name).is_some() {
            return Err(Error::SymbolAlreadyExists(name));
        }
        symbols.0.insert(name);

        self.check(symbols, body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BooleanStatement, Error};

    #[test]
    fn test_merge() {
        let mut symb1 = Symbols::new();
        symb1.0.insert("x");
        let mut symb2 = Symbols::new();
        symb2.0.insert("x");
        let vec: Vec<Symbols> = vec![symb1, symb2];

        let mut end_symb = Symbols::new();
        end_symb.0.insert("x");
        assert_eq!(end_symb, vec.merge().unwrap());
    }

    #[test]
    fn test_assignment_err() {
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res, Err(Error::SymbolAlreadyExists("x")));
    }

    #[test]
    fn test_funcdef_err() {
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::Sequence(vec![
            Box::new(Statement::FuncDef("foo", body.clone())),
            Box::new(Statement::FuncDef("foo", body.clone())),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res, Err(Error::SymbolAlreadyExists("foo")));
    }

    #[test]
    fn test_conditional_ok() {
        let stmt = Statement::Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Statement::Assignment("x", RVal::Num(5))),
            Box::new(Statement::Assignment("x", RVal::Num(6))),
        );

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res.unwrap(), ());
    }

    #[test]
    fn test_switch_ok() {
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

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res.unwrap(), ());
    }
}
