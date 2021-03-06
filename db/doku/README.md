# mariaDB standalone setup

## Verbindungsherstellung zur VM

```cmd
ssh rn@142.132.225.82
```

## Installation mariaDB

```txt
    4  sudo apt upgrade
    5  ps aux
    6  exit
    7  sudo apt update
    8  sudo apt install mariadb-server
    9  ps aux | grep maria
   10  ls
   11  cd ..
   12  ls
   13  cd..
   14  cd ..
   15  ls
   16  sudo apt install mariadb-server
   17  sudo apt --fix-broken install
   18  history
```

[Quelle 1](https://www.digitalocean.com/community/tutorials/how-to-install-mariadb-on-ubuntu-20-04-de)

### Security-Setup

```bash
sudo mysql_secure_installation
```

- root password
- none
- yes remove anonymous
- yes
- yes
- yes

## Konfiguration

### Anlegen einer Datenbank und Tabelle für die ToDo-Liste

```mysql
show databases;
create database todolist;
use todolist;
create table main(id int auto_increment, content varchar(1000) not null, primary key(id));
```

### Anlegen eines Benutzerkontos für Verwaltung der ToDo-Liste

Der Benutzer soll lediglich Listenelemente hinzufügen, löschen und anzeigen können.

```mysql
CREATE USER gr4@todolist IDENTIFIED BY 'Gruppe4!PI19#dhge';
GRANT SELECT, INSERT, DELETE ON todolist.main TO 'gr4'@'todolist';
exit;
```
Überprüfung des Benutzerlogins durch Eingabe von ``mysql -h 142.132.225.82 -u 'gr4' -p 'Gruppe4!PI19#dhge' 'todolist'`` schlägt fehl.

Das hat nicht geklappt, richtiger User:
```bash
sudo mariadb;
```

#### Anpassung der Benutzer-"Domäne"

```mysql
drop user gr4@todolist;
CREATE USER 'gr4'@localhost IDENTIFIED BY 'Gruppe4!PI19#dhge';
select user from mysql.user;
SHOW GRANTS FOR 'gr4'@localhost;
GRANT SELECT, INSERT, DELETE ON todolist.main TO 'gr4'@localhost;
exit;
```
```bash
mysql -u gr4 -p;
```
> Passwort eingeben

Der Login ist nun erfolgreich. Somit kann nachfolgend überprüft werden, ob der Benutzer ausschließlich die erlaubten Funktionen nutzen darf.

```mysql
use todolist;
insert into main (content) VALUES ('test');
select * from main;
delete from main where id=1;
create test;
> ERROR 1142 (42000): CREATE command denied to user 'gr4'@'localhost' for table 'test' # Command denied
```
## Netzkonfig

Aktuell ist die Datenbank nur lokal erreichbar. Um die Erreichbarkeit der mariaDB über das Netzwerk zu überprüfen wird konfiguriert, dass mariaDB auf allen IP-Adressen erreichbar ist.

```bash
sudo nano /etc/mysql/mariadb.conf.d/50-server.cnf
```
change
```txt
bind-address                              = 127.0.0.1
```
to
0.0.0.0 or public Ip
```bash
sudo systemctl restart mysql.service
sudo systemctl restart mariadb.service
```

[Quelle 2](https://www.digitalocean.com/community/tutorials/how-to-allow-remote-access-to-mysql)

```mysql
RENAME USER 'gr4'@localhost TO 'gr4'@'142.132.225.82';
```

# Docker

## Installation
[Quelle Docker](https://docs.docker.com/engine/install/ubuntu/)
[MariaDB Image](https://hub.docker.com/r/bitnami/mariadb-galera#configuration)

```bash
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg


 echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null


sudo apt-get install docker-ce docker-ce-cli containerd.io
sudo docker run hello-world
```

## mariaDB mit Docker

siehe Files

```bash
$ mkdir mariadbdocker
$ cd mariadbdocker/
$ touch Dockerfile
$ touch schema.sql
$ nano Dockerfile
```

```bash
$ sudo docker build -f Dockerfile .
$ sudo docker run 006fb496d230
$ sudo docker build -t mariadbtest .
$ sudo docker run -d mariadbtest:latest
$ sudo docker inspect -f '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}' determined_robinson
$ mysql -h 172.17.0.2 -u gr4 -p
```

```mysql
MariaDB [todolist]> create database secondary;
ERROR 1044 (42000): Access denied for user 'gr4'@'%' to database 'secondary'
```

Volume nutzen - Persistente Daten

```
sudo docker run -d -v mydatabasevolume:/bitnami/mariadb/ mariadbtest:latest
sudo docker inspect mydatabasevolume
```

Files executable nicht vergessen!

```bash
chmod +x ./deploy.sh
chmod +x gradlew #Gradle wurde zu einem früheren Zeitpunkt benutzt.
```

