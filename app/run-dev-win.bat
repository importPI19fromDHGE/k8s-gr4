docker run --detach ^
    --name k8s-gr4-app-dev-win ^
    -p 8080:8080 ^
    -e APP_LOG=debug ^
    -e APP_SECRET=some-secret ^
    -e POOL_HOST=host.docker.internal:3306 ^
    -e POOL_DATABASE=todolist ^
    -e POOL_USER=some-user ^
    -e POOL_PASSWORD=some-pwd ^
    -e POOL_TABLE=main ^
    k8s-gr4/app-rust
