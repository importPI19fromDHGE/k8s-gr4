

ssh rn@142.132.225.82

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

Quelle:
https://www.digitalocean.com/community/tutorials/how-to-install-mariadb-on-ubuntu-20-04-de

sudo mysql_secure_installation
   root password
   none
   yes remove anonymous
   yes
   yes
   yes

    show databases;

     create database todolist;

    use todolist; create table main(id int auto_increment, content varchar(1000) not null, primary key(id));