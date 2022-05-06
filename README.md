Verteilte Systeme - Kubernetes - Klausurersatzleistung Gruppe 4
==============================

# Autoren

- Oliver @rvnovae
- Benedict @b3n1d1c7
- Felix @photovoltex
- Jonathan @nukerxy

6. Semester, Matrikel 19, DHGE, 2022

# Quick-Start

```bash
git clone https://github.com/importPI19fromDHGE/k8s-gr4.git
```

# Dokumentation: Ein Kubernetes-Cluster einrichten

- Wie richtet man einen Kubernetes-Cluster ein?
- Kein Micro8ks, kein minikube, nur mit kubectl, kubelet und kubeadm.

## Systemumgebung

- leistungsstarke Ubuntu-Server 20.04 LTS (>32 GB RAM)
- Master-VM: dhge-bot12 23.88.43.92
- Worker-VM: dhge-bot11 142.132.225.82

Anleitung dafür: https://computingforgeeks.com/deploy-kubernetes-cluster-on-ubuntu-with-kubeadm/

Kubernetes-Repository hinzufügen (alle Nodes):
```bash
sudo apt -y install curl apt-transport-https
curl -s https://packages.cloud.google.com/apt/doc/apt-key.gpg | sudo apt-key add -
echo "deb https://apt.kubernetes.io/ kubernetes-xenial main" | sudo tee /etc/apt/sources.list.d/kubernetes.list
```
Pakete installieren (alle Nodes)
```bash
sudo apt update
sudo apt -y install vim git curl wget kubelet kubeadm kubectl
sudo apt-mark hold kubelet kubeadm kubectl
```
SWAP-Speicher deaktivieren (alle Nodes)
```bash
sudo sed -i '/ swap / s/^\(.*\)$/#\1/g' /etc/fstab
sudo swapoff -a
```
Kernel-Module aktivieren (alle Nodes)
```bash
# Kernel-Module aktivieren
sudo modprobe overlay
sudo modprobe br_netfilter

# sysctl konfigurieren
sudo tee /etc/sysctl.d/kubernetes.conf<<EOF
net.bridge.bridge-nf-call-ip6tables = 1
net.bridge.bridge-nf-call-iptables = 1
net.ipv4.ip_forward = 1
EOF

# systctl neu laden
sudo sysctl --system
```
Docker-Runtime bereitstellen (alle Nodes)
```bash
# Repository hinzufügen und Pakete installieren
sudo apt update
sudo apt install -y curl gnupg2 software-properties-common apt-transport-https ca-certificates
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo apt-key add -
sudo add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
sudo apt update
sudo apt install -y containerd.io docker-ce docker-ce-cli

# User der Docker-Gruppe hinzufügen TODO
sudo usermod -a -G docker $(id -u)
groups

# Verzeichnisse anlegen
sudo mkdir -p /etc/systemd/system/docker.service.d

# Daemon-Konfig
sudo tee /etc/docker/daemon.json <<EOF
{
  "exec-opts": ["native.cgroupdriver=systemd"],
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "100m"
  },
  "storage-driver": "overlay2"
}
EOF

sudo systemctl daemon-reload
sudo systemctl restart docker
sudo systemctl enable docker
```
K8s-Docker Interface installieren (alle Nodes)
> Anleitung: https://computingforgeeks.com/install-mirantis-cri-dockerd-as-docker-engine-shim-for-kubernetes/
```
# Hilfswerkzeuge installieren
sudo apt update
sudo apt install git wget curl

# Neueste dockerd-cri Version als Variable festlegen
VER=$(curl -s https://api.github.com/repos/Mirantis/cri-dockerd/releases/latest|grep tag_name | cut -d '"' -f 4)

# Binaries laden, entpacken und verschieben
wget https://github.com/Mirantis/cri-dockerd/releases/download/${VER}/cri-dockerd-${VER}-linux-amd64.tar.gz
tar xvf cri-dockerd-${VER}-linux-amd64.tar.gz
sudo mv cri-dockerd /usr/local/bin/

# systemd konfigurieren
wget https://raw.githubusercontent.com/Mirantis/cri-dockerd/master/packaging/systemd/cri-docker.service
wget https://raw.githubusercontent.com/Mirantis/cri-dockerd/master/packaging/systemd/cri-docker.socket
sudo mv cri-docker.socket cri-docker.service /etc/systemd/system/
sudo sed -i -e 's,/usr/bin/cri-dockerd,/usr/local/bin/cri-dockerd,' /etc/systemd/system/cri-docker.service

# Dienste neu starten
sudo systemctl daemon-reload
sudo systemctl enable cri-docker.service
sudo systemctl enable --now cri-docker.socket
```

## Master-Node initialisieren

Master: dhge-bot-12
```bash
# kubelet-Dienst aktivieren
sudo systemctl enable kubelet

# Notwendige Container laden
sudo kubeadm config images pull
```
```bash
# Die pod-network-cidr haben wir wegen unserem Netzwerkplugin Flannel so festgelegt.
kubeadm init --pod-network-cidr=10.244.0.0/24
```

