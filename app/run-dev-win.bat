docker run --detach ^
    --name k8s-gr4-dev-win ^
    -p 8080:8080 ^
    --env K8SGR4_LOG=debug ^
    --env MARIADB_HOST=host.docker.internal:3306 ^
    --env MARIADB_DATABASE=todolist ^
    --env MARIADB_USER=some-user ^
    --env MARIADB_PASSWORD=some-pwd ^
    --env MARIADB_TABLE=main ^
    k8s-gr4/app-rust