#!/bin/bash

# Generates BCC bindings when used on Ubuntu.
#
# CAVEATS:
# - must be run from the root of the repository
# - written against Ubuntu 20.04 LTS
# - uses LLVM 7 by default, which can be tweaked below
# - forcefully tries to remove BCC libraries/headers (won't play nice with system-managed BCC)

LLVM="7"
VERSION=$1
VERSION_SAFE=$(echo "$VERSION" | tr . _)

# Install some pre-requisites.
sudo apt-get install linux-headers-"$(uname -r)"
sudo apt-get remove *llvm* *clang* *gtk*
sudo apt-get --yes install clang-"${LLVM}" libclang-"${LLVM}"-dev libelf-dev \
    libfl-dev llvm-"${LLVM}"-dev libz-dev llvm-"${LLVM}" cmake flex bison build-essential

# Clone/update to the correct version of BCC, including the libbpf submodule.
test -d bcc || git clone https://github.com/iovisor/bcc
pushd bcc && git checkout $VERSION && git submodule update

# Remove any installed versions of BCC.
sudo rm -rf /usr/include/bcc /usr/share/bcc \
    /usr/lib/python3/dist-packages/bcc \
    /usr/lib/python3/dist-packages/bcc* \
    /usr/lib/libbcc* \
    /usr/lib/x86_64-linux-gnu/libbcc* \
    /usr/lib/x86_64-linux-gnu/pkgconfig/libbcc.pc

# Build BCC.
sudo rm -rf _build; mkdir _build; pushd _build
cmake .. -DCMAKE_INSTALL_PREFIX=/usr
make -j24
sudo make install
popd

# Now generate the bindings, move them into place, and make sure tests pass.
popd
cargo build --features generate,${VERSION_SAFE} && mv src/generated.rs src/bccapi/${VERSION_SAFE}.rs
cargo test --features $VERSION_SAFE