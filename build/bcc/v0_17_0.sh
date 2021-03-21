#!/usr/bin/bash -ev

set -e

# Get some configuration out of the way so it doesn't affect our apt-get usage below.
export DEBIAN_FRONTEND=noninteractive DEBCONF_NONINTERACTIVE_SEEN=true
cat >> preseed.txt << 'EOF'
tzdata tzdata/Areas select America
tzdata tzdata/Zones/America select New_York
EOF
debconf-set-selections preseed.txt

## Update apt
apt-get update
apt-get install -y curl git cmake build-essential bison python

## Install Rust.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup default stable

## Install kernel headers for matching version
#apt-get install -y linux-headers-"$(uname -r)"
apt-get remove -y *llvm* *clang* *gtk*
apt-get install -y clang-8 libclang-8-dev libelf-dev \
    libfl-dev llvm-8-dev libz-dev llvm-8

## build/install BCC
git clone https://github.com/iovisor/bcc || true
pushd bcc
git checkout v0.17.0
git submodule update

mkdir -p _build
pushd  _build
cmake .. -DCMAKE_INSTALL_PREFIX=/usr
make -j24
make install
popd
popd

# Now generate the bindings.
cd /src
RUST_BACKTRACE=1 cargo build --features generate