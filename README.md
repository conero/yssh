# yssh

> 2019年11月7日 星期四
>
> Joshua Conero







## 概述

*基于 rust 与 golang 的代码编写常用的命令行工具。rust 语言的启发项目：[deno](https://github.com/denoland/deno), [rocket](https://github.com/SergioBenitez/Rocket).*



### rust

```shell
# 打开本地文档
rustup doc
```



#### rust fmt

[代码仓库](https://github.com/rust-lang/rustfmt)，初始化如下：

```shell
# 使用 fmt 格式化代码
# 安装组件
rustup component add rustfmt
# 格式化
cargo fmt
```



## 设计

### 命令程序

```powershell
# 运行
uysh
```



### uymas

*与使用 ，Rocket 类似的宏实现路由，考虑的复杂性先使用简单的回调函数实现。*

