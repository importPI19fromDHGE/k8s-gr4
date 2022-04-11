

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

     CREATE USER gr4@todolist IDENTIFIED BY 'Gruppe4!PI19#dhge';
     GRANT SELECT, INSERT, DELETE ON todolist.main TO 'gr4'@'todolist';
     exit;
     mysql -h 142.132.225.82 -u 'gr4' -p 'Gruppe4!PI19#dhge' 'todolist' # Login mit gr4 Nutzer funktioniert nicht
     exit;
sudo mariadb;
     drop user gr4@todolist
     CREATE USER 'gr4'@localhost IDENTIFIED BY 'Gruppe4!PI19#dhge';
     select user from mysql.user;
     SHOW GRANTS FOR 'gr4'@localhost;
     GRANT SELECT, INSERT, DELETE ON todolist.main TO 'gr4'@localhost;
     mysql -u gr4 -p;
     use todolist;
     insert into main (content) VALUES ('test');
     select * from main;
     delete from main where id=1;

ERROR 1142 (42000): CREATE command denied to user 'gr4'@'localhost' for table 'test' # Command denied