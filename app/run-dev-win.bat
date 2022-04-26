docker run --detach ^
    --name k8s-gr4-app-dev-win ^
    -p 8080:8080 ^
    -e K8SGR4_LOG=debug ^
    -e K8SGR4_SECRET=some-secret ^
    -e MARIADB_HOST=host.docker.internal:3306 ^
    -e MARIADB_DATABASE=todolist ^
    -e MARIADB_USER=some-user ^
    -e MARIADB_PASSWORD=some-pwd ^
    -e MARIADB_TABLE=main ^
    k8s-gr4/app-rust
