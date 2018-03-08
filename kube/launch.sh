#!/bin/sh

kubectl create -f resources/server-service.yaml
kubectl create -f resources/server-pod.yaml
kubectl create -f resources/worker-deployment.yaml
