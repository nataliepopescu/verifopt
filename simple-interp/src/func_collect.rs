use crate::statement::{FuncVal, Statement, Type};
use crate::error::Error;
use crate::funcs::Funcs;

pub struct FuncCollector {}

impl FuncCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(
        &self,
        funcs: &mut Funcs,
        stmt: &Statement,
    ) -> Result<(), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.collect_seq(funcs, stmt_vec),
            Statement::FuncDef(name, is_method, params, rettype, body) => self
                .collect_funcdef(funcs, name, is_method, params, rettype, body),
            Statement::TraitImpl(
                trait_name,
                struct_name,
                func_names,
                func_impls,
            ) => self.collect_trait_impl(
                funcs,
                trait_name,
                struct_name,
                func_names,
                func_impls,
            ),
            _ => Ok(()),
        }
    }

    fn collect_seq(
        &self,
        funcs: &mut Funcs,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter() {
            let res = self.collect(funcs, &*stmt);
            if res.is_err() {
                return res;
            }
        }
        Ok(())
    }

    fn collect_funcdef(
        &self,
        funcs: &mut Funcs,
        func_name: &'static str,
        is_method: &bool,
        params: &Vec<(&'static str, Type)>,
        rettype: &Option<Box<Type>>,
        body: &Box<Statement>,
    ) -> Result<(), Error> {
        match funcs.funcs.get(&func_name) {
            Some(_) => {
                panic!("SSA BUG: funcname {:?} already exists", &func_name)
            }
            None => {
                funcs.funcs.insert(
                    func_name,
                    vec![(
                        None,
                        FuncVal::new(
                            *is_method,
                            params.clone(),
                            rettype.clone(),
                            body.clone(),
                        ),
                    )],
                );

                self.collect(funcs, body)
            }
        }
    }

    fn collect_trait_impl(
        &self,
        funcs: &mut Funcs,
        trait_name: &'static str,
        struct_name: &'static str,
        func_names: &Vec<&'static str>,
        func_impls: &Vec<FuncVal>,
    ) -> Result<(), Error> {
        for (func_name, func_impl) in std::iter::zip(func_names, func_impls) {
            let new_funcval = (
                Some((trait_name, struct_name)),
                FuncVal::new(
                    func_impl.is_method,
                    func_impl.params.clone(),
                    func_impl.rettype.clone(),
                    func_impl.body.clone(),
                ),
            );

            match funcs.funcs.get(func_name) {
                Some(existing_funcs) => {
                    let mut updated_funcs = existing_funcs.clone();
                    updated_funcs.push(new_funcval);
                    funcs.funcs.insert(func_name, updated_funcs.to_vec());
                }
                None => {
                    funcs.funcs.insert(func_name, vec![new_funcval]);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::BooleanStatement;
    use crate::Statement::{
        Assignment, Conditional, FuncDef, InvokeFunc, Print, Return, Sequence,
        Struct, TraitDecl, TraitImpl,
    };
    use crate::{AssignmentRVal, FuncDecl, FuncVal, RVal};

    #[test]
    fn test_print() {
        let fc = FuncCollector::new();
        let stmt = Print("hello");
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_assign_num() {
        let fc = FuncCollector::new();
        let stmt =
            Assignment("x", Box::new(AssignmentRVal::RVal(RVal::Num(5))));
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_seq() {
        let fc = FuncCollector::new();
        let stmt_vec = vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ))];
        let stmt = Sequence(stmt_vec);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_nested_seq() {
        let fc = FuncCollector::new();
        let stmt = Sequence(vec![
            Box::new(Sequence(vec![Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            ))])),
            Box::new(Sequence(vec![Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Num(6))),
            ))])),
        ]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_var_undef() {
        let fc = FuncCollector::new();
        let stmt = Sequence(vec![Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Var("y"))),
        ))]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_assign_var() {
        let fc = FuncCollector::new();
        let stmt = Sequence(vec![
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Num(5))),
            )),
            Box::new(Assignment(
                "y",
                Box::new(AssignmentRVal::RVal(RVal::Var("x"))),
            )),
        ]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let check_funcs = Funcs::new();
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_funcdef() {
        let fc = FuncCollector::new();
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt = FuncDef("foo", false, vec![], None, body.clone());
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body))],
        );
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_funcdef_args_do_nothing() {
        let fc = FuncCollector::new();
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt =
            FuncDef("foo", false, vec![("y", Type::Int())], None, body.clone());
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "foo",
            vec![(
                None,
                FuncVal::new(false, vec![("y", Type::Int())], None, body),
            )],
        );
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_assign_funcptr() {
        let fc = FuncCollector::new();
        let body = Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, body.clone())),
            Box::new(Assignment(
                "x",
                Box::new(AssignmentRVal::RVal(RVal::Var("foo"))),
            )),
        ]);
        let mut funcs = Funcs::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, body.clone()))],
        );
        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_nested_direct_calls_no_args() {
        let bar_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let foo_body = Box::new(Sequence(vec![
            Box::new(FuncDef("bar", false, vec![], None, bar_body.clone())),
            Box::new(InvokeFunc("bar", vec![])),
        ]));
        let stmt = Sequence(vec![
            Box::new(FuncDef("foo", false, vec![], None, foo_body.clone())),
            Box::new(InvokeFunc("foo", vec![])),
        ]);

        let mut funcs = Funcs::new();
        let fc = FuncCollector::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body))],
        );
        check_funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body))],
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
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

        let mut funcs = Funcs::new();
        let fc = FuncCollector::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "foo",
            vec![(None, FuncVal::new(false, vec![], None, foo_body.clone()))],
        );
        check_funcs.funcs.insert(
            "bar",
            vec![(None, FuncVal::new(false, vec![], None, bar_body.clone()))],
        );
        check_funcs.funcs.insert(
            "baz",
            vec![(None, FuncVal::new(false, vec![], None, baz_body.clone()))],
        );
        check_funcs.funcs.insert(
            "qux",
            vec![(None, FuncVal::new(false, vec![], None, qux_body.clone()))],
        );
        check_funcs.funcs.insert(
            "baz2",
            vec![(None, FuncVal::new(false, vec![], None, baz2_body.clone()))],
        );
        check_funcs.funcs.insert(
            "qux2",
            vec![(None, FuncVal::new(false, vec![], None, qux2_body.clone()))],
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_trait_impl() {
        let cat_speak_body = Box::new(Sequence(vec![Box::new(Print("meow"))]));

        let funcdef =
            FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);
        let cat_funcimpl = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let stmt = Sequence(vec![
            Box::new(TraitDecl("Animal", vec!["speak"], vec![funcdef.clone()])),
            Box::new(Struct("Cat", vec![], vec![])),
            Box::new(TraitImpl(
                "Animal",
                "Cat",
                vec!["speak"],
                vec![cat_funcimpl.clone()],
            )),
            Box::new(Assignment(
                "edgar",
                Box::new(AssignmentRVal::RVal(RVal::Struct("Cat", vec![]))),
            )),
        ]);

        let mut funcs = Funcs::new();
        let fc = FuncCollector::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "speak",
            vec![(Some(("Animal", "Cat")), cat_funcimpl.clone())],
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }

    #[test]
    fn test_dyn_traits_two_impl() {
        let funcdecl =
            FuncDecl::new(true, vec![("self", Type::DynTrait("Animal"))], None);

        let cat_speak_body = Box::new(Print("meow"));
        let cat_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Cat"))],
            None,
            cat_speak_body.clone(),
        );

        let dog_speak_body = Box::new(Print("woof"));
        let dog_speak = FuncVal::new(
            true,
            vec![("self", Type::Struct("Dog"))],
            None,
            dog_speak_body.clone(),
        );

        let gmaa = Box::new(Sequence(vec![Box::new(Conditional(
            Box::new(BooleanStatement::TrueOrFalse()),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Cat",
                vec![],
            )))])),
            Box::new(Sequence(vec![Box::new(Return(RVal::Struct(
                "Dog",
                vec![],
            )))])),
        ))]));

        let stmt = Sequence(vec![
            Box::new(TraitDecl(
                "Animal",
                vec!["speak"],
                vec![funcdecl.clone()],
            )),
            Box::new(FuncDef(
                "giveMeAnAnimal",
                false,
                vec![],
                Some(Box::new(Type::DynTrait("Animal"))),
                gmaa.clone(),
            )),
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
                "specific_animal",
                Box::new(AssignmentRVal::Statement(Box::new(
                    Statement::InvokeFunc("giveMeAnAnimal", vec![]),
                ))),
            )),
            Box::new(InvokeFunc("speak", vec!["specific_animal"])),
        ]);

        let mut funcs = Funcs::new();
        let fc = FuncCollector::new();
        let res = fc.collect(&mut funcs, &stmt);

        let mut check_funcs = Funcs::new();
        check_funcs.funcs.insert(
            "speak",
            vec![
                (Some(("Animal", "Cat")), cat_speak.clone()),
                (Some(("Animal", "Dog")), dog_speak.clone()),
            ],
        );
        check_funcs.funcs.insert(
            "giveMeAnAnimal",
            vec![(
                None,
                FuncVal::new(
                    false,
                    vec![],
                    Some(Box::new(Type::DynTrait("Animal"))),
                    gmaa.clone(),
                ),
            )],
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(funcs, check_funcs);
    }
}
