use std::fmt;

use crate::statement::RWStatement::{
    Assignment, Conditional, FuncDef, InvokeFunc, InvokeTraitFunc, Print, Return,
    Sequence, Struct, Switch, TraitDecl, TraitImpl, TraitSwitch,
};
use crate::statement::{
    BStatement, FuncDecl, RVal, RWAssignmentRVal, RWFuncVal, RWStatement, Type,
};

impl fmt::Display for RWStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Assignment(var, value) => {
                write!(f, "let {} = {};", var, *value)
            }
            Print(s) => {
                write!(f, "println!(\"{}\");", s)
            }
            Sequence(stmt_vec) => {
                let mut s = String::new();
                for (i, stmt) in stmt_vec.iter().enumerate() {
                    if i == 0 {
                        s = format!("{}{}", s, stmt);
                    } else {
                        s = format!("{}\n{}", s, stmt);
                    }
                }
                write!(f, "{}", s)
            }
            Conditional(b, t_branch, f_branch) => {
                write!(f, "if {} {{\n{}\n}} else {{\n{}\n}}", b, t_branch, f_branch)
            }
            FuncDef(func) => write!(f, "{}", func),
            InvokeFunc(name, args) => {
                let mut s = format!("{}(", name);
                if !args.is_empty() {
                    for arg in args.iter() {
                        s = format!("{}\n{},", s, arg);
                    }
                    s = format!("{}\n);", s);
                } else {
                    s = format!("{});", s);
                }
                write!(f, "{}", s)
            }
            InvokeTraitFunc(name, ts_tup, args) => {
                let mut s = format!("<{} as {}>::{}(", ts_tup.1, ts_tup.0, name);
                if !args.is_empty() {
                    for arg in args.iter() {
                        s = format!("{}\n{},", s, arg);
                    }
                    s = format!("{}\n);", s);
                } else {
                    s = format!("{});", s);
                }
                write!(f, "{}", s)
            }
            Return(rval) => {
                write!(f, "return {};", rval)
            }
            Switch(rval, case_vec) => {
                // make just a long if statement
                let mut s = String::new();

                for (i, case) in case_vec.iter().enumerate() {
                    if i == 0 {
                        s = format!(
                            "{}if {} as usize == {} as usize {{\n{}\n}}",
                            s,
                            rval.clone(),
                            case.0,
                            case.1
                        );
                    } else {
                        s = format!(
                            "{}\nif {} as usize == {} as usize {{\n{}\n}}",
                            s,
                            rval.clone(),
                            case.0,
                            case.1
                        );
                    }
                }
                write!(f, "{}", s)
            }
            TraitSwitch(rval, case_vec) => {
                // make just a long if statement
                let mut s = format!("let typeid = {}.typeid();", rval);
                s = format!("{}\nlet rawptr = Box::into_raw({}) as *const ();", s, rval);

                for (i, case) in case_vec.iter().enumerate() {
                    if i == 0 {
                        s = format!("{}if typeid == Type::{} {{", s, case.0);
                        s = format!("{}\nunsafe {{", s);
                        s = format!(
                            "{}\nlet {}: &{} = std::mem::transmute::<*const (), &{}>(rawptr);",
                            s, rval, case.0, case.0
                        );
                        s = format!("{}\n{}", s, case.1);
                        s = format!("{}\n}}", s);
                        s = format!("{}\n}}", s);
                    } else {
                        s = format!("{}\nif typeid == Type::{} {{", s, case.0);
                        s = format!("{}\nunsafe {{", s);
                        s = format!(
                            "{}\nlet {}: &{} = std::mem::transmute::<*const (), &{}>(rawptr);",
                            s, rval, case.0, case.0
                        );
                        s = format!("{}\n{}", s, case.1);
                        s = format!("{}\n}}", s);
                        s = format!("{}\n}}", s);
                    }
                }
                write!(f, "{}", s)
            }
            Struct(name, field_types, field_names) => {
                let mut s = format!("struct {} {{", name);
                // add `typeid` field
                s = format!("{}\ntypeid: Type,", s);

                if !field_types.is_empty() {
                    for (field_name, field_type) in
                        std::iter::zip(field_names, field_types)
                    {
                        s = format!("{}\n{}: {},", s, field_name, field_type);
                    }
                }
                s = format!("{}\n}}", s);
                write!(f, "{}", s)
            }
            TraitDecl(name, func_decls) => {
                let mut s = format!("trait {} {{", name);
                // add `typeid()` decl
                s = format!("{}\nfn typeid(&self) -> Type;", s);

                // add remaining decls
                if !func_decls.is_empty() {
                    for func_decl in func_decls.iter() {
                        s = format!("{}\n{}", s, func_decl);
                    }
                }
                s = format!("{}\n}}", s);
                write!(f, "{}", s)
            }
            TraitImpl(tname, sname, func_impls) => {
                let mut s = format!("impl {} for {} {{", tname, sname);
                // add `typeid()` impl
                s = format!(
                    "{}\nfn typeid(&self) -> Type {{\nself.typeid.clone()\n}}",
                    s
                );

                // add remaining impls
                if !func_impls.is_empty() {
                    for func_impl in func_impls.iter() {
                        s = format!("{}\n{}", s, func_impl);
                    }
                }
                s = format!("{}\n}}", s);
                write!(f, "{}", s)
            }
            _ => todo!("stmt: {:?}", self),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Int() => write!(f, "usize"),
            Type::Struct(name) => write!(f, "{}", name),
            Type::DynTrait(name) => write!(f, "&dyn {}", name),
            _ => todo!("self: {:?}", &self),
        }
    }
}

