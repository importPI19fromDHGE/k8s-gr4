# k8s-gr4

> TODO: \
> disconnect from database => currently connection is just dropped... not good \
> ![notLikeThisCat](https://cdn.discordapp.com/emojis/900341955244990504.webp?size=44&quality=lossless)

rust version of a simple todo rest-api using [splx](https://github.com/launchbadge/sqlx) and [actix-web](https://github.com/actix/actix-web)

---
As Setup is rustc/cargo required. It is recommended to follow this [Setup](https://www.rust-lang.org/tools/install).
> [rustc](https://www.rust-lang.org/) => compiler | [cargo](https://doc.rust-lang.org/cargo/guide/why-cargo-exists.html) => package manager and more

## Docker
Executing the `build` script will build the application in a new container with a specific image to build a [static linked executable](https://stackoverflow.com/questions/49098753/unable-to-run-a-docker-image-with-a-rust-executable). This insures that the executable can run under every environment (like an alpine image).
After the rust build is done the image is build via the `Dockerfile`.

With the `run` and `run-dev` script the container can be startet with the required predefined environment variables. These should be adjusted to the usage.
> `run-dev` uses the --network=host flag to access the localhost, useful when the db only runs locally

## Config
The configuration is fully configured via environment variables and every variable has to be provided except `K8SGR4_PORT` which defaults to `8080` if not provided
```
K8SGR4_LOG          = <debug level -> info | debug | trace | error>
K8SGR4_PORT         = <port for the rest-api to run on>
MARIADB_DATABASE    = <database name>
MARIADB_HOST        = <database host>:<database port>
MARIADB_USER        = <database username>
MARIADB_PASSWORD    = <database password>
MARIADB_TABLE       = <database table>
```
Following command is an example command to execute the application without global set environment variables:
```sh
K8SGR4_LOG=info \
K8SGR4_PORT=80 \
MARIADB_DATABASE=todolist \
MARIADB_HOST=localhost:3306 \
MARIADB_PASSWORD=some-pwd \
MARIADB_USER=some-user \
MARIADB_TABLE=main \
<executable>
```
In development the `.env` file can be edited and the variables set inside will be provided during runtime.

## Endpoints
> adjust url/port if using provided example curl commands (depending on environment variables)
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
    curl --header "Content-Type: application/json" --request "POST" http://localhost/ --data {\"content\":\"test-todo\"}
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
            --env MARIADB_USER=some-user \
            --env MARIADB_PASSWORD=some-pwd \
            --env MARIADB_ROOT_PASSWORD=root-pwd \
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
