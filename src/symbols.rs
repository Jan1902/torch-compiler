use std::collections::HashMap;

pub struct Symbol {
    pub name: String,
    pub position: usize,
}

pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}