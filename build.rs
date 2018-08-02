extern crate bindgen;

use std::process;

fn main() {
    println!("cargo:rustc-link-lib=bcc");
    // Uncomment below to update binding
    // build_bcc_bindings();
}

const WHITELIST_FUNCTION: &'static [&'static str] = &["bpf_.*", "bcc_.*", "perf_reader_.*"];

const WHITELIST_TYPES: &'static [&'static str] = &[
    "bcc_symbol",
    "perf_reader",
    "__sk_buff",
    "xdp_action",
    "xdp_md",
    "sk_action",
    "bpf_.*",
];

const WHITELIST_VARS: &'static [&'static str] = &[
    "LOG_BUF_SIZE",
    "BPF_.*",
    "MAX_BPF_REG",
    "MAX_BPF_ATTACH_TYPE",
    "__BPF_FUNC_MAPPER",
    "__BPF_ENUM_FN",
    "XDP_PACKET_HEADROOM",
    "TCP_BPF_IW",
    "TCP_BPF_SNDCWND_CLAMP",
    "STT_GNU_IFUNC",
];

const BLACKLIST_TYPES: &'static [&'static str] = &[
    // bindgen generates a misaligned type for this struct
    "bpf_raw_tracepoint_args",
];

fn build_bcc_bindings() {
    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I")
        .clang_arg(concat!(env!("CARGO_MANIFEST_DIR"), "/include"));

    for func in WHITELIST_FUNCTION {
        bindings = bindings.whitelist_function(func);
    }

    for ty in WHITELIST_TYPES {
        bindings = bindings.whitelist_type(ty);
    }

    for var in WHITELIST_VARS {
        bindings = bindings.whitelist_var(var);
    }

    for ty in BLACKLIST_TYPES {
        bindings = bindings.blacklist_type(ty);
    }

    bindings = bindings
        .derive_debug(true)
        .impl_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .impl_partialeq(true)
        .derive_eq(true)
        .derive_partialord(true)
        .derive_ord(true)
        .derive_hash(true)
        .rustfmt_bindings(true);

    let builder = bindings
        .generate()
        .expect("Should generate BCC API bindings OK");

    builder
        .write_to_file("src/bccapi.rs")
        .expect("Couldn't write bcc bindings!");
    let have_working_rustfmt = process::Command::new("rustup")
        .args(&["run", "nightly", "rustfmt", "--version"])
        .stdout(process::Stdio::null())
        .stderr(process::Stdio::null())
        .status()
        .ok()
        .map_or(false, |status| status.success());

    if !have_working_rustfmt {
        println!(
            "
        The latest `rustfmt` is required to format the generated bindings. Install
            `rustfmt` with:
            $ rustup component add rustfmt-preview
            $ rustup update
            "
        );
    }
}
