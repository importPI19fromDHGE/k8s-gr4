docker build -t k8s-gr4/db .
docker run -d -v mydatabasevolume:/bitnami/mariadb/ k8s-gr4/db:latest
