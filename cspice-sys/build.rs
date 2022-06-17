fn main() {
    #[cfg(feature = "generate")]
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    #[cfg(feature = "generate")]
    bindgen::Builder::default()
        .header("cspice/SpiceUsr.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .whitelist_function(".*_c")
        .whitelist_type("Spice.*")
        .whitelist_var("SPICE_.*")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join("cspice.rs"))
        .expect("Unable to save bindings");

    println!("cargo:rustc-link-search=native=cspice-sys/cspice/");
    println!("cargo:rustc-link-search=native=cspice/"); // cargo publish
    println!("cargo:rustc-link-lib=static=cspice");
}
