#!/bin/bash
# shellcheck disable=SC2164
cd "$(dirname "$0")"

ENV_FOLDER="env"
PROJECT_HANDLE="discount-app"
APP_HANDLE="quick-deploy"
DEPLOY_HOST="root@app-discount.shopone.ai"

ENV_APP_CONFIG="/srv/ENV_APP_CONFIG/$PROJECT_HANDLE/prod/$APP_HANDLE"
IMAGE_NAME="docker.io/manhavn/rust-quick-deploy:latest"

echo 'ssh' -p 22 $DEPLOY_HOST "mkdir -p $ENV_APP_CONFIG/env; mkdir -p $ENV_APP_CONFIG/frontend; ufw allow 9901"
ssh -p 22 $DEPLOY_HOST "mkdir -p $ENV_APP_CONFIG/env; mkdir -p $ENV_APP_CONFIG/frontend; ufw allow 9901"

echo 'scp' -P 22 $ENV_FOLDER/*.* $DEPLOY_HOST:$ENV_APP_CONFIG/env
scp -P 22 $ENV_FOLDER/*.* $DEPLOY_HOST:$ENV_APP_CONFIG/env

echo 'export' DOCKER_HOST="ssh://$DEPLOY_HOST:22"
export DOCKER_HOST="ssh://$DEPLOY_HOST:22"

docker stop "$APP_HANDLE-$PROJECT_HANDLE"
docker rm "$APP_HANDLE-$PROJECT_HANDLE"

docker run -d --name "$APP_HANDLE-$PROJECT_HANDLE" \
    -p 172.17.0.1:9901:8080 \
    -v $ENV_APP_CONFIG/frontend:/frontend \
    -v $ENV_APP_CONFIG/env:/env \
    -it $IMAGE_NAME

# curl -X PUT https://app-discount.shopone.ai/server/frontend/upload -H "Content-Type: multipart/form-data" -F "dist=@./dist.zip;type=application/zip"
