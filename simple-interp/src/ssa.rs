use crate::error::Error;
use crate::statement::{Merge, RVal, Statement};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Symbols(HashMap<&'static str, Option<Box<Symbols>>>);

impl Symbols {
    pub fn new() -> Self {
        Self(HashMap::<&'static str, Option<Box<Symbols>>>::new())
    }
}

impl FromIterator<(&'static str, Option<Box<Symbols>>)> for Symbols {
    fn from_iter<I: IntoIterator<Item = (&'static str, Option<Box<Symbols>>)>>(
        iter: I,
    ) -> Self {
        let mut s = Symbols::new();

        for i in iter {
            s.0.insert(i.0, i.1);
        }

        s
    }
}

impl Merge<Symbols> for Vec<Symbols> {
    fn merge(&self) -> Result<Option<Symbols>, Error> {
        if self.is_empty() {
            return Ok(None);
        }
        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        for symbols in self.iter() {
            for (key, val) in symbols.clone().0.iter_mut() {
                match merged.0.get_mut(key) {
                    Some(mergedval) => match (val, mergedval) {
                        (None, None) => continue,
                        (Some(a), Some(b)) => {
                            if a != b {
                                todo!("when NOT EQUAL?")
                            }
                        }
                        _ => return Err(Error::SymbolAlreadyExists(key)),
                    },
                    None => {
                        merged.0.insert(key, val.clone());
                    }
                }
            }
        }

        Ok(Some(merged))
    }
}

pub struct SSAChecker {}

impl SSAChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn check(&self, symbols: &mut Symbols, stmt: &Statement) -> Result<(), Error> {
        match stmt {
            Statement::Assignment(name, _) => self.check_assignment(symbols, name),
            Statement::Sequence(stmt_vec) => self.check_seq(symbols, stmt_vec),
            Statement::Conditional(_, true_branch, false_branch) => {
                self.check_conditional(symbols, true_branch, false_branch)
            }
            Statement::Switch(_, vec) => self.check_switch(symbols, vec),
            Statement::FuncDef(func) => {
                self.check_funcdef(symbols, func.name, &func.params, &*func.body)
            }
            Statement::Struct(struct_name, _, _) => {
                self.check_struct(symbols, struct_name)
            }
            _ => Ok(()),
        }
    }

    fn check_assignment(
        &self,
        symbols: &mut Symbols,
        name: &'static str,
    ) -> Result<(), Error> {
        if symbols.0.contains_key(name) {
            return Err(Error::SymbolAlreadyExists(name));
        }
        symbols.0.insert(name, None);

        Ok(())
    }

    fn check_seq(
        &self,
        symbols: &mut Symbols,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter() {
            self.check(symbols, stmt)?;
        }
        Ok(())
    }

    fn check_conditional(
        &self,
        symbols: &mut Symbols,
        true_branch: &Statement,
        false_branch: &Statement,
    ) -> Result<(), Error> {
        let mut res_symbols = vec![];
        let mut symbols_clone = symbols.clone();

        self.check(symbols, true_branch)?;
        res_symbols.push(symbols.clone());

        self.check(&mut symbols_clone, false_branch)?;
        res_symbols.push(symbols_clone);

        match res_symbols.merge() {
            Ok(Some(new_symbols)) => {
                *symbols = new_symbols;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn check_switch(
        &self,
        symbols: &mut Symbols,
        vec: &Vec<(RVal, Box<Statement>)>,
    ) -> Result<(), Error> {
        let mut res_scopes = vec![];
        for (_, switch_stmt) in vec.iter() {
            let mut scoped_symbols = symbols.clone();
            self.check(&mut scoped_symbols, switch_stmt)?;
            res_scopes.push(scoped_symbols);
        }

        match res_scopes.merge() {
            Ok(Some(new_symbols)) => {
                *symbols = new_symbols;
                Ok(())
            }
            Ok(None) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn check_funcdef(
        &self,
        symbols: &mut Symbols,
        name: &'static str,
        params: &Vec<(&'static str, crate::statement::Type)>,
        body: &Statement,
    ) -> Result<(), Error> {
        if symbols.0.contains_key(name) {
            return Err(Error::SymbolAlreadyExists(name));
        }

        let mut func_symbols = Symbols::new();
        for (argname, _) in params.iter() {
            func_symbols.0.insert(argname, None);
        }
        self.check(&mut func_symbols, body)?;

        symbols.0.insert(name, Some(Box::new(func_symbols)));
        Ok(())
    }

    fn check_struct(
        &self,
        symbols: &mut Symbols,
        struct_name: &'static str,
    ) -> Result<(), Error> {
        if symbols.0.contains_key(struct_name) {
            return Err(Error::SymbolAlreadyExists(struct_name));
        }
        symbols.0.insert(struct_name, None);
        Ok(())
    }
}

#[cfg(test)]
mod ssa_tests;
