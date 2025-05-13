use std::collections::HashMap;
use thiserror::Error;

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

// intentionally skipping Or, And, Xor, and GreaterThan for simplicity
#[derive(Debug, Clone, PartialEq)]
pub enum BooleanStatement {
    True(),
    False(),
    TrueOrFalse(),
    Not(Box<BooleanStatement>),
    Equals(RVal, RVal),
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum Error {
    #[error("Variable {0} already exists")]
    VarAlreadyExists(&'static str),
}

/// Define collector state

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    funcs: HashMap<&'static str, StoreVal>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::<&'static str, StoreVal>::new(),
        }
    }
}

pub struct Collector {}

/// Implement collector

impl Collector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(&self, env: Env, stmt: Statement) -> Result<Env, Error> {
        match stmt {
            Statement::FuncDef(name, body) => {
                self.collect_funcdef(env, name, body)
            }
            _ => Ok(env),
        }
    }

    pub fn collect_funcdef(
        &self,
        env: Env,
        name: &'static str,
        body: Box<Statement>,
    ) -> Result<Env, Error> {
        match env.funcs.get(name) {
            Some(_) => {
                return Err(Error::VarAlreadyExists(name));
            }
            None => {
                let mut new_env = env.clone();
                new_env.funcs.insert(name, StoreVal::FuncPtr(body.clone()));
                Ok(new_env)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_funcdef() {
        let coll = Collector::new();
        let body = Box::new(Statement::Assignment("x", RVal::Num(5)));
        let stmt = Statement::FuncDef("foo", body.clone());
        let res = coll.collect(Env::new(), stmt);

        let mut env = Env::new();
        env.funcs.insert("foo", StoreVal::FuncPtr(body));
        assert_eq!(res.unwrap(), env);
    }
}
