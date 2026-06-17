fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::PathBuf::from(&out_dir);

    let bindings = if std::env::var_os("CARGO_FEATURE_SERIALIZE").is_some() {
        "bindings/plugin_h_serialize.rs"
    } else {
        "bindings/plugin_h.rs"
    };

    println!("cargo:rerun-if-changed={bindings}");

    std::fs::copy(bindings, out_path.join("plugin_h.rs"))
        .unwrap_or_else(|e| panic!("Could not copy vendored bindings {bindings}: {e}"));
}
