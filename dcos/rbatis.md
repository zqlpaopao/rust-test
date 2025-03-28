# 一、rbatis简介

高性能 SQL 工具包和编译时 ORM 库。一个异步、纯`Rust`SQL 包，具有编译时动态 SQL

它是一个ORM，一个小型编译器，一个动态SQL语言

- 兼容大多数mybatis3语法。您可以开始将Java项目重新编码为`Rust`！
- 无运行时，无垃圾收集，高性能，基于Future/Tokio
- 零成本[动态SQL](https://rbatis.github.io/rbatis.io/#/../v4/dyn_sql)，使用(proc-macro,compile-time,Cow(减少不必要的克隆))技术实现。不需要ONGL引擎(mybatis)
- 类似JDBC的驱动设计，驱动使用cargo.toml依赖和`Box<dyn Driver>`分离
- 支持所有数据库驱动程序`#{arg}`, `${arg}`,`?` 占位符（pg/mssql 自动处理 '?' 到 '$1' 和 '@P1'）
- 动态SQL（用SQL自由编写代码）、分页、`py_sql`查询语言和`html_sql`（灵感Mybatis）。
- 动态配置连接池（基于https://github.com/rbatis/fast_pool）
- 支持基于拦截器实现的日志记录
- 100% 安全纯净，`Rust`启用`#![forbid(unsafe_code)]`
- [rbatis/示例](https://github.com/rbatis/example)
- [abs_admin 项目](https://github.com/rbatis/abs_admin)一个后台用户管理系统(Vue.js+rbatis+axum)

#### [支持的数据库驱动程序](https://rbatis.github.io/rbatis.io/#/v4/?id=supported-database-driver)

> RBatis 支持任何 impl [rdbc](https://crates.io/crates/rbdc)驱动程序。如果你没有你想要的以下驱动程序，你可以自己写一个，只要 impl `rbdc::db::*`Traits即可

| database(crates.io)                                | github_link                                                  |
| -------------------------------------------------- | ------------------------------------------------------------ |
| [Mysql](https://crates.io/crates/rbdc-mysql)       | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql) |
| [Postgres](https://crates.io/crates/rbdc-pg)       | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg) |
| [Sqlite](https://crates.io/crates/rbdc-sqlite)     | [rbatis/rbdc-sqlite](https://github.com/rbatis/rbatis/tree/master/rbdc-sqlite) |
| [Mssql](https://crates.io/crates/rbdc-mssql)       | [rbatis/rbdc-mssql](https://github.com/rbatis/rbatis/tree/master/rbdc-mssql) |
| [MariaDB](https://crates.io/crates/rbdc-mysql)     | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql) |
| [TiDB](https://crates.io/crates/rbdc-mysql)        | [rbatis/rbdc-mysql](https://github.com/rbatis/rbatis/tree/master/rbdc-mysql) |
| [CockroachDB](https://crates.io/crates/rbdc-pg)    | [rbatis/rbdc-pg](https://github.com/rbatis/rbatis/tree/master/rbdc-pg) |
| [Oracle](https://crates.io/crates/rbdc-oracle)     | [chenpengfan/rbdc-oracle](https://github.com/chenpengfan/rbdc-oracle) |
| [TDengine](https://crates.io/crates/rbdc-tdengine) | [tdcare/rbdc-tdengine](https://github.com/tdcare/rbdc-tdengine) |



# 二、使用

## 2.1 toml

```
#rbatis deps
rbs = { version = "4.5"}
rbatis = { version = "4.5"}
rbdc-sqlite = { version = "4.5" }
#rbdc-mysql={version="4.5"}
#rbdc-pg={version="4.5"}
#rbdc-mssql={version="4.5"}

serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

- `toml` `native-tls`（选项）

```
rbs = { version = "4.5" }
rbdc-sqlite = { version = "4.5", default-features = false, features = ["tls-native-tls"] }
#rbdc-mysql={version="4.5", default-features = false, features = ["tls-native-tls"]}
#rbdc-pg={version="4.5", default-features = false, features = ["tls-native-tls"]}
#rbdc-mssql={version="4.5", default-features = false, features = ["tls-native-tls"]}
rbatis = { version = "4.5" }

serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
log = "0.4"
fast_log = "1.6"
```

