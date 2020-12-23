extern crate bindgen;

use std::env;
use std::path::PathBuf;
use std::collections::HashSet;


#[derive(Debug)]
struct IgnoreMacros(HashSet<String>);

impl bindgen::callbacks::ParseCallbacks for IgnoreMacros {
    fn will_parse_macro(&self, name: &str) -> bindgen::callbacks::MacroParsingBehavior {
        if self.0.contains(name) {
            bindgen::callbacks::MacroParsingBehavior::Ignore
        } else {
            bindgen::callbacks::MacroParsingBehavior::Default
        }
    }
}

fn main() {
    pkg_config::Config::new().probe("libpjproject").unwrap();
    println!("cargo:rerun-if-changed=wrapper.h");

    let ignored_macros = IgnoreMacros(
        vec![
            "FP_NORMAL".into(),
            "FP_SUBNORMAL".into(),
            "FP_ZERO".into(),
            "FP_INFINITE".into(),
            "FP_NAN".into(),
            "IPPORT_RESERVED".into(),
        ]
        .into_iter()
        .collect(),
    );
    let bindings = bindgen::Builder::default()
        .clang_arg("-L/usr/local/lib")
        .clang_arg("-I/usr/local/include")
        .header("wrapper.h")
        .parse_callbacks(Box::new(ignored_macros))
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
