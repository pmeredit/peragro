use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Struct(Struct),
    Enum(Enum),
    Opaque(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Struct {
    name_space: Vec<String>,
    name: String,
    fields: BTreeMap<String, Type>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Enum {
    name_space: Vec<String>,
    name: String,
    variants: BTreeMap<String, Type>,
}
