#!/bin/bash
# shellcheck disable=SC2164
cd "$(dirname "$0")"

podman build --arch amd64 --tag localhost/rust-musl/builder:1.91-alpine -f create-image-builder.Dockerfile .

podman image prune -f
