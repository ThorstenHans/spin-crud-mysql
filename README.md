# CRUD in WebAssembly with Fermyon Spin and MySQL

This respository contains a sample implementation to illustrate how to create a CRUD API in WebAssembly using Fermyon Spin and persist data in MySQL.

To run this application, you must have Spin CLI installed in version `0.7.0` or newer. Additionally, you must have access to a MySQL server (for demonstration purposes MySQL is executed in Docker).

## Running the Sample

### MySQL

```bash
# Run MySQL in Docker
docker run -d -p 3306:3306 \
 -e MYSQL_ROOT_PASSWORD=adminFooBar \
 -e MYSQL_USER=sampleuser \
 -e MYSQL_PASSWORD=foobar \
 -e MYSQL_DATABASE=products \
 --name mysql \
 mysql

# Create Products table leveraging ubuntu container
docker run -it --rm --link mysql ubuntu
# Install mysql-client
$ apt update
$ apt install mysql-client --yes
$ mysql -h mysql -u sampleuser -p
# provide user password when asked (foobar)

# Select products database
mysql> use products;
# Create the products table
mysql> create table products.Products
(
  Id bigint unsigned auto_increment primary key,
  Name varchar(250) charset utf8mb3 not null,
  Price float not null
);
```

Having the MySQL server running in Docker, you must update `Spin.toml` and provide your connection string as part of the component configuration `[component.config]`.

Finally, you can do a `spin build --up --follow-all` to spin up the Spin application.

Further guidance and a detailed walk-through can be found in this article: [https://www.thorsten-hans.com/crud-in-webassembly-with-fermyon-spin-and-mysql/](https://www.thorsten-hans.com/crud-in-webassembly-with-fermyon-spin-and-mysql/)
