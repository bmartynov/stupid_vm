use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum SymbolType {
    Label(usize),
    Integer(i32),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub stype: SymbolType,
}

#[derive(Debug, Clone)]
pub struct SymbolTable(HashMap<String, Symbol>);

impl SymbolTable {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn add(&mut self, k: String, st: SymbolType) -> Option<Symbol> {
        self.0.insert(k, Symbol { stype: st })
    }
    pub fn get_offset(&self, k: &str) -> Option<usize> {
        let symbol = self.0.get(k)?;

        match symbol.stype {
            SymbolType::Label(offset) => Some(offset),
            _ => None,
        }
    }
    pub fn get_integer(&self, k: &str) -> Option<i32> {
        let symbol = self.0.get(k)?;

        match symbol.stype {
            SymbolType::Integer(value) => Some(value),
            _ => None,
        }
    }
}