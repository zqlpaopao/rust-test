# diesel

Diesel 是一个用于 Rust 语言的现代化、类型安全的 SQL 查询库。它允许你以声明式的方式编写 SQL 查询，并提供了一组丰富的功能，包括关联、聚合、事务管理等。Diesel 与 Actix-Web 等 Web 框架很好地集成，可以轻松地构建高性能、安全的 Web 应用程序。

以下是一个简单的示例，说明如何使用 Diesel 执行 SQL 查询和操作：

1. 添加依赖：

在您的 `Cargo.toml` 文件中添加 Diesel 及其相关依赖：

```
[dependencies]
diesel = { version = "2.1.0", features = ["postgres"] }
dotenvy = "0.15"
```



2. 创建表

```
CREATE TABLE users (
  id INTEGER PRIMARY KEY AUTO_INCREMENT,
  name TEXT NOT NULL,
  hair_color TEXT,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);
```



3. 连接参数

与 MySQL 数据库的连接。连接 URL 的格式应为 `mysql://[user[:password]@]host/database_name[?unix_socket=socket-path&ssl_mode=SSL_MODE*&ssl_ca=/etc/ssl/certs/ca-certificates.crt&ssl_cert=/etc/ssl/certs/client-cert.crt&ssl_key=/etc/ssl/certs/client-key.crt]`

- `host`可以是 IP 地址或主机名。如果设置为`localhost`，将尝试通过 处的套接字建立连接`/tmp/mysql.sock`。如果您想通过 TCP 连接到本地服务器（例如 docker 容器），请使用`0.0.0.0`或`127.0.0.1`代替。
- `unix_socket`需要 unix 套接字的路径
- `ssl_ca`接受系统证书根的路径
- `ssl_cert`接受客户端证书文件的路径
- `ssl_key`接受客户端私钥文件的路径
- `ssl_mode`需要为 MySQL 客户端命令选项定义的值`--ssl-mode` 请参阅https://dev.mysql.com/doc/refman/5.7/en/connection-options.html#option_general_ssl-mode



## 1. 连接

