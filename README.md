[![Build Status](https://travis-ci.org/rust-bpf/bcc-sys.svg?branch=master)](https://travis-ci.org/rust-bpf/bcc-sys)

Rust binding for [bcc](https://github.com/iovisor/bcc).

# Pre-requisites
* bcc must be installed before using this binding.

# Supported bcc versions

bcc 0.4.0 - 0.23.0 are supported via feature flags.

**Note:** If you do not specify a version of bcc using feature flags, the
library will expect the latest version of bcc supported by the release of this
library. You must specify a version using the matching feature flag to override
this behavior.

# Static linking

This library allows static linking of bcc and other libraries required to write
BPF utilities in Rust without needing to have bpf/bcc libraries available at
runtime. This requires matching the llvm feature flag to your installed version
and having specific prerequisites available. See the [GitHub Workflow config]
for more details.

# Generating new bindings

Freshly generated bindings will sometimes be required:
* to support new versions of bcc
* to regenerate older bindings with a newer version of bindgen

## How to generate new bindings

Before you generate fresh bindings, you should have `rustfmt` installed on your
system. Install `rustfmt` with:
```
$ rustup component add rustfmt
$ rustup update
```

You may then build this crate using:
```cargo build --features generate```

The fresh bindings will be placed into `src/bccapi/generated.rs`

**NOTE:** you may need to make changes to `build.rs` and/or `wrapper.h`
to add support for a new version of bcc

## Adding newly generated bindings

To use the newly generated bindings, we must add them to the library.

* Move your generated bindings to a version specific module under `src/bccapi`.
Follow existing naming conventions.
* If you've added bindings for a new version:
 * Modify `src/bccapi/mod.rs` to add the new module to this library.
 * Modify `Cargo.toml` to define a feature for the new bcc version.

`rustfmt` is required to generate new bindings.

[GitHub Workflow config]: https://github.com/rust-bpf/bcc-sys/tree/.github/workflows/cargo.yml