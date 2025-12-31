use std::collections::HashMap;

pub struct SymbolTable {
    pub map: HashMap<String, usize>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        for i in 0..=15 { map.insert(format!("R{}", i), i); }
        map.insert("SP".to_string(), 0);
        map.insert("LCL".to_string(), 1);
        map.insert("ARG".to_string(), 2);
        map.insert("THIS".to_string(), 3);
        map.insert("THAT".to_string(), 4);
        map.insert("SCREEN".to_string(), 16384);
        map.insert("KBD".to_string(), 24576);
        Self { map }
    }

    pub fn add_entry(&mut self, symbol: String, address: usize) {
        self.map.insert(symbol, address);
    }

    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<usize> {
        self.map.get(key).copied()
    }
}