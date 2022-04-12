docker build -t k8s-gr4/app --rm .
docker run -p 80:8080 -d k8s-gr4/app