use crate::{Error, Funcs, SigVal};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Sigs {
    pub sigs: HashMap<SigVal, HashSet<&'static str>>,
}

impl Sigs {
    pub fn new() -> Self {
        Self {
            sigs: HashMap::<SigVal, HashSet<&'static str>>::new(),
        }
    }
}

pub struct SigCollector {}

impl SigCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(&self, funcs: &Funcs, sigs: &mut Sigs) -> Result<(), Error> {
        for (name, func) in funcs.funcs.iter() {
            let sig =
                SigVal::new(func.paramtypes.clone(), func.rettype.clone());
            match sigs.sigs.get(&sig) {
                Some(existing_funcs) => {
                    let mut func_names = existing_funcs.clone();
                    func_names.insert(name);
                    sigs.sigs.insert(sig, func_names);
                }
                None => {
                    let mut func_names = HashSet::<&'static str>::new();
                    func_names.insert(name);
                    sigs.sigs.insert(sig, func_names);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Statement::Assignment;
    use crate::{AssignmentRVal, FuncVal, RVal, Type};

    #[test]
    fn test_funcdef() {
        let body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let mut funcs = Funcs::new();
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, body));

        let mut sigs = Sigs::new();

        let sc = SigCollector::new();
        let res = sc.collect(&funcs, &mut sigs);

        let mut func_names = HashSet::new();
        func_names.insert("foo");
        let mut check_sigs = Sigs::new();
        check_sigs
            .sigs
            .insert(SigVal::new(vec![], None), func_names);

        assert_eq!(res.unwrap(), ());
        assert_eq!(sigs, check_sigs);
    }

    #[test]
    fn test_same_sigs() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let bar_body = Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        funcs
            .funcs
            .insert("foo", FuncVal::new(vec![], vec![], None, foo_body));
        funcs
            .funcs
            .insert("bar", FuncVal::new(vec![], vec![], None, bar_body));

        let mut sigs = Sigs::new();

        let sc = SigCollector::new();
        let res = sc.collect(&funcs, &mut sigs);

        let mut func_names = HashSet::new();
        func_names.insert("foo");
        func_names.insert("bar");
        let mut check_sigs = Sigs::new();
        check_sigs
            .sigs
            .insert(SigVal::new(vec![], None), func_names);

        assert_eq!(res.unwrap(), ());
        assert_eq!(sigs, check_sigs);
    }

    #[test]
    fn test_more_sigs() {
        let mut funcs = Funcs::new();
        let foo_body = Box::new(Assignment(
            "x",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let bar_body = Box::new(Assignment(
            "y",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        let baz_body = Box::new(Assignment(
            "z",
            Box::new(AssignmentRVal::RVal(RVal::Num(5))),
        ));
        funcs.funcs.insert(
            "foo",
            FuncVal::new(
                vec![Type::Int(), Type::Int()],
                vec!["a", "b"],
                Some(Box::new(Type::Int())),
                foo_body,
            ),
        );
        funcs.funcs.insert(
            "bar",
            FuncVal::new(
                vec![Type::Int(), Type::Int()],
                vec!["a", "b"],
                Some(Box::new(Type::Int())),
                bar_body,
            ),
        );
        let baz_funcarg_rettype = Box::new(Type::Int());
        funcs.funcs.insert(
            "baz",
            FuncVal::new(
                vec![Type::Func(SigVal::new(vec![],
                Some(baz_funcarg_rettype.clone())))],
                vec![],
                None,
                baz_body,
            ),
        );

        let mut sigs = Sigs::new();

        let sc = SigCollector::new();
        let res = sc.collect(&funcs, &mut sigs);

        let mut func_names = HashSet::new();
        func_names.insert("foo");
        func_names.insert("bar");
        let mut func_names2 = HashSet::new();
        func_names2.insert("baz");

        let mut check_sigs = Sigs::new();
        check_sigs.sigs.insert(
            SigVal::new(
                vec![Type::Int(), Type::Int()],
                Some(Box::new(Type::Int())),
            ),
            func_names,
        );
        check_sigs.sigs.insert(
            SigVal::new(
                vec![Type::Func(SigVal::new(vec![], Some(baz_funcarg_rettype)))],
                None,
            ),
            func_names2,
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(sigs, check_sigs);
    }
}
