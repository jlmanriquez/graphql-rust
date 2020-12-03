## Configuración de la Base de Datos Postgres
Se utiliza imagen de postgres sobre docker. Para correr esta imágen ejecutar el comando:
```
docker run -p 5432:5432 --name postgres -e POSTGRES_PASSWORD=dbpass -d postgres
```
El comando anterior descargará la ultima versión de Postgres y ejecutará el contenedor con nombre *'postgres'* y se asiganrá la password *'dbpass'*.

Para acceder a postgres se debe ejecutar lo siguiente:
```
docker exec -it postgres psql -U postgres hackernews
```
Crear la base de datos
```
CREATE DATABASE hackernews;
```
## Diesel
Se instala el CLI de *diesel* con las caracteristicas para postgres
```
cargo install diesel_cli --no-default-features --features postgres
```
Una vez instalado se configura en el archivo .env dentro del directorio raiz del proyecto la variable de entorno con la ruta a la base de datos.
```
echo DATABASE_URL=postgres://localhost/hackernews > .env
```
Se generan los scripts requeridos.
```
diesel setup
diesel migration generate create_users
diesel migration generate create_links
```
Se ejecuta la migración
```
diesel migration run
```
