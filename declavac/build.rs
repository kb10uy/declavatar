use cbindgen::{Builder as CBindgenBuilder, Config as CBindgenConfig};
use csbindgen::Builder as CsBindgenBuilder;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let cbindgen_config =
        CBindgenConfig::from_file("../cbindgen.toml").expect("failed to read cbindgen.toml");

    static_vcruntime::metabuild();

    CBindgenBuilder::new()
        .with_crate(crate_dir)
        .with_config(cbindgen_config)
        .generate()
        .expect("failed to generate header")
        .write_to_file("../ffi/declavatar.h");

    CsBindgenBuilder::default()
        .input_extern_file("./src/lib.rs")
        .csharp_dll_name("declavac")
        .csharp_namespace("KusakaFactory.Declavatar")
        .generate_csharp_file("../ffi/NativeMethods.g.cs")
        .expect("failed to generate C# file");
}
