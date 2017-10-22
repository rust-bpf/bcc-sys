#[macro_use]
extern crate lazy_static;
extern crate bindgen;

use std::path::Path;

lazy_static! {
    pub static ref ALL_HEADERS: Vec<&'static str> = vec![
        "/usr/include/bcc/libbpf.h",
        "/usr/include/bcc/bcc_syms.h",
        "/usr/include/bcc/perf_reader.h",
    ];
}

fn main() {
    build_bcc_bindings();
}

fn build_bcc_bindings() {
    for binding in ALL_HEADERS.iter() {
        let bindings = bindgen::Builder::default()
            .clang_arg("-include")
            .clang_arg("/usr/include/bcc/bpf_common.h")
            .header(*binding)
            .generate()
            .expect("Unable to generate bcc bindings");
        let trim_path = Path::new(binding.trim_left_matches("/usr/include/bcc/"));
        let stem = trim_path.file_stem().unwrap().to_str().unwrap();
        let mut path = String::new();
        path.push_str("./src/");
        path.push_str(stem);
        path.push_str(".rs");

        bindings
            .write_to_file(path)
            .expect("Couldn't write bcc bindings!");
    }
}
