# Rocket 阅读记录:

## IDE(Clion) 设置:

- [packages/rocket/rocket-0.5.0-rc.2/examples/Cargo.toml](./rocket-0.5.0-rc.2/examples/Cargo.toml)
    - IDE 打开这个 cargo.toml 文件, IDE 会提示 attach 到工程.
    - 这样 IDE 就会为 examples 创建项目索引, 就可以正常跳转代码.

## 阅读方法:

- 从示例入手, 逐步拆解整个 web 框架
- 这是最高效的阅读方式

> 示例入口:

- [examples](./rocket-0.5.0-rc.2/examples)

## 默认注解规则:

- 所有新增注解, 注释前缀为如下2种格式:
- 区分原 repo 注释, 以及方便醒目查看

```ruby 

// TODO X: xxx
// todo x: xxx

```

- `jump in`标志, 此处可以跳转到 web 框架, 会继续注解

## 版本说明:

- [rocket-0.5.0-rc.2](./rocket-0.5.0-rc.2)
    - https://github.com/SergioBenitez/Rocket/releases/tag/v0.5.0-rc.2


