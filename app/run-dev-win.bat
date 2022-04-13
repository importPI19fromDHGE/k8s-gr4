# exec container from img
docker run --detach -p 80:8080 --env K8SGR4_LOG=trace --env MARIADB_HOST=host.docker.internal:3306 --env MARIADB_DATABASE=todolist --env MARIADB_USER=some-user --env MARIADB_PASSWORD=some-pwd --env MARIADB_TABLE=main k8s-gr4/app-rust
