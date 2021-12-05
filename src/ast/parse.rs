use crate::{ast::node::Type, set};
use std::{collections::BTreeSet, error::Error, fs::File, io::Read};
use syn;

pub fn parse_file(file_name: &str) -> Result<BTreeSet<Type>, Box<dyn Error>> {
    let mut file = File::open(file_name)?;
    let mut content = String::new();
    file.read_to_string(&mut content);

    let ast = syn::parse_file(&content)?;

    println!("{:?}", ast);
    Ok(set! {})
}

#[test]
fn test_parse_file() {
    assert_eq!(
        BTreeSet::new(),
        parse_file("/home/pmeredit/git/peragro/test/test.rs").unwrap()
    );
}
