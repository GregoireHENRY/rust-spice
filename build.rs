extern crate bindgen;

use std::env;
use std::path::PathBuf;

/// Return path to SPICE.
///
/// It checks in the environment variable `$PATH` the first path containing "cspice".
/// It should be the path to the CSPICE folder, example: "/home/username/softwares/cspice".
/// If CSPICE has been installed correctly, it should contains the folders "include" and "lib".
/// Please do not forget to rename the static library "cspice.a" into "libcspice.a", located inside
/// "cspice/lib".
fn get_spice_path() -> String {
    let system_path = env::var("PATH").unwrap();

    let spice_path = system_path
        .split(':')
        .filter(|path| path.contains("cspice"))
        .collect::<Vec<_>>();

    // Return the first path that contains "cspice".
    spice_path.first().unwrap().to_string()
}

fn main() {
    // Tell cargo to tell rustc to link static lib CSPICE.
    let spice_path = get_spice_path();
    println!("cargo:rustc-link-search=native={}/lib", spice_path);
    println!("cargo:rustc-link-lib=static=cspice");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Add CSPICE include folder.
        .clang_arg(format!("-I{}/include", spice_path))
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
