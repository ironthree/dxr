#/usr/bin/bash

set -e

cargo publish --package dxr
cargo publish --package dxr_derive
cargo publish --package dxr_server
cargo publish --package dxr_server_axum
cargo publish --package dxr_client
