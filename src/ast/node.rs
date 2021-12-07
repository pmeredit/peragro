use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    Enum(Items),
    Path(String),
    Ptr(Box<Type>),
    Reference(Box<Type>),
    Struct(Items),
    Tuple(Vec<Type>),
}

impl Type {
    pub fn generics(&self) -> Option<&[String]> {
        match self {
            Type::Struct(st) => Some(&st.generics),
            Type::Enum(e) => Some(&e.generics),
            _ => None,
        }
    }

    fn inst(&self, mut theta: BTreeMap<String, String>) -> Self {
        match self {
            Type::Enum(e) => Type::Enum(e.inst(theta)),
            Type::Path(p) => Type::Path(theta.remove(p).unwrap_or_else(|| p.clone())),
            Type::Ptr(t) => Type::Ptr(Box::new(t.inst(theta))),
            Type::Reference(t) => Type::Reference(Box::new(t.inst(theta))),
            Type::Struct(st) => Type::Struct(st.inst(theta)),
            Type::Tuple(tup) => Type::Tuple(
                tup.iter()
                    .cloned()
                    .map(|ty| ty.inst(theta.clone()))
                    .collect(),
            ),
        }
    }

    pub fn instantiate(&self, arguments: &[String]) -> Self {
        match self.generics() {
            None => self.clone(),
            Some(g) => {
                let theta = g
                    .iter()
                    .cloned()
                    .zip(arguments.iter().cloned())
                    .collect::<BTreeMap<String, String>>();

                self.inst(theta)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Items {
    generics: Vec<String>,
    items: BTreeMap<String, Type>,
}

impl Items {
    fn inst(&self, theta: BTreeMap<String, String>) -> Self {
        Self {
            generics: vec![],
            items: self
                .items
                .iter()
                .map(|(k, v)| (k.clone(), v.inst(theta.clone())))
                .collect(),
        }
    }
}
