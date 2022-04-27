docker build -t k8s-gr4/db .
id=$(docker container ls -a --filter name=k8s-gr4-db-dev -q)
if [ "$id" != "" ]
then
    docker container stop $id
    docker container rm $id
fi
docker run -d -p 3306:3306 \
    --name k8s-gr4-db-dev \
    -v mydatabasevolume:/bitnami/mariadb/ k8s-gr4/db:latest
