# axum 阅读记录:

## 默认注解规则:

- 所有新增注解, 注释前缀为如下2种格式:
- 区分原 repo 注释, 以及方便醒目查看

```ruby 

// TODO X: xxx
// todo x: xxx

```

- `jump in`标志, 此处可以跳转到 web 框架, 会继续注解

## 版本说明:

- [axum-axum-v0.4.5](./axum-axum-v0.4.5)
    - https://github.com/tokio-rs/axum/releases/tag/axum-v0.4.5

## 阅读方法:

- 从示例入手, 逐步拆解整个 web 框架
- 这是最高效的阅读方式

> 示例入口:

- [examples](./axum-axum-v0.4.5/examples)
    - [hello-world](./axum-axum-v0.4.5/examples/hello-world)
        - 最简单示例
    - [cors](./axum-axum-v0.4.5/examples/cors)
        - 多线程示例


