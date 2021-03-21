#!/usr/bin/bash

UBUNTU_VERSION=${UBUNTU_VERSION:-"20.04"}

if [ ! -f wrapper.h ]; then
    echo "docker_gen.sh must be called from the root of the repository!"
    exit 1
fi

if [ -z "$BCC_VERSION" ]; then
    echo << 'EOF'
        BCC version must be specified.

        This should be the base name of the bcc build script.  For example, "v0_18_0" would map to
        build/bcc/v0_18_0.sh.  This script is called in the context of an Ubuntu container that is
        executing from the roof of this repository.
EOF
    exit 1
fi

BCC_BUILD_SCRIPT="/src/build/bcc/${BCC_VERSION}.sh"

docker_name="bcc-sys-bindgen-${BCC_VERSION}"
docker run --name $docker_name \
    --rm \
    --mount type=bind,source="$(pwd)",target=/src \
    -e "RUST_LOG=debug" \
    ubuntu:${UBUNTU_VERSION} \
    $BCC_BUILD_SCRIPT