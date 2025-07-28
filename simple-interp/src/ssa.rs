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
    fn from_iter<
        I: IntoIterator<Item = (&'static str, Option<Box<Symbols>>)>,
    >(
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
        if self.len() == 0 {
            return Ok(None);
        }
        if self.len() == 1 {
            return Ok(Some(self[0].clone()));
        }

        let mut merged = self[0].clone();
        for i in 1..self.len() {
            for (key, val) in self[i].clone().0.iter_mut() {
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

    pub fn check(
        &self,
        symbols: &mut Symbols,
        stmt: &Statement,
    ) -> Result<(), Error> {
        match stmt {
            Statement::Assignment(name, _) => {
                self.check_assignment(symbols, name)
            }
            Statement::Sequence(stmt_vec) => self.check_seq(symbols, stmt_vec),
            Statement::Conditional(_, true_branch, false_branch) => self
                .check_conditional(symbols, &(*true_branch), &(*false_branch)),
            Statement::Switch(_, vec) => self.check_switch(symbols, vec),
            Statement::FuncDef(name, _, params, _, body) => {
                self.check_funcdef(symbols, name, params, body)
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
        if symbols.0.get(name).is_some() {
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
            self.check(symbols, &(*stmt))?;
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
            self.check(&mut scoped_symbols, &(*switch_stmt))?;
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
        body: &Box<Statement>,
    ) -> Result<(), Error> {
        if symbols.0.get(name).is_some() {
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
        if symbols.0.get(struct_name).is_some() {
            return Err(Error::SymbolAlreadyExists(struct_name));
        }
        symbols.0.insert(struct_name, None);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::statement::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Sequence, Struct,
        Switch, TraitDecl, TraitImpl,
    };
    use crate::statement::{
        AssignmentRVal, BooleanStatement, FuncDecl, FuncVal, Type,
    };

    #[test]
    fn test_merge_vars() {
        let mut symb1 = Symbols::new();
        symb1.0.insert("x", None);
        let mut symb2 = Symbols::new();
        symb2.0.insert("x", None);
        let vec: Vec<Symbols> = vec![symb1, symb2];

        let mut end_symb = Symbols::new();
        end_symb.0.insert("x", None);
        assert_eq!(end_symb, vec.merge().unwrap().unwrap());
    }

    #[test]
    fn test_merge_same_funcs() {
        let mut body = Symbols::new();
        body.0.insert("x", None);
        let mut symb1 = Symbols::new();
        symb1.0.insert("foo", Some(Box::new(body.clone())));
        let mut symb2 = Symbols::new();
        symb2.0.insert("foo", Some(Box::new(body.clone())));
        let vec: Vec<Symbols> = vec![symb1, symb2];

        let mut end_symb = Symbols::new();
        end_symb.0.insert("foo", Some(Box::new(body)));
        assert_eq!(end_symb, vec.merge().unwrap().unwrap());
    }

    #[test]
    fn test_merge_err() {
        let mut body = Symbols::new();
        body.0.insert("x", None);
        let mut symb1 = Symbols::new();
        symb1.0.insert("foo", Some(Box::new(body.clone())));
        let mut symb2 = Symbols::new();
        symb2.0.insert("foo", None);
        let vec: Vec<Symbols> = vec![symb1, symb2];

        assert!(vec.merge().is_err());
    }

    #[test]
    fn test_assignment_err() {
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res, Err(Error::SymbolAlreadyExists("x")));
    }

    #[test]
    fn test_funcdef_args_do_nothing() {
        let body = Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt = Sequence(vec![Box::new(FuncDef(
            "foo",
            false,
            vec![("x", Type::Int())],
            None,
            body.clone(),
        ))]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let _ = sc.check(&mut symbols, &stmt);

        let mut check_symbols = Symbols::new();
        let mut inner_check_symbols = Symbols::new();
        inner_check_symbols.0.insert("x", None);
        inner_check_symbols.0.insert("y", None);
        check_symbols
            .0
            .insert("foo", Some(Box::new(inner_check_symbols)));

        assert_eq!(check_symbols, symbols);
    }

    #[test]
    fn test_funcdef_err() {
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res, Err(Error::SymbolAlreadyExists("foo")));
    }

    #[test]
    fn test_nested_funcdefs() {
        let body = Box::new(FuncDef(
            "baz",
            false,
            vec![],
            None,
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
        ));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, body.clone())),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let _ = sc.check(&mut symbols, &stmt);

        let mut check_symbols = Symbols::new();
        let mut body_check_symbols = Symbols::new();
        let mut baz_check_symbols = Symbols::new();
        baz_check_symbols.0.insert("x", None);
        body_check_symbols
            .0
            .insert("baz", Some(Box::new(baz_check_symbols)));
        check_symbols
            .0
            .insert("foo", Some(Box::new(body_check_symbols.clone())));
        check_symbols
            .0
            .insert("bar", Some(Box::new(body_check_symbols.clone())));

        assert_eq!(symbols, check_symbols);
    }

    #[test]
    fn test_conditional_ok() {
        let stmt = Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            )),
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
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(0))),
                )),
            ),
            (
                RVal::Num(5),
                Box::new(Assignment(
                    "y",
                    Box::new(AssignmentRVal::RVal(RVal::Num(1))),
                )),
            ),
        ];
        let stmt = Sequence(vec![
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(5))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Num(4))),
                )),
            )),
            Box::new(Switch(RVal::Var("x"), switch_vec)),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        assert_eq!(res.unwrap(), ());
    }

    #[test]
    fn test_nested_indirect_calls_no_args() {
        let baz_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(1))),
        ));
        let qux_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(2))),
        ));
        let baz2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(3))),
        ));
        let qux2_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(4))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz", false, vec![], None, baz_body.clone())),
            Box::new(FuncDef("qux", false, vec![], None, qux_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));
        let bar_body = Box::new(Sequence(vec![
            Box::new(FuncDef("baz2", false, vec![], None, baz2_body.clone())),
            Box::new(FuncDef("qux2", false, vec![], None, qux2_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("baz2"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("qux2"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]));

        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
            Box::new(Conditional(
                Box::new(BooleanStatement::TrueOrFalse()),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
                )),
                Box::new(Assignment(
                    "x",
                    Box::new(AssignmentRVal::RVal(RVal::Var("bar"))),
                )),
            )),
            Box::new(InvokeFunc("x", vec![])),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        let mut check_symbols = Symbols::new();
        let mut foo_symbols = Symbols::new();
        let mut bar_symbols = Symbols::new();
        let mut baz_symbols = Symbols::new();
        let mut qux_symbols = Symbols::new();
        let mut baz2_symbols = Symbols::new();
        let mut qux2_symbols = Symbols::new();

        baz_symbols.0.insert("x", None);
        qux_symbols.0.insert("x", None);
        baz2_symbols.0.insert("x", None);
        qux2_symbols.0.insert("x", None);

        foo_symbols.0.insert("baz", Some(Box::new(baz_symbols)));
        foo_symbols.0.insert("qux", Some(Box::new(qux_symbols)));
        foo_symbols.0.insert("x", None);

        bar_symbols.0.insert("baz2", Some(Box::new(baz2_symbols)));
        bar_symbols.0.insert("qux2", Some(Box::new(qux2_symbols)));
        bar_symbols.0.insert("x", None);

        check_symbols.0.insert("foo", Some(Box::new(foo_symbols)));
        check_symbols.0.insert("bar", Some(Box::new(bar_symbols)));
        check_symbols.0.insert("x", None);

        assert_eq!(res.unwrap(), ());
        assert_eq!(check_symbols, symbols);
    }

    #[test]
    fn test_dyn_traits_two_impl() {
        let funcdef = FuncDecl::new(
            true,
            vec![("animal", Type::DynTrait("Animal"))],
            None,
        );

        let cat_speak_body = Box::new(Print("meow"));
        let cat_speak = FuncVal::new(
            true,
            vec![("animal", Type::DynTrait("Animal"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Print("woof"));
        let dog_speak = FuncVal::new(
            true,
            vec![("animal", Type::DynTrait("Animal"))],
            None,
            dog_speak_body.clone(),
        );

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdef.clone()])),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(Struct("Dog", vec![], vec![])),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_speak.clone()],
            )),
            Box::new(TraitImpl(
                "Animal",
                "Dog",
                vec!["speak"],
                vec![dog_speak.clone()],
            )),
            Box::new(Assignment(
                "cat",
                Box::new(AssignmentRVal::RVal(RVal::Struct("Cat", vec![]))),
            )),
            Box::new(InvokeFunc("speak", vec!["cat"])),
        ]);

        let sc = SSAChecker::new();
        let mut symbols = Symbols::new();
        let res = sc.check(&mut symbols, &stmt);

        let mut check_symbols = Symbols::new();
        check_symbols.0.insert("Cat", None);
        check_symbols.0.insert("Dog", None);
        check_symbols.0.insert("cat", None);

        assert_eq!(res.unwrap(), ());
        assert_eq!(check_symbols, symbols);
    }
}
