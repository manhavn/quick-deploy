#!/bin/bash
# shellcheck disable=SC2164
cd "$(dirname "$0")"

IMAGE_NAME="docker.io/manhavn/rust-quick-deploy:latest"

podman manifest create $IMAGE_NAME

podman build --arch amd64 \
  --manifest $IMAGE_NAME -f build.Dockerfile .

podman manifest push $IMAGE_NAME $IMAGE_NAME
# podman manifest rm $IMAGE_NAME
# podman image prune -f

podman run --rm --name quick-deploy -p 8080:8080 -v $(pwd)/frontend:/frontend -v $(pwd)/env:/env -it $IMAGE_NAME

# curl http://localhost:8080/

podman image rm $IMAGE_NAME

podman manifest rm $IMAGE_NAME
podman image prune -f
