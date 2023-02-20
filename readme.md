# Database Manage System

清华大学《数据库系统概论》课程项目，Rust 编写。

A database manage system written in Rust, as a course project of *Introduction to Database Management System* in THU.

## 运行

``` bash
cargo build --release
```

`target/release/database` 即为可执行文件。

开发及测试均在 Linux 环境下完成，不保证其它系统能够正常运行。

运行（注：需要提前新建相关目录）：

``` bash
./database --path path/to/database [--cache <cache_size>]
```

或者直接一步编译运行

``` bash
cargo run --release -- --path path/to/database [--cache <cache_size>]
```

## 完成内容

+ 基本数据库/表管理
+ 基本增删改查，其中 SELECT 语句包含
  - col op col
  - col op value
  - col in value
  - col like pattern
  - 嵌套查询
  - limit/offset
+ （联合）主外键
+ （多列）索引

## antlr 相关

antlr 官方目前未提供 Rust 支持。

使用[antlr-rust](https://crates.io/crates/antlr-rust)转换工具和运行时库。

目前使用的是 `0.3.0-beta`。

安装 JRE 库并下载作者的 [rust 解析生成库](https://github.com/rrevenantt/antlr4/tree/rust-target) 后，执行安装。（助教提供的生成命令中的 `-nolistener` 选项会导致编译错误，故删去）

```bash
java -jar <antlr4-4.8-2-SNAPSHOT-complete.jar> -Dlanguage=Rust SQL.g4  -visitor -o parser
```

## 开发说明

可以参阅[实验报告](/report.pdf)，不过是一天赶出来的，有些地方可能会有错漏。

另外[这里](/dev.md)是一些偏吐槽向的开发总结。
