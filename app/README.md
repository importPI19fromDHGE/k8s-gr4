# k8s-gr4/app-rust

rust version of a simple todo rest-api using [splx](https://github.com/launchbadge/sqlx) and [actix-web](https://github.com/actix/actix-web)

---
For development rustc/cargo is required. It is recommended to follow this [Setup](https://www.rust-lang.org/tools/install).
> [rustc](https://www.rust-lang.org/) => compiler | [cargo](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) => package manager and more

## Docker
### Linux
Executing the `build` script will build the application in a new container with a specific image to build a [static linked executable](https://stackoverflow.com/questions/49098753/unable-to-run-a-docker-image-with-a-rust-executable). This insures that the executable can run under every environment (like an alpine image).
After the rust build is done the image is build via the `Dockerfile`.

With the `run` and `run-dev` scripts the container can be started with the required predefined environment variables. These should be adjusted for the usage.
> `run-dev` uses the --network=host flag to access the localhost, useful when the db only runs locally

### Windows
Almost the same. For Window the scripts `build-win.bat` and `run-dev-win.bat` are provided.
Windows can't use the network flag of docker. So it has to use `host.docker.internal` as `localhost` replacement (see in `run-dev-win.bat` => POOL_HOST).

## Config
The configuration is fully configured via environment variables and every `POOL_` variable has to be provided

All `APP_` variables have a default option (LOG = info, PORT = 80808, SECRET = )
```
APP_LOG          = <debug level -> info | debug | trace | error>
APP_PORT         = <port for the rest-api to run on>
APP_SECRET       = <secret to access the api>
POOL_DATABASE    = <database name>
POOL_HOST        = <database host>:<database port>
POOL_USER        = <database username>
POOL_PASSWORD    = <database password>
POOL_TABLE       = <database table>
```
The following command is an example command to execute the application without global environment variables in linux:
```sh
APP_LOG=info \
APP_PORT=8080 \
POOL_DATABASE=todolist \
POOL_HOST=localhost:3306 \
POOL_PASSWORD=some-pwd \
POOL_USER=some-user \
POOL_TABLE=main \
<executable>
```
In development the `.env` file can be edited and the variables set inside will be provided during runtime. This shouldn't be used for production usage.

## Endpoints
> adjust url/port if using provided example curl commands (depending on environment variables)

> when app is launched with APP_SECRET `--header "Authorized: <secret>"` should be provided with the configured secret
### `/`
----
### GET
- retrieves all entries
    ```sh
    curl http://localhost/
    ```
### POST
- inserts the provided json
- example curl (json needs to be escaped via curl)
    ```sh
    curl --header "Content-Type: application/json" --request "POST" http://localhost:8080/ --data {\"content\":\"test-todo\"}
    ```
    - spaces and surrounding quotation marks result in errors (seems more like a curl problem maybe)

### `/{id}`
----
### GET
- retrieves the item for the provided id
    ```sh
    curl http://localhost/1
    ```
### DELETE
- deletes the item for the provided id
    ```sh
    curl --request "DELETE" http://localhost/1
    ```

---
## Used dev setup
docker mariadb database
```sh
docker run --detach\
            --name some-mariadb \
            -p 3306:3306 \
            --env POOL_USER=some-user \
            --env POOL_PASSWORD=some-pwd \
            --env POOL_ROOT_PASSWORD=root-pwd \
            mariadb:latest
```
```sh
mysql -proot-pwd
mysql -u some-user -psome-pwd
```
mariadb simple config for some-user (for additional help see [db/doc](https://github.com/importPI19fromDHGE/k8s-gr4/tree/main/db/doku))
```sql
CREATE DATABASE todolist;
GRANT ALL PRIVILEGES ON todolist.* TO 'some-user'@'%' IDENTIFIED BY 'some-pwd';
FLUSH PRIVILEGES;
USE todolist;
CREATE TABLE main(id INT auto_increment, content VARCHAR(1000) NOT NULL, PRIMARY KEY(id));
```
### Tests
In a linux dev environment u can run `./test <url> <id> <secret>`:
```sh
./test localhost:8080 6 some-secret
```
The first parameter represents the url of the running container, the second parameter is the id to query and delete and the third the secret if configured.
