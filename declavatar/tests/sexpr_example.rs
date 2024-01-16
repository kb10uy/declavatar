use std::{fs::read_to_string, path::PathBuf};

use declavatar::{
    avatar_v2::transform_avatar,
    decl_v2::{load_declaration, DeclarationFormat, PreprocessData},
};
use once_cell::sync::Lazy;
use pretty_assertions::assert_eq;
use rstest::*;

static SEXPR_FORMAT: Lazy<DeclarationFormat> = Lazy::new(|| {
    let extension_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../declavatar-extra/sexpr")
        .canonicalize()
        .expect("should be valid path");
    DeclarationFormat::Sexpr(vec![extension_dir])
});

#[rstest]
fn compiles_all_sexpr_examples(#[files("../examples/sexpr/*.declisp")] filename: PathBuf) {
    let source = read_to_string(&filename).expect("source file should exist");

    println!("compiling {:?}", filename.canonicalize().unwrap());
    let decl_avatar = load_declaration(&source, SEXPR_FORMAT.clone(), PreprocessData::default())
        .expect("declaration file load failure");

    let avatar = transform_avatar(decl_avatar);
    assert!(avatar.avatar.is_some());
    assert_eq!(avatar.logs, vec![]);
}
