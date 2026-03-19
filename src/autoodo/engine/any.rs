use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum Any {
    String(String),
    Int(i32),
    Float(f64),
    Bool(bool),
    List(Vec<Any>),
    Dict(HashMap<String, Any>),
    Set(HashSet<Any>),
}
