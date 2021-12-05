use crate::{ast::node::Type, set};
use std::{collections::BTreeMap, error::Error, fs::File, io::Read};
use syn;

// TODO: Support type aliases across files?
pub fn parse_file(file_name: &str) -> Result<BTreeMap<Vec<String>, Type>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let ast = syn::parse_file(&content)?;

    Ok(ast
        .items
        .iter()
        .filter_map(|x| match x {
            syn::Item::Type(t) => Some((vec![t.ident.to_string()], syn_to_type(&t.ty))),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>())
}

fn syn_to_type(ty: &syn::Type) -> Type {
    match ty {
        syn::Type::Array(_) => unimplemented!(),
        syn::Type::BareFn(_) => unimplemented!(),
        syn::Type::Group(_) => unimplemented!(),
        syn::Type::ImplTrait(_) => unimplemented!(),
        syn::Type::Infer(_) => unimplemented!(),
        syn::Type::Macro(_) => unimplemented!(),
        syn::Type::Never(_) => unimplemented!(),
        syn::Type::Paren(_) => unimplemented!(),
        // TODO: handle leading colon and generics
        syn::Type::Path(p) => Type::Path(
            p.path
                .segments
                .iter()
                .map(|x| x.ident.to_string())
                .collect(),
        ),
        syn::Type::Ptr(t) => Type::Ptr(Box::new(syn_to_type(&t.elem))),
        syn::Type::Reference(r) => Type::Reference(Box::new(syn_to_type(&r.elem))),
        syn::Type::Slice(_) => unimplemented!(),
        syn::Type::TraitObject(_) => unimplemented!(),
        syn::Type::Tuple(t) => Type::Tuple(t.elems.iter().map(|x| syn_to_type(x)).collect()),
        syn::Type::Verbatim(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

#[test]
fn test_parse_file() {
    assert_eq!(
        BTreeMap::new(),
        parse_file("/home/pmeredit/git/peragro/test/test.rs").unwrap()
    );
}
