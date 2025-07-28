pub mod constraints;
pub mod error;
pub mod func_collect;
pub mod funcs;
pub mod interpret;
pub mod rewrite;
pub mod sig_collect;
pub mod sigs;
pub mod ssa;
pub mod statement;
pub mod traits;

use crate::constraints::ConstraintMap;
use crate::error::Error;
use crate::func_collect::FuncCollector;
use crate::funcs::Funcs;
use crate::interpret::Interpreter;
use crate::rewrite::Rewriter;
use crate::sig_collect::SigCollector;
use crate::sigs::Sigs;
use crate::ssa::{SSAChecker, Symbols};
use crate::statement::Statement;
use crate::traits::Traits;

pub struct SimpleInterp {
    ssa_checker: SSAChecker,
    func_collector: FuncCollector,
    sig_collector: SigCollector,
    interpreter: Interpreter,
    rewriter: Rewriter,
}

impl SimpleInterp {
    pub fn new() -> Self {
        Self {
            ssa_checker: SSAChecker::new(),
            func_collector: FuncCollector::new(),
            sig_collector: SigCollector::new(),
            interpreter: Interpreter::new(),
            rewriter: Rewriter::new(),
        }
    }

    pub fn interp(&self, mut stmt: Statement) -> Result<Statement, Error> {
        //println!("\nOriginal program statement: \n\n{:#?}", &stmt);

        let mut symbols = Symbols::new();
        let res1 = self.ssa_checker.check(&mut symbols, &stmt);
        if res1.is_err() {
            return Err(res1.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 1: SSA Check");
        //println!("-----------------------------------");
        //println!("\n1. General symbols set: \n\n{:#?}", &symbols);
        //println!("\n2. Original program statement");

        let mut funcs = Funcs::new();
        let res2 = self.func_collector.collect(&mut funcs, &stmt);
        if res2.is_err() {
            return Err(res2.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 2: Function Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table: \n\n{:#?}", &funcs);
        //println!("\n2. Original program statement");

        let mut sigs = Sigs::new();
        let res3 = self.sig_collector.collect(&funcs, &mut sigs);
        if res3.is_err() {
            return Err(res3.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 3: Signature Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Function signatures table: \n\n{:#?}", &sigs);

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        let res4 = self.interpreter.interp(
            &funcs,
            &mut cmap,
            &mut traits,
            None,
            &stmt,
        );
        if res4.is_err() {
            return Err(res4.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 4: Flow Interpretation");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!(
        //    "\n2. Flow-sensitive variable symbols table: \n\n{:#?}",
        //    &cmap
        //);
        //println!("\n3. Original program statement");

        let res5 = self
            .rewriter
            .rewrite(&funcs, &cmap, &sigs, &traits, None, &mut stmt, true);
        if res5.is_err() {
            return Err(res5.err().unwrap());
        }

        //println!("\n-----------------------------------");
        //println!("PHASE 5: Switch-Case Rewrite");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Flow-sensitive variable symbols table (from PHASE
        // 3)"); println!("\n3. (Maybe) modified program statement:
        // \n\n{:#?}", &stmt);

        Ok(stmt)
    }
}
