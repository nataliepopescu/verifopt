use crate::error::Error;
use crate::funcs::Funcs;
use crate::statement::{FuncVal, Statement, Type};

pub struct FuncCollector {}

impl FuncCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(&self, funcs: &mut Funcs, stmt: &Statement) -> Result<(), Error> {
        match stmt {
            Statement::Sequence(stmt_vec) => self.collect_seq(funcs, stmt_vec),
            Statement::FuncDef(func) => self.collect_funcdef(
                funcs,
                func.name,
                &func.is_method,
                &func.params,
                &func.rettype,
                &*func.body,
            ),
            Statement::TraitImpl(trait_name, struct_name, func_impls) => {
                self.collect_trait_impl(funcs, trait_name, struct_name, func_impls)
            }
            _ => Ok(()),
        }
    }

    fn collect_seq(
        &self,
        funcs: &mut Funcs,
        stmt_vec: &Vec<Box<Statement>>,
    ) -> Result<(), Error> {
        for stmt in stmt_vec.iter() {
            self.collect(funcs, stmt)?;
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
        body: &Statement,
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
                            func_name,
                            *is_method,
                            params.clone(),
                            rettype.clone(),
                            Box::new(body.clone()),
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
        func_impls: &Vec<FuncVal>,
    ) -> Result<(), Error> {
        for func_impl in func_impls.iter() {
            let new_funcval = (
                Some((trait_name, struct_name)),
                FuncVal::new(
                    func_impl.name,
                    func_impl.is_method,
                    func_impl.params.clone(),
                    func_impl.rettype.clone(),
                    func_impl.body.clone(),
                ),
            );

            match funcs.funcs.get(func_impl.name) {
                Some(existing_funcs) => {
                    let mut updated_funcs = existing_funcs.clone();
                    updated_funcs.push(new_funcval);
                    funcs.funcs.insert(func_impl.name, updated_funcs.to_vec());
                }
                None => {
                    funcs.funcs.insert(func_impl.name, vec![new_funcval]);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod func_collect_tests;