impl fmt::Display for BStatement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BStatement::True() => write!(f, "true"),
            BStatement::False() => write!(f, "false"),
            BStatement::Not(b) => write!(f, "!{}", b),
            BStatement::Equals(v1, v2) => {
                write!(f, "{} == {}", v1, v2)
            }
            BStatement::TrueOrFalse() => write!(f, "TODO"),
        }
    }
}

impl fmt::Display for RWAssignmentRVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RWAssignmentRVal::RVal(rval) => write!(f, "{}", rval),
            RWAssignmentRVal::Statement(statement) => {
                write!(f, "{}", statement)
            }
        }
    }
}

impl fmt::Display for RVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RVal::Num(num) => write!(f, "{}", num),
            RVal::IdkNum() => write!(f, "IDK-Num"),
            RVal::Var(var) => write!(f, "{}", var),
            RVal::IdkVar() => write!(f, "IDK-Var"),
            RVal::Struct(name, field_values, field_names) => {
                let mut s = format!("{} {{", name);
                if !field_values.is_empty() {
                    for (field_name, field_value) in
                        std::iter::zip(field_names, field_values)
                    {
                        s = format!("{}\n{}: {},", s, field_name, field_value);
                    }
                    s = format!("{}\n", s);
                }
                s = format!("{}}}", s);
                write!(f, "{}", s)
            }
            RVal::IdkStruct(name) => write!(f, "IDK-Struct({})", name),
        }
    }
}

impl fmt::Display for FuncDecl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("fn {}(", self.name);
        if self.is_method {
            s = format!("{}\n&self,", s);
        }
        if !self.params.is_empty() {
            for (i, param) in self.params.iter().enumerate() {
                if self.is_method && i == 0 {
                    continue;
                }
                s = format!("{}\n{}: {},", s, param.0, param.1);
            }
            s = format!("{}\n)", s);
        } else {
            s = format!("{})", s);
        }
        if self.rettype.is_some() {
            s = format!("{} -> {};", s, self.rettype.as_ref().unwrap(),);
        } else {
            s = format!("{};", s);
        }
        write!(f, "{}", s)
    }
}

impl fmt::Display for RWFuncVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("fn {}(", self.name);
        if self.is_method {
            s = format!("{}\n&self,", s);
        }
        if !self.params.is_empty() {
            for (i, param) in self.params.iter().enumerate() {
                if self.is_method && i == 0 {
                    continue;
                }
                s = format!("{}\n{}: {},", s, param.0, param.1);
            }
            s = format!("{}\n)", s);
        } else {
            s = format!("{})", s);
        }
        if self.rettype.is_some() {
            s = format!(
                "{} -> {} {{\n{}\n}}",
                s,
                self.rettype.as_ref().unwrap(),
                self.body
            );
        } else {
            s = format!("{} {{\n{}\n}}", s, self.body);
        }
        write!(f, "{}", s)
    }
}
