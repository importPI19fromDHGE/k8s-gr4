#!/bin/bash
# compile application in a seperate container with static linking (see image ekidd/rust-musl-builder)
if [ "$(docker container ls -a --filter name=k8s-gr4-app-builder -q)" = "" ]
then
    docker run -it \
        -v "$PWD":/app \
        -w /app \
        --name k8s-gr4-app-builder \
        ekidd/rust-musl-builder \
        cargo build --release
else
    docker start k8s-gr4-app-builder --attach
fi

# build the docker image
docker build -t k8s-gr4/app-rust --rm .
# save image to file
docker save k8s-gr4/app-rust -o k8s-gr4.tar
