[![Build Status](https://travis-ci.org/rust-bpf/bcc-sys.svg?branch=master)](https://travis-ci.org/rust-bpf/bcc-sys)

Rust binding for [bcc](https://github.com/iovisor/bcc).

# Pre-requisites
* bcc must be installed before using this binding.

# Supported bcc versions

* Version 0.6.0 reflects the 0.5.0 version of bcc.
* Version 0.7.0 reflects the 0.6.1 version of bcc.
* Version 0.7.1 reflects the 0.6.1 version of bcc.
* Version 0.8.0 reflects bcc v0.4.0-v0.8.0 using feature flags.
* Version 0.9.0 reflects bcc v0.4.0-v0.9.0 using feature flags.
* Version 0.10.1 reflects bcc v0.4.0-v0.10.0 using feature flags.

**Note:** If you do not specify a version of bcc using feature flags, the
library will expect the latest version of bcc supported by the release of this
library. You must specify a version using the matching feature flag to override
this behavior.

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
