docker build -t k8smariadb .
docker run -d -v mydatabasevolume:/bitnami/mariadb/ k8smariadb:latest