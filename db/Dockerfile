FROM bitnami/mariadb-galera:latest
ENV MARIADB_ROOT_PASSWORD=wirh4ben1SehrstarkesPasswort!!!
ENV MARIADB_ROOT_USER=admin MARIADB_USER=gr4
ENV MARIADB_PASSWORD=Gruppe4PI19dhge2022
ENV MARIADB_DATABASE=todolist
ENV MARIADB_GALERA_MARIABACKUP_PASSWORD=backupssindnichtueberbewertet
WORKDIR /docker-entrypoint-initdb.d
COPY schema.sql .
