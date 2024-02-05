use std::{fs::read_to_string, path::PathBuf};

use declavatar::{
    avatar_v2::Transformer,
    decl_v2::{compile_declaration, Arguments, DeclarationFormat},
};
use once_cell::sync::Lazy;
use pretty_assertions::assert_eq;
use rstest::*;

static TEST_ARGUMENTS: Lazy<Arguments> = Lazy::new(|| {
    let extension_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../extensions/sexpr")
        .canonicalize()
        .expect("should be valid path");

    let mut args = Arguments::new();
    args.add_library_path(extension_dir);
    args.define_symbol("declavatar-test");
    args.define_symbol("out-of-unity");
    args.define_localization("cargo-pkg-version", env!("CARGO_PKG_VERSION"));
    args.define_localization("cargo-pkg-authors", env!("CARGO_PKG_AUTHORS"));

    args
});

#[rstest]
fn compiles_all_sexpr_examples(#[files("../examples/sexpr/*.declisp")] filename: PathBuf) {
    let source = read_to_string(&filename).expect("source file should exist");

    println!("compiling {:?}", filename.canonicalize().unwrap());
    let decl_avatar =
        compile_declaration(&source, DeclarationFormat::Sexpr, TEST_ARGUMENTS.clone())
            .expect("declaration file load failure");

    let transformer = Transformer::new();
    let avatar = transformer.transform_avatar(decl_avatar);

    assert!(avatar.avatar.is_some());
    assert_eq!(avatar.logs, vec![]);
}
