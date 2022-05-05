#!/bin/sh

addresses='$(cat deploy-nodes)'
kubectl apply -f https://raw.githubusercontent.com/haproxytech/kubernetes-ingress/v1.7/deploy/haproxy-ingress.yaml
kubectl patch deployment haproxy-kubernetes-ingress -n haproxy-controller -p '{ "spec": { "template": { "containers": { "args": [ "--ingress.class=haproxy" ] } } } }'
kubectl patch service haproxy-kubernetes-ingress -n haproxy-controller --patch-file deploy-nodes.yaml
kubectl apply -f deployment.yaml
