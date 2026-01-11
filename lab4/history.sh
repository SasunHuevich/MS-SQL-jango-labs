docker run -it -p 8000:8000 --name django-dev -v ./lab4:/lab4 -w /lab4 python:3.14-slim bash

pip install django

django-admin --version

python manage.py runserver 0.0.0.0:8000

apt update

apt install -y unixodbc unixodbc-dev curl gnupg

pip install django-mssql-backend pyodbc pytz tzdata

apt update && apt install -y curl gnupg2 apt-transport-https unixodbc unixodbc-dev

pip install django mssql-django pymssql

pip uninstall django-mssql-backend pyodbc -y

pip install mssql-django pymssql

pip install django-mssql

pip install mssql-django


######

docker run -it -p 8000:8000 --name django-dev -v ./lab4:/lab4 -w /lab4 python:3.14-slim bash

pip install django django-mssql-backend pytz pyodbc

apt-get update && apt-get install -y \
    unixodbc \
    libodbc2

#######

docker run -it -p 8000:8000 --name django-dev -v ./lab4:/lab4 -w /lab4 debian:12 bash

apt install curl -y

./install_ODBC.sh

apt update && apt upgrade -y

apt install -y python3 python3-pip python3-venv python3-dev build-essential libpq-dev default-libmysqlclient-dev

pip install --upgrade pip --break-system-packages

apt update && apt install -y \
    build-essential \
    python3-dev \
    default-libmysqlclient-dev \
    libpq-dev \
    pkg-config

pip install django django-mssql-backend psycopg2-binary mysqlclient --break-system-packages