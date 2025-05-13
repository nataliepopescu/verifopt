pub mod func_collector;
pub mod interpreter;
pub mod translator;

#[cfg(test)]
mod tests {
    use super::func_collector::{Env, FuncCollector};
    use super::interpreter::{Interpreter, Statement, Store};
    //use super::*;

    #[test]
    fn test_print() {
        let stmt = Statement::Print("hello");

        let fc = FuncCollector::new();
        let fc_res = fc.collect(Env::new(), stmt.clone());

        let unwrapped_fc_res = fc_res.unwrap();
        let interp = Interpreter::new(unwrapped_fc_res.0);
        let i_res = interp.interp(Store::new(), unwrapped_fc_res.1);

        assert_eq!(i_res.unwrap(), (Store::new(), stmt));
    }
}
