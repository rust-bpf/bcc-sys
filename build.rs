#[allow(unused)]
const WHITELIST_FUNCTION: &'static [&'static str] = &["bpf_.*", "bcc_.*", "perf_reader_.*"];

#[allow(unused)]
const WHITELIST_TYPES: &'static [&'static str] = &[
    "bcc_symbol",
    "perf_reader",
    "__sk_buff",
    "xdp_action",
    "xdp_md",
    "sk_action",
    "bpf_.*",
];

#[allow(unused)]
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

#[allow(unused)]
const BLACKLIST_TYPES: &'static [&'static str] = &[
    // bindgen generates a misaligned type for this struct
    "bpf_raw_tracepoint_args",
];

fn main() {
    linking_info();

    #[cfg(feature = "generate")]
    {
        build_bcc_bindings();
    }
}

// define the function for when we will generate bindings
#[cfg(feature = "generate")]
fn build_bcc_bindings() {
    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I")
        .clang_arg(concat!(env!("CARGO_MANIFEST_DIR"), "/include"));

    if cfg!(any(
        feature = "v0_4_0",
        feature = "v0_5_0",
        feature = "v0_6_0",
        feature = "v0_6_1",
        feature = "v0_7_0",
        feature = "v0_8_0",
    )) {
        bindings = bindings.clang_arg("-D__BPF_COMMON__");
    } else {
        bindings = bindings.clang_arg("-D__BCC_COMMON__");
    }

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
        .write_to_file("src/generated.rs")
        .expect("Couldn't write bcc bindings!");
    let have_working_rustfmt = std::process::Command::new("rustup")
        .args(&["run", "rustfmt", "--version"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .ok()
        .map_or(false, |status| status.success());

    if !have_working_rustfmt {
        println!(
            "
        The latest `rustfmt` is required to format the generated bindings. Install
            `rustfmt` with:
            $ rustup component add rustfmt
            $ rustup update
            "
        );
    }
}

#[cfg(not(feature = "static"))]
fn linking_info() {
    println!("cargo:rustc-link-lib=bcc");
}

#[cfg(feature = "static")]
fn linking_info() {
    println!("cargo:rustc-link-lib=static=z");
    println!("cargo:rustc-link-lib=static=elf");
    println!("cargo:rustc-link-lib=static=xml2");
    println!("cargo:rustc-link-lib=static=lzma");
    println!("cargo:rustc-link-lib=static=tinfo");

    println!("cargo:rustc-link-lib=stdc++");

    println!("cargo:rustc-link-lib=static=bcc");
    println!("cargo:rustc-link-lib=static=bcc-loader-static");
    if cfg!(any(
        feature = "v0_10_0",
    )) {
        println!("cargo:rustc-link-lib=static=bcc_bpf");
    } else {
        println!("cargo:rustc-link-lib=static=bpf");
    }
    println!("cargo:rustc-link-lib=static=b_frontend");
    println!("cargo:rustc-link-lib=static=clang_frontend");
    println!("cargo:rustc-link-lib=static=usdt-static");

    llvm_static_linking();

    println!("cargo:rustc-link-lib=static=clangAnalysis");
    println!("cargo:rustc-link-lib=static=clangARCMigrate");
    println!("cargo:rustc-link-lib=static=clangAST");
    println!("cargo:rustc-link-lib=static=clangASTMatchers");
    println!("cargo:rustc-link-lib=static=clangBasic");
    println!("cargo:rustc-link-lib=static=clangCodeGen");
    println!("cargo:rustc-link-lib=static=clangDriver");
    println!("cargo:rustc-link-lib=static=clangDynamicASTMatchers");
    println!("cargo:rustc-link-lib=static=clangEdit");
    println!("cargo:rustc-link-lib=static=clangFormat");
    println!("cargo:rustc-link-lib=static=clangFrontend");
    println!("cargo:rustc-link-lib=static=clangFrontendTool");
    println!("cargo:rustc-link-lib=static=clangIndex");
    println!("cargo:rustc-link-lib=static=clangLex");
    println!("cargo:rustc-link-lib=static=clangParse");
    println!("cargo:rustc-link-lib=static=clangRewrite");
    println!("cargo:rustc-link-lib=static=clangRewriteFrontend");
    println!("cargo:rustc-link-lib=static=clangSema");
    println!("cargo:rustc-link-lib=static=clangSerialization");
    println!("cargo:rustc-link-lib=static=clangStaticAnalyzerCheckers");
    println!("cargo:rustc-link-lib=static=clangStaticAnalyzerCore");
    println!("cargo:rustc-link-lib=static=clangStaticAnalyzerFrontend");
    println!("cargo:rustc-link-lib=static=clangTooling");
    println!("cargo:rustc-link-lib=static=clangToolingCore");
    println!("cargo:rustc-link-lib=static=clangToolingRefactor");
}

// this function generates the linking info for llvm
#[allow(dead_code)]
fn llvm_static_linking() {
    let llvm_config = std::process::Command::new("llvm-config")
        .arg("--libs")
        .output()
        .expect("failed to run llvm-config");
    let llvm_libs = String::from_utf8(llvm_config.stdout)
        .expect("llvm-config output contains non-utf8 characters");
    let libs: Vec<&str> = llvm_libs.split_whitespace().collect();
    for lib in libs {
        let (_, name) = lib.split_at(2);
        println!("cargo:rustc-link-lib=static={}", name);
    }
}
