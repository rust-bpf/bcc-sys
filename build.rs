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
        feature = "v0_4_0",
        feature = "v0_5_0",
        feature = "v0_6_0",
        feature = "v0_6_1",
        feature = "v0_7_0",
        feature = "v0_8_0",
        feature = "v0_9_0",
    )) {
        println!("cargo:rustc-link-lib=static=bpf");
    } else {
        println!("cargo:rustc-link-lib=static=bcc_bpf");
    }
    println!("cargo:rustc-link-lib=static=b_frontend");
    println!("cargo:rustc-link-lib=static=clang_frontend");
    println!("cargo:rustc-link-lib=static=usdt-static");

    println!("cargo:rustc-link-lib=static=LLVMLTO");
    println!("cargo:rustc-link-lib=static=LLVMPasses");
    println!("cargo:rustc-link-lib=static=LLVMObjCARCOpts");
    println!("cargo:rustc-link-lib=static=LLVMSymbolize");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoPDB");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoDWARF");
    println!("cargo:rustc-link-lib=static=LLVMFuzzMutate");
    println!("cargo:rustc-link-lib=static=LLVMMCA");
    println!("cargo:rustc-link-lib=static=LLVMTableGen");
    println!("cargo:rustc-link-lib=static=LLVMDlltoolDriver");
    println!("cargo:rustc-link-lib=static=LLVMLineEditor");
    println!("cargo:rustc-link-lib=static=LLVMXRay");
    println!("cargo:rustc-link-lib=static=LLVMOrcJIT");
    println!("cargo:rustc-link-lib=static=LLVMCoverage");
    println!("cargo:rustc-link-lib=static=LLVMMIRParser");
    println!("cargo:rustc-link-lib=static=LLVMBPFDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMBPFCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMBPFAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMBPFDesc");
    println!("cargo:rustc-link-lib=static=LLVMBPFInfo");
    println!("cargo:rustc-link-lib=static=LLVMObjectYAML");
    println!("cargo:rustc-link-lib=static=LLVMLibDriver");
    println!("cargo:rustc-link-lib=static=LLVMOption");
    println!("cargo:rustc-link-lib=static=LLVMWindowsManifest");
    println!("cargo:rustc-link-lib=static=LLVMTextAPI");
    println!("cargo:rustc-link-lib=static=LLVMX86Disassembler");
    println!("cargo:rustc-link-lib=static=LLVMX86AsmParser");
    println!("cargo:rustc-link-lib=static=LLVMX86CodeGen");
    println!("cargo:rustc-link-lib=static=LLVMGlobalISel");
    println!("cargo:rustc-link-lib=static=LLVMSelectionDAG");
    println!("cargo:rustc-link-lib=static=LLVMAsmPrinter");
    println!("cargo:rustc-link-lib=static=LLVMX86Desc");
    println!("cargo:rustc-link-lib=static=LLVMMCDisassembler");
    println!("cargo:rustc-link-lib=static=LLVMX86Info");
    println!("cargo:rustc-link-lib=static=LLVMX86Utils");
    println!("cargo:rustc-link-lib=static=LLVMMCJIT");
    println!("cargo:rustc-link-lib=static=LLVMInterpreter");
    println!("cargo:rustc-link-lib=static=LLVMExecutionEngine");
    println!("cargo:rustc-link-lib=static=LLVMRuntimeDyld");
    println!("cargo:rustc-link-lib=static=LLVMCodeGen");
    println!("cargo:rustc-link-lib=static=LLVMTarget");
    println!("cargo:rustc-link-lib=static=LLVMCoroutines");
    println!("cargo:rustc-link-lib=static=LLVMipo");
    println!("cargo:rustc-link-lib=static=LLVMInstrumentation");
    println!("cargo:rustc-link-lib=static=LLVMVectorize");
    println!("cargo:rustc-link-lib=static=LLVMScalarOpts");
    println!("cargo:rustc-link-lib=static=LLVMLinker");
    println!("cargo:rustc-link-lib=static=LLVMIRReader");
    println!("cargo:rustc-link-lib=static=LLVMAsmParser");
    println!("cargo:rustc-link-lib=static=LLVMInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMBitWriter");
    println!("cargo:rustc-link-lib=static=LLVMAggressiveInstCombine");
    println!("cargo:rustc-link-lib=static=LLVMTransformUtils");
    println!("cargo:rustc-link-lib=static=LLVMAnalysis");
    println!("cargo:rustc-link-lib=static=LLVMProfileData");
    println!("cargo:rustc-link-lib=static=LLVMObject");
    println!("cargo:rustc-link-lib=static=LLVMMCParser");
    println!("cargo:rustc-link-lib=static=LLVMMC");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoCodeView");
    println!("cargo:rustc-link-lib=static=LLVMDebugInfoMSF");
    println!("cargo:rustc-link-lib=static=LLVMBitReader");
    println!("cargo:rustc-link-lib=static=LLVMBitstreamReader");
    println!("cargo:rustc-link-lib=static=LLVMCore");
    println!("cargo:rustc-link-lib=static=LLVMBinaryFormat");
    println!("cargo:rustc-link-lib=static=LLVMSupport");
    println!("cargo:rustc-link-lib=static=LLVMDemangle");
    println!("cargo:rustc-link-lib=static=LLVMRemarks");

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
}
