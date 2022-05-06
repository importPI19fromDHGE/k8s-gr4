#!/bin/bash
# start minikube if not already running 
_=$(minikube status)
if [ $? -ne 0 ]
then minikube start
fi

cd app
./build.sh
echo Loading Image 'k8s-gr4/app-rust' into minikube...
minikube image load k8s-gr4/app-rust

cd ..
# uncomment if this is required -> install haproxy ingress
# kubectl apply -f https://raw.githubusercontent.com/haproxytech/kubernetes-ingress/v1.7/deploy/haproxy-ingress.yaml

kubectl create -f etcd-cluster.yml

kubectl delete -f deployment.yaml
kubectl create -f deployment.yaml

echo -e "\nService running on: http://$(minikube ip):$(kubectl get service k8sgr4-service -o=jsonpath='{.spec.ports[0].nodePort}')"
echo -e "\ton 'unable to connect' retry in 5 seconds..."
