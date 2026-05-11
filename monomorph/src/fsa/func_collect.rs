use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::DefId;

pub struct FuncVal {}

pub struct FuncMap {
    pub all_fns: HashMap<DefId, FuncVal>,
}
