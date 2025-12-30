use std::collections::HashMap;

pub struct Symbol {
    pub name: String,
    pub position: usize,
    pub source_id: usize,
    pub id: u32,
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
    pub next_id: u32,
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
            next_id: 0,
        }
    }

    fn next_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
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

    pub fn id_of(&self, name: &str) -> u32 {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return sym.id;
            }
        }
        
        panic!("Could not resolve symbol");
    }

    pub fn define(&mut self, name: &str, pos: usize, source_id: usize) -> Result<(), String> {
        let next_id = self.next_id();
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
                    id: next_id,
                },
            );

            Ok(())
        }
    }
}