# annotated-rs:

- rust 知名项目源码分析

## related:

- https://github.com/better-rs/learn-rs
    - rust 编程练习, 包含一些算法练习

## 注解项目列表:

> 进展: axum / substrate 注解ing

- 入口在每个项目([packages](./packages))的 `README.md` 中, 如 [axum](./packages/axum/readme.md)

| 项目                                | 类型     | 评分     | 注解状态 | 备注                                                                               |
|:----------------------------------|:-------|:-------|:-----|:---------------------------------------------------------------------------------|
| [tokio](./packages/tokio)         | 异步 I/O | ⭐⭐⭐⭐⭐⭐ | Yes  | [v1.14.1](https://github.com/tokio-rs/tokio/releases/tag/tokio-1.14.1)           |
| [axum](./packages/axum)           | web 框架 | ⭐⭐⭐⭐⭐⭐ | Yes  | [v0.4.5](https://github.com/tokio-rs/axum/releases/tag/axum-v0.4.5)              |
| [rocket](./packages/rocket)       | web 框架 | ⭐⭐⭐⭐   | Yes    | [v0.5.0-rc.2](https://github.com/SergioBenitez/Rocket/releases/tag/v0.5.0-rc.2)      |
| [substrate](./packages/substrate) | 区块链    | ⭐⭐⭐⭐⭐⭐ | Yes  | [v2022-02](https://github.com/paritytech/substrate/releases/tag/monthly-2022-02) |
| [xxx](./xxx)                      | Web 框架 | ⭐⭐⭐    | No   | https://github.com/actix/actix-web                                               |
| [xxx](./xxx)                      | kv db  | ⭐⭐⭐    | No   | https://github.com/tikv/tikv                                                     |
| [xxx](./xxx)                      | db ORM | ⭐⭐⭐    | No   | https://github.com/diesel-rs/diesel                                              |
| [xxx](./xxx)                      | 区块链    | ⭐⭐⭐    | No   | https://github.com/solana-labs/solana                                            |
| [xxx](./xxx)                      | xxx    | ⭐⭐⭐    | No   | xxx                                                                              |
| [xxx](./xxx)                      | xxx    | ⭐⭐⭐    | No   | xxx                                                                              |
| [xxx](./xxx)                      | xxx    | ⭐⭐⭐    | No   | xxx                                                                              |

## 准备工作:

- 拉取本 repo:

```ruby

git clone git@github.com:better-rs/annotated-rs.git

# or:
git clone https://github.com/better-rs/annotated-rs.git

```

### 1. 搭建阅读环境:

> 安装 rust 开发环境

- 略

> 配置源码阅读工具: Clion

- https://github.com/better-rs/.github/discussions/8
- 更好的代码跳转
- 默认单个目标项目内, 是无法自动识别+跳转的
- Clion Mem:  建议 > 4GB, `Substrate` 工程源码巨大, IDE 默认内存不足

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