use std::fmt;

use crate::statement::Statement::{
    Assignment, Conditional, FuncDef, InvokeFunc, InvokeTraitFunc, Print, Return,
    Sequence, Struct, Switch, TraitDecl, TraitImpl,
};
use crate::statement::{
    AssignmentRVal, BStatement, FuncDecl, FuncVal, RVal, Statement, Type,
};

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Assignment(var, value) => {
                write!(f, "let {} = {};", var, *value)
            }
            Print(s) => {
                write!(f, "println!(\"{}\");", s)
            }
            Sequence(stmt_vec) => {
                let mut s = format!("");
                for i in 0..stmt_vec.len() {
                    if i == 0 {
                        s = format!("{}{}", s, stmt_vec[i]);
                    } else {
                        s = format!("{}\n{}", s, stmt_vec[i]);
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
                if args.len() > 0 {
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
                let mut s = format!("");

                for (i, case) in case_vec.into_iter().enumerate() {
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
            Struct(name, field_types, field_names) => {
                let mut s = format!("struct {} {{", name);
                if field_types.len() > 0 {
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
                if func_decls.len() > 0 {
                    for func_decl in func_decls.iter() {
                        s = format!("{}\n{}", s, func_decl);
                    }
                }
                s = format!("{}\n}}", s);
                write!(f, "{}", s)
            }
            TraitImpl(tname, sname, func_impls) => {
                let mut s = format!("impl {} for {} {{", tname, sname);
                if func_impls.len() > 0 {
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
            _ => todo!(),
        }
    }
}

impl fmt::Display for AssignmentRVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AssignmentRVal::RVal(rval) => write!(f, "{}", rval),
            AssignmentRVal::Statement(statement) => {
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
                if field_values.len() > 0 {
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
        if self.params.len() > 0 {
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

impl fmt::Display for FuncVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("fn {}(", self.name);
        if self.is_method {
            s = format!("{}\n&self,", s);
        }
        if self.params.len() > 0 {
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
