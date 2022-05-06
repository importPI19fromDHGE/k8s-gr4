# k8s-gr4

Praxisteil / Prüfung
Gruppe 4 Verteilte Systeme

Oliver, Benedict, Felix, Jonathan

## Voraussetzungen

- ein eingerichtetes Kubernetes Cluster
- ein Netzwerkplugin (wie bspw. `flannel`) muss auf dem Cluster installiert sein

## Deployment

- in `patch-deploy-nodes.yaml` die IP-Adressen der verfügbaren Nodes eintragen
- `deploy.sh` ausführen

## Was passiert beim Deployment?

Eine Reihe an `etcd`-Pods und -Services werden gestartet. 
Diese sorgen dafür, dass die Datenbanken sich später finden und synchronisieren können.

Es wird ein Deployment mit App-DB-Pods angelegt.
Hier wird die eigentliche Anwendung repliziert ausgeführt.

Für die Kommunikation mit den Anwendungen wird ein Service angelegt.
Damit dieser von außerhalb des Clusters erreichbar ist, wird zusätzlich ein Ingress angelegt.
Das Load-Balancing wird mithilfe eines haproxy-Controllers realisiert.