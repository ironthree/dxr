#/usr/bin/bash

set -e

cargo publish --package dxr_shared
cargo publish --package dxr_derive
cargo publish --package dxr_server
cargo publish --package dxr_server_axum
cargo publish --package dxr_client
cargo publish --package dxr
