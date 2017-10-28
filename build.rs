extern crate bindgen;

use std::path::PathBuf;

fn main() {
    build_bcc_bindings();
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

fn build_bcc_bindings() {
    let mut bindings = bindgen::Builder::default().header("wrapper.h");

    for func in WHITELIST_FUNCTION {
        bindings = bindings.whitelist_function(func);
    }

    for ty in WHITELIST_TYPES {
        bindings = bindings.whitelist_type(ty);
    }

    for var in WHITELIST_VARS {
        bindings = bindings.whitelist_var(var);
    }

    // TODO: there's problem on formatting the generated patch.
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
        .rustfmt_bindings(false);

    let builder = bindings.generate().expect(
        "Should generate BCC API bindings OK",
    );

    builder.write_to_file("src/bccapi.rs").expect(
        "Couldn't write bcc bindings!",
    );
}
