use rustc_data_structures::fx::FxHashMap as HashMap;
use rustc_public::DefId;

pub struct FuncVal {}

pub struct FuncMap {
    pub all_fns: HashMap<DefId, FuncVal>,
}

impl FuncMap {
    pub fn new() -> FuncMap {
        Self {
            all_fns: HashMap::default(),
        }
    }
}

pub struct FuncCollectPass;

impl FuncCollectPass {
    pub fn new() -> FuncCollectPass {
        Self {}
    }

    pub fn run(&self, fmap: &mut FuncMap) {
    }

    fn collect_metadata(&self, fmap: &mut FuncMap) {
    }
}
