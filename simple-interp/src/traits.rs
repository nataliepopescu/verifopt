use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Traits {
    pub traits: HashMap<&'static str, Vec<&'static str>>,
}

impl Traits {
    pub fn new() -> Self {
        Self {
            traits: HashMap::new(),
        }
    }
}

