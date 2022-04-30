#!/bin/bash
# exec container from img
docker run --detach \
    -p 8080:8080 \
    --name k8s-gr4 \
    --env K8SGR4_LOG=info \
    --env K8SGR4_PORT=8080 \
    --env MARIADB_HOST=172.17.0.2:3306 \
    --env MARIADB_DATABASE=todolist \
    --env MARIADB_USER=gr4 \
    --env MARIADB_PASSWORD=Gruppe4PI19dhge2022 \
    --env MARIADB_TABLE=main \
    k8s-gr4/app-rust
