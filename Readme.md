# annotated-rs:

- rust 知名项目源码分析

## projects:

> 注解 ing:

- https://github.com/actix/actix-web

> 备选项目:

- https://github.com/tokio-rs/tokio
    - 异步 I/O
- https://github.com/tokio-rs/axum
    - web 框架
- https://github.com/actix/actix-web
    - web 框架
- https://github.com/tikv/tikv
    - kv db
- https://github.com/diesel-rs/diesel
    - ORM
- https://github.com/paritytech/substrate
    - 区块链
- https://github.com/solana-labs/solana
    - 区块链

## 准备工作:

### 1. 搭建阅读环境:

> 安装 rust 开发环境

- 略

> 配置源码阅读工具: Clion

- https://github.com/better-rs/.github/discussions/8
- 更好的代码跳转
- 默认单个目标项目内, 是无法自动识别+跳转的

> 以 axum 为例:

- IDE 打开: `axum-axum-v0.4.5` 文件夹, 找到 `Cargo.toml` 右键,
- 需要手动找到工程的根目录, attach `Cargo.toml` 配置
- 之后 IDE 会自动安装依赖包
- 首次索引会比较慢, 耐心等待

### 阅读 axum:

> 安装依赖包:

- 依赖 go-task 工具(替代 Makefile)

```ruby 


task install 

```

## 默认注解规则:

> `TODO X: XXX` 标志:

- 所有新增注解, 注释前缀为如下2种格式:
- 区分原 repo 注释, 以及方便醒目查看

```ruby 

// TODO X: xxx
// todo x: xxx

```

> `jump in` 标志:

- 此处基于 IDE, 可以跳转上下文, 会继续注解

## ref:

- https://fancy.rs/
- https://github.com/tokio-rs
- https://github.com/AppFlowy-IO/AppFlowy
- https://github.com/rustdesk/rustdesk
- https://github.com/getzola/zola
- https://github.com/LemmyNet/lemmy
    - reddit 社区