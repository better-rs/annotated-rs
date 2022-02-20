# substrate 阅读记录:

## 阅读方法:

- 有示例, 从示例入口开始阅读. 没有, 查看 test 代码.
- 通过示例方式, 拆解框架.
- 这是最高效的阅读方式

> 入口:

- [bin/node/cli](./substrate-monthly-2022-02/bin/node/cli/bin/main.rs)
    - [client/cli/commands](./substrate-monthly-2022-02/client/cli/src/commands)

## 源码目录结构说明:

> 根目录结构:

```ruby 

-> % tree substrate-monthly-2022-02 -L 1

substrate-monthly-2022-02
├── Cargo.lock
├── Cargo.toml
├── README.md
├── bin            // 命令行工具: 项目阅读入口
├── client         // 客户端
├── docker
├── docs
├── frame          // 框架
├── primitives
├── rustfmt.toml
├── shell.nix
├── test-utils
└── utils

8 directories, 5 files


```

> bin 目录结构:

- 项目入口: [bin/node/cli](./substrate-monthly-2022-02/bin/node/cli/bin/main.rs)

```ruby 



``` 

## 版本说明:

- [substrate-monthly-2022-02](substrate-monthly-2022-02)
    - https://github.com/paritytech/substrate/releases/tag/monthly-2022-02

## substrate reference:

- https://docs.substrate.io/v3/getting-started/installation/
- https://www.subdev.cn/
- https://www.subdev.cn/docs/learn_resource
- https://www.subdev.cn/docs/course
- https://zhuanlan.zhihu.com/substrate
- [M1编译Substrate随笔](https://zhuanlan.zhihu.com/p/337224781)
- https://zhuanlan.zhihu.com/p/161771205
- https://space.bilibili.com/67358318

> notes:

- https://whisperd.tech/post/substrate_read_source_code/