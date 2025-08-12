pub mod constraints;
pub mod display;
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
use crate::statement::RWStatement as RWS;
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

    pub fn interp(&self, stmt: Statement) -> Result<RWS, Error> {
        //println!("\nOriginal program statement: \n\n{:#?}", &stmt);

        let mut symbols = Symbols::new();
        self.ssa_checker.check(&mut symbols, &stmt)?;

        //println!("\n-----------------------------------");
        //println!("PHASE 1: SSA Check");
        //println!("-----------------------------------");
        //println!("\n1. General symbols set: \n\n{:#?}", &symbols);
        //println!("\n2. Original program statement");

        let mut funcs = Funcs::new();
        self.func_collector.collect(&mut funcs, &stmt)?;

        //println!("\n-----------------------------------");
        //println!("PHASE 2: Function Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table: \n\n{:#?}", &funcs);
        //println!("\n2. Original program statement");

        let mut sigs = Sigs::new();
        self.sig_collector.collect(&funcs, &mut sigs)?;

        //println!("\n-----------------------------------");
        //println!("PHASE 3: Signature Collection");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Function signatures table: \n\n{:#?}", &sigs);

        let mut cmap = ConstraintMap::new();
        let mut traits = Traits::new();
        self.interpreter
            .interp(&funcs, &mut cmap, &mut traits, None, &stmt)?;

        //println!("\n-----------------------------------");
        //println!("PHASE 4: Flow Interpretation");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!(
        //    "\n2. Flow-sensitive variable symbols table: \n\n{:#?}",
        //    &cmap
        //);
        //println!("\n3. Original program statement");

        let rw_stmt = self
            .rewriter
            .rewrite(&funcs, &cmap, &sigs, &traits, None, &stmt, true)?;

        //println!("\n-----------------------------------");
        //println!("PHASE 5: Switch-Case Rewrite");
        //println!("-----------------------------------");
        //println!("\n1. Function symbols table (from PHASE 2)");
        //println!("\n2. Flow-sensitive variable symbols table (from PHASE
        // 3)"); println!("\n3. (Maybe) modified program statement:
        // \n\n{:#?}", &stmt);

        Ok(rw_stmt)
    }
}
