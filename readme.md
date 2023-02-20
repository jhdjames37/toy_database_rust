# Database Manage System

《数据库系统概论》课程项目，Rust 编写。

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

## 一些碎碎念

想着用 Rust 来写这玩意儿也是大作业布置当天突发奇想的一个玩意儿（事实上我是暑假才开始学 Rust 的，这个大作业其实是我的第一个尝试）。然后当周试了一下发现，发现技术栈上并不缺东西（当时主要担心的就是 antlr 和 filesystem），于是就这样子写下来了。

用 Rust 来写的话，说实话确实有些地方比较头疼。和借用检查器做斗争很难受，就有些地方我知道它没必要 `clone` 但不得不 `clone`（不知道该怎么改）。最大的坑其实是出在 `RecordManager` (可以看 `record/table.rs`)，引入了一个（其实没啥必要的）生命周期参数，其它地方还好，到了查询处理要上迭代器了，查询树遍历那个地方的生命周期就死活调不对，然后折腾了一个晚上，还是回去改迭代器那里才让它跑通的。当然也许有些 Rust 大佬可以给出更好的解决方案。另外在写 B+ 树的时候有些地方的写法也是受到限制的，也许可以试试 `unsafe`？

当然，Rust 当然还是比较现代的语言，相比 C++ 相对来说轮子和语法糖肯定会多一些（我不会 C++14/17/20。）。至于内存安全这件事情的话，我个人觉得因为这个代码规模还没完全上去，所以说优势可能还不明显？另外关于 Rust 更多高级特性，本人才疏学浅不会用，因此也没用。

然后谈谈开发的历程吧。10.17 布置的大作业，然后我那周就开始移植 C++ 祖传代码，试了一下 antlr 编译。然后开始写记录模块，把数据记录写好之后，就去搞前端，先把 `INSERT` 和 `SELECT` 能够跑出来（这个时候的元数据甚至还是 mock 的）。这样子的话你写后面的操作就可以保证每一个输入就能看到输出（之前都是写单测的，因为没有做 mock 的支持，所以后面还写单测有点麻烦就不写了2333），同时错误处理也可以一起做掉。这之后又加了一些 SELECT 子句然后就去造机了。造机造完回来开始写元数据的存储，主外键，索引，以及各种细枝末节的东西。因为框架已经搭在那里了，思路很多都已经确定下来了，接下来其实就是写代码的事情了。当然 12 月份逃离清华之后在家里（除了期末考以外）基本上就是写数据库了。。。

最后验收前群里问了一些稀奇古怪的 corner case，搞得有点慌。不过最后还是测的是基础的错误处理。

然后关于项目代码的话，后面的许多代码其实质量肉眼可见地不高。很多可以复用的代码就一遍一遍地在复制粘贴，就比如说 `Box<dyn std::error::Error>` 我敲了不知道多少遍然后就不把它重定义掉。另外一个印象比较深的点就是 WHERE 语句在 SELECT 和 UPDATE/DELETE 中是写了两遍的，代码其实是差不多的，应该也是可以合并的。然后像查询处理的某两个文件也非常的冗长，找一个函数要找半天。。。最后不算 parser 的话总共 8200 行代码，比之前老师声称的 5000-6000 要多一些，估计需要感谢 `rustfmt`。。。

总之，Rust 肯定是能写出这个项目的，加上一字班小学期开始教 Rust 了，也许未来会有更多人选 Rust？