**Am Ende der Ausgabe von kubeadm init steht  `kubeadm join ...`. Diesen Befehl abspeichern.**

kubectl soll ohne sudo ausführbar sein, auf Master-Node:
```bash
mkdir -p $HOME/.kube
sudo cp -f /etc/kubernetes/admin.conf $HOME/.kube/config
sudo chown $(id -u):$(id -g) $HOME/.kube/config
```
Ohne Netzworkplugin geht's nicht, z.B. Flanel installieren (Master-Node)
[Bitte nur ein Netzwerkplugin gleichzeitig installiert haben.](#zwei-netzwerk-plugins-sind-sehr-schlecht)
```bash
kubectl create -f https://github.com/flannel-io/flannel/blob/master/Documentation/kube-flannel.yml
```
Warten, bis alle Pods READY sind (auf Master-Node)
```bash
watch kubectl get pods --all-namespaces
```
**Ausführung des gespeicherten Befehls auf Worker-Nodes**
```bash
sudo kubeadm join ....
```
[Wenn da etwas schiefgeht.. ](#diagnose-am-cluster)

Warten, bis die Nodes ready sind. Dann ist der Cluster eingerichtet.
```bash
kubectl get nodes
```
Node wird nicht fertig? [Diagnosetools](#diagnose-am-cluster)

Es ist sinnvoll, ein worker-Label auf den Knoten zu platzieren
```bash
kubectl label node <node-name> node-role.kubernetes.io/worker=worker
```

# Debugging, wie man Probleme löst

Wenn Probleme bei kubeadm init/join auftreten: In die Logs schauen
```bash
sudo journalctl -u kubelet.service -e
```

## Diagnose am Cluster

```bash
# Zuordnung Pods zu Nodes
kubectl get pod -o=custom-columns=NAME:.metadata.name,STATUS:.status.phase,NODE:.spec.nodeName --all-namespaces
# (Fast) alles was so auf dem Cluster läuft
kubectl get all -A
kubectl get all -all-namespaces
# Ingress und Services zeigen
kubectl get service
kubectl get ingress
```
Logs/Events
```bash
kubectl describe pods
# shortform
kubectl logs <podname> <containername>
# Longform
kubectl logs -p <podname> -c <containername>
kubectl get events --all-namespaces  --sort-by='.metadata.creationTimestamp'
```

## Zwei Netzwerk-Plugins sind sehr schlecht

Wir hatten zeitweise sowohl flannel als auch calico installiert, das klappt nicht. [Also Calico wieder entfernen.](https://stackoverflow.com/questions/61672804/after-uninstalling-calico-new-pods-are-stuck-in-container-creating-state)
```bash
sudo rm /etc/cni/net.d/calico-kubeconfig
sudo rm /etc/cni/net.d/10-calico.conflist
```

## Zwei Kubernetes Installationen sind auch sehr schlecht

Wir hatten zuerst microk8s verwendet, das war auch recht zügig einsatzbereit. Bei einem Wechsel zum "händischen" K8s muss das aber wieder runter, sonst sind die Ports blockiert etc. etc. Das beißt sich!
```bash
microk8s delete nodes
microk8s kubectl stop nodes
microk8s kubectl
microk8s remove-node dhge-bot11
microk8s remove-node dhge-bot12
microk8s stop
```
ggf. sind weitere Schritte nötig. **Stackoverflow hilft gern!**

## Wenn man zu viel Zeit mit Debugging verbringt...

...hatte der Join-Token in der Zeit einen Timeout. Neuen Token erzeugen:
```bash
kubeadm token create --print-join-command
```

## lokale Docker-Images verteilen

Solange lokale Images genutzt werden, müssen alle Nodes dieses Image in ihrer Docker-Registry haben. Es wird manuell verteilt.
```bash
docker build <...>
docker save image:latest > image.tar
scp image.tar user@<workerip>:/path/to/destination/image.tar
ssh user@<workerip> docker load -i /path/to/destination/image.tar
```

### Zugriff Shell eines Container eines Pods

```bash
kubectl exec -it podname -c containername -- /bin/sh
```

## Cluster zurücksetzen

Wenn irgendetwas unerklärlich schiefgelaufen ist, um die meisten Konfigdateien zu löschen.
Am besten auf allen Nodes die folgenden Schritte:
```bash
sudo kubeadm reset
```
setzt den Cluster zurück. D.h auf den default-Stand vor kubeadm init. Wir mussten außerdem noch Rückstände in `/var/lib/cni/` entfernen.
```bash
sudo rm -rf /var/lib/cni/
```
```bash
systemctl daemon-reload
systemctl restart kubelet
```
```bash
iptables --list
```
#### **Vorsicht**
**Wenn man weiß, was man tut** (z.B. auf einer VM)

**Einträge löschen**
```bash
sudo iptables -F && sudo iptables -t nat -F && sudo iptables -t mangle -F && sudo iptables -X
```