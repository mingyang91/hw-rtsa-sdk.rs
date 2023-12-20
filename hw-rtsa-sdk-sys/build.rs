extern crate bindgen;

use bindgen::callbacks::{ParseCallbacks, TypeKind};
use cmake::Config;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use fs_extra;

#[derive(Debug, Eq, PartialEq)]
struct DeriveSelector {
    kind: TypeKind,
    name: String,
}

impl core::hash::Hash for DeriveSelector {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.kind {
            TypeKind::Struct => "Struct".hash(state),
            TypeKind::Union => "Union".hash(state),
            TypeKind::Enum => "Enum".hash(state),
        };
        self.name.hash(state);
    }
}

impl DeriveSelector {
    fn new(kind: TypeKind, name: String) -> Self {
        Self { kind, name }
    }
}

#[derive(Debug)]
struct DeriveCallbacks {
    derives: HashMap<DeriveSelector, Vec<String>>,
}

impl DeriveCallbacks {
    fn new(derives: HashMap<DeriveSelector, Vec<String>>) -> Self {
        Self { derives }
    }
}

impl ParseCallbacks for DeriveCallbacks {
    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        let selector = DeriveSelector::new(info.kind, info.name.to_string());
        if let Some(matched) = self.derives.get(&selector) {
            matched.clone()
        } else {
            vec![]
        }
    }
}

fn derive_callbacks() -> DeriveCallbacks {
    let derives: HashMap<DeriveSelector, Vec<String>> = HashMap::from([
        (DeriveSelector::new(TypeKind::Struct,"huawei_rtsa_ProxyHandler".to_string()), vec!["Handler".to_string()])
    ]);
    DeriveCallbacks::new(derives)
}

fn main() {
    let target = env::var("TARGET").unwrap();
    // Link C++ standard library
    if let Some(cpp_stdlib) = get_cpp_link_stdlib(&target) {
        println!("cargo:rustc-link-lib=dylib={}", cpp_stdlib);
    }

    println!("cargo:rerun-if-changed=cpp/include/ffi.h");
    println!("cargo:rerun-if-changed=cpp/src/ffi.cpp");
    println!("cargo:rerun-if-changed=cpp/CMakeLists.txt");

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let cpp_root = out.join("cpp/");

    if !cpp_root.exists() {
        std::fs::create_dir_all(&cpp_root).unwrap();
        fs_extra::dir::copy("./cpp", &out, &Default::default()).unwrap_or_else(|e| {
            panic!(
                "Failed to copy cpp sources into {}: {}",
                cpp_root.display(),
                e
            )
        });
    }

    let sdk_include = "-Icpp/hw-rtsa-sdk/include";
    let bindings = bindgen::Builder::default()
        .header("cpp/include/ffi.h")
        .opaque_type("std::.*")
        .blocklist_type("rep")
        .blocklist_type("char_type")
        .blocklist_type("iterator")
        .blocklist_item("std::value")
        .blocklist_item("__gnu_cxx::__max")
        .blocklist_item("__gnu_cxx::__min")
        .clang_args(&["-x","c++","-std=c++17", "-I./cpp", sdk_include])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(derive_callbacks()))
        .generate();
    let b = bindings.expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    b.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let mut config = Config::new(&cpp_root);

    config
        .profile("Release")
        .very_verbose(true)
        .pic(true);

    let destination = config.build();

    if env::var("TARGET").unwrap().contains("window") {
        println!(
            "cargo:rustc-link-search={}",
            out.join("build").join("Release").display()
        );
    } else {
        println!("cargo:rustc-link-search={}", out.join("build").display());
    }
    println!("cargo:rustc-link-search=native={}", destination.display());
    println!("cargo:rustc-link-search={}", out.join("cpp/hw-rtsa-sdk/lib").display());
    println!("cargo:rustc-link-lib=dylib=ffi");
}

fn get_cpp_link_stdlib(target: &str) -> Option<&'static str> {
    if target.contains("msvc") {
        None
    } else if target.contains("apple") || target.contains("freebsd") || target.contains("openbsd") {
        Some("c++")
    } else if target.contains("android") {
        Some("c++_shared")
    } else {
        Some("stdc++")
    }
}