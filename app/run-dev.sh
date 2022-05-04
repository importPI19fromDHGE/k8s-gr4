#!/bin/bash
id=$(docker container ls -a --filter name=k8s-gr4-app-dev -q)
if [ "$id" != "" ]
then
    docker container stop $id
    docker container rm $id
fi

# exec container from image
docker run -p 8080:8080 \
    --detach \
    --add-host=host.docker.internal:host-gateway \
    --name k8s-gr4-app-dev \
    -e K8SGR4_LOG=debug \
    -e K8SGR4_SECRET=some-secret \
    -e MARIADB_HOST=host.docker.internal:3306 \
    -e MARIADB_DATABASE=todolist \
    -e MARIADB_USER=gr4 \
    -e MARIADB_PASSWORD=Gruppe4PI19dhge2022 \
    -e MARIADB_TABLE=main \
    k8s-gr4/app-rust
