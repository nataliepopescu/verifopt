use crate::{Error, FuncName, Funcs, SigVal};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct Sigs {
    pub sigs: HashMap<SigVal, HashSet<FuncName>>,
}

impl Sigs {
    pub fn new() -> Self {
        Self {
            sigs: HashMap::<SigVal, HashSet<FuncName>>::new(),
        }
    }
}

pub struct SigCollector {}

impl SigCollector {
    pub fn new() -> Self {
        Self {}
    }

    pub fn collect(&self, funcs: &Funcs, sigs: &mut Sigs) -> Result<(), Error> {
        for (func_name, (tso, func)) in funcs.funcs.iter() {
            let (_, paramtypes): (Vec<&'static str>, Vec<crate::Type>) = func.params.clone().into_iter().unzip();
            let sig =
                SigVal::new(paramtypes, func.rettype.clone());
            match sigs.sigs.get(&sig) {
                Some(existing_funcs) => {
                    let mut func_names = existing_funcs.clone();
                    func_names.insert(func_name);
                    sigs.sigs.insert(sig, func_names);
                }
                None => {
                    let mut func_names = HashSet::<&'static str>::new();
                    func_names.insert(func_name);
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
    use crate::Statement::{Assignment, Print, Sequence};
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
            .insert("foo", (None, FuncVal::new(vec![], None, body)));

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
        funcs.funcs.insert(
            "foo",
            (None, FuncVal::new(vec![], None, foo_body)),
        );
        funcs.funcs.insert(
            "bar",
            (None, FuncVal::new(vec![], None, bar_body)),
        );

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
            (
                None,
                FuncVal::new(
                    vec![("a", Type::Int()), ("b", Type::Int())],
                    Some(Box::new(Type::Int())),
                    foo_body,
                ),
            ),
        );
        funcs.funcs.insert(
            "bar",
            (
                None,
                FuncVal::new(
                    vec![("a", Type::Int()), ("b", Type::Int())],
                    Some(Box::new(Type::Int())),
                    bar_body,
                ),
            ),
        );
        let baz_funcarg_rettype = Box::new(Type::Int());
        funcs.funcs.insert(
            "baz",
            (
                None,
                FuncVal::new(
                    // FIXME func name?
                    vec![("func", Type::Func(vec![], Some(baz_funcarg_rettype.clone())))],
                    None,
                    baz_body,
                ),
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
                vec![Type::Func(vec![], Some(baz_funcarg_rettype))],
                None,
            ),
            func_names2,
        );

        assert_eq!(res.unwrap(), ());
        assert_eq!(sigs, check_sigs);
    }

    #[test]
    fn test_trait_impl() {
        let cat_speak_body = Box::new(Sequence(vec![Box::new(Print("meow"))]));
        let cat_funcimpl =
            FuncVal::new(vec![], None, cat_speak_body.clone());

        let mut funcs = Funcs::new();
        funcs
            .funcs
            .insert("speak", (Some(("Animal", "Cat")), cat_funcimpl.clone()));

        let mut sigs = Sigs::new();
        let sc = SigCollector::new();
        let res = sc.collect(&funcs, &mut sigs);

        let mut check_sigs = Sigs::new();
        check_sigs
            .sigs
            .insert(SigVal::new(vec![], None), HashSet::from(["speak"]));

        assert_eq!(res.unwrap(), ());
        assert_eq!(sigs, check_sigs);
    }
}
