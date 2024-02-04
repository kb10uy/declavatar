use std::{fs::read_to_string, path::PathBuf};

use declavatar::{
    avatar_v2::transform_avatar,
    decl_v2::{compile_declaration, DeclarationFormat, PreprocessData},
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

static TEST_PREPROCESS_DATA: Lazy<PreprocessData> = Lazy::new(|| {
    let symbols = vec!["declavatar-test", "out-of-unity"];
    let localizations = vec![
        ("cargo-pkg-version", env!("CARGO_PKG_VERSION")),
        ("cargo-pkg-authors", env!("CARGO_PKG_AUTHORS")),
    ];
    PreprocessData {
        symbols: symbols.into_iter().map(|s| s.to_string()).collect(),
        localizations: localizations
            .into_iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect(),
    }
});

#[rstest]
fn compiles_all_sexpr_examples(#[files("../examples/sexpr/*.declisp")] filename: PathBuf) {
    let source = read_to_string(&filename).expect("source file should exist");

    println!("compiling {:?}", filename.canonicalize().unwrap());
    let decl_avatar = compile_declaration(&source, SEXPR_FORMAT.clone(), TEST_PREPROCESS_DATA.clone())
        .expect("declaration file load failure");

    let avatar = transform_avatar(decl_avatar);
    assert!(avatar.avatar.is_some());
    assert_eq!(avatar.logs, vec![]);
}
