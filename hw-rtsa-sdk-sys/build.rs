use std::path::PathBuf;

fn main() {
    let lib = std::path::Path::new("hw-rtsa-sdk/lib").canonicalize().expect("sdk/lib not found");
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}", lib.to_str().expect("lib not found"));

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=bz2");

    let h_ext = std::ffi::OsStr::new("h");
    let include = std::path::Path::new("hw-rtsa-sdk/include/").read_dir().expect("sdk/include not found");
    let h_files: Vec<_> = include
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|f| f.is_file()).unwrap_or(false))
        .map(|entry| entry.path())
        .filter(|path| path.extension() == Some(h_ext))
        .collect();

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.

    let mut bindings = bindgen::Builder::default();

    for entry in h_files {
        let fullname = entry.canonicalize().expect("not found");
        let h_name = fullname.to_str().expect("to_str fail");

        // Tell cargo to invalidate the built crate whenever the wrapper changes
        println!("cargo:rerun-if-changed={}", h_name);
        bindings = bindings.header(h_name);
    }

    let bindings = bindings
        .vtable_generation(true)
        .clang_args(&["-x","c++","-std=c++17"])
        .opaque_type("std::.*")
        .blocklist_item("std::value")
        .blocklist_type("rep")
        .blocklist_type("char_type")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .size_t_is_usize(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR not found"));
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}