#!/bin/sh

kubectl apply -f https://raw.githubusercontent.com/haproxytech/kubernetes-ingress/v1.7/deploy/haproxy-ingress.yaml
kubectl patch deployment haproxy-kubernetes-ingress -n haproxy-controller --patch-file patch-ingress-controller.yaml
kubectl patch service haproxy-kubernetes-ingress -n haproxy-controller --patch-file patch-deploy-nodes.yaml
kubectl apply -f deployment.yaml
