[![Build Status](https://travis-ci.org/rust-bpf/bcc-sys.svg?branch=master)](https://travis-ci.org/rust-bpf/bcc-sys)

Rust binding for [bcc](https://github.com/iovisor/bcc).

You need to install bcc before using this binding.

To format the generated bindings, you need to install rustfmt.

Install `rustfmt` with:
```
$ rustup component add rustfmt-preview
$ rustup update
```
* Version 0.6.0 reflects the 0.5.0 version of bcc.
* Version 0.7.0 reflects the 0.6.1 version of bcc.
* Version 0.7.1 reflects the 0.6.1 version of bcc.
* Version 0.8.0 reflects bcc v0.4.0-v0.8.0 using feature flags.
* Version 0.9.0 reflects bcc v0.4.0-v0.9.0 using feature flags.

**Note:** If you do not specify a version of bcc using feature flags, the
library will expect the latest version of bcc supported by the release of this
library. You may override this by using the corresponding feature flag to match
the bcc version you are using.
