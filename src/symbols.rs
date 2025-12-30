use std::collections::HashMap;

pub struct Symbol {
    pub name: String,
    pub position: usize,
    pub source_id: usize,
}

pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
    pub parent: Option<usize>,
}

impl Scope {
    pub fn new(parent: Option<usize>) -> Self {
        Scope {
            symbols: HashMap::new(),
            parent,
        }
    }
}

pub struct SymbolTable {
    pub scopes: Vec<Scope>,
    pub current: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        let root = Scope {
            symbols: HashMap::new(),
            parent: None,
        };

        SymbolTable {
            scopes: vec![root],
            current: 0,
        }
    }

    pub fn begin_scope(&mut self) {
        let parent = Some(self.current);
        let idx = self.scopes.len();

        self.scopes.push(Scope::new(parent));

        self.current = idx;
    }

    pub fn end_scope(&mut self) {
        let parent = self.scopes[self.current].parent;
        self.current = parent.unwrap_or(0);
    }

    pub fn resolve(&self, name: &str) -> Result<&Symbol, String> {
        let mut idx = self.current;

        loop {
            if let Some(sym) = self.scopes[idx].symbols.get(name) {
                return Ok(sym);
            }

            match self.scopes[idx].parent {
                Some(p) => idx = p,
                None => break,
            }
        }

        Err(format!("use of undeclared variable `{}`", name))
    }

    pub fn define(&mut self, name: &str, pos: usize, source_id: usize) -> Result<(), String> {
        let scope = &mut self.scopes[self.current];

        if scope.symbols.contains_key(name) {
            Err(format!("variable `{}` already declared in this scope", name))
        } else {
            scope.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    position: pos,
                    source_id: source_id,
                },
            );

            Ok(())
        }
    }
}