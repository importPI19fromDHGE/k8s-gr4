FROM bitnami/mariadb-galera:latest
ENV MARIADB_ROOT_PASSWORD=root-pwd
ENV MARIADB_ROOT_USER=root
ENV MARIADB_USER=some-user
ENV MARIADB_PASSWORD=some-pwd
ENV MARIADB_DATABASE=todolist
ENV MARIADB_GALERA_MARIABACKUP_PASSWORD=backupssindnichtueberbewertet
WORKDIR /docker-entrypoint-initdb.d
COPY schema.sql .
WORKDIR /etc
COPY ./my.cnf /etc/my.cnf
