use cbindgen::{Builder as CBindgenBuilder, Language};

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");

    static_vcruntime::metabuild();

    CBindgenBuilder::new()
        .with_crate(crate_dir)
        .with_language(Language::C)
        .generate()
        .expect("failed to generate header")
        .write_to_file("../ffi/declavatar.h");
}
