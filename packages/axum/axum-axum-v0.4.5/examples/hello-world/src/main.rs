//! Run with
//!
//! ```not_rust
//! cargo run -p example-hello-world
//! ```

// todo x: 导入核心依赖
//  - Router: 路由调度
//  - http get 请求类型
//  - http response 格式, 基本套路
use axum::{response::Html, routing::get, Router};

// todo x: 标准库: 绑定 server 端口
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // todo x: 路由注册: jump in 框架
    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // todo x: 绑定端口
    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on {}", addr);

    //----------------------------------------------//

    // todo x: bind + serve 启动 http server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// todo x: 具体 API 相应
//  - 返回值类型: & 'static 标记
async fn handler() -> Html<&'static str> {
    // 打印一个提示:
    println!("Hello, World");

    // todo x: 表达式, 直接返回 HTML // jump in
    Html("<h1>Hello, World!</h1>")
}
