use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Enum(EnumDefinition),
    Path(Vec<String>),
    Ptr(Box<Type>),
    Reference(Box<Type>),
    Struct(StructDefinition),
    Tuple(Vec<Type>),
    Opaque(String),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct StructDefinition {
    generics: Vec<String>,
    fields: BTreeMap<String, Type>,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumDefinition {
    generics: Vec<String>,
    variants: BTreeMap<String, Type>,
}
