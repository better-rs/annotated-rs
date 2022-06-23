//! Run with
//!
//! ```not_rust
//! cargo run -p example-cors
//! ```

/*
    todo x:
        -  Json

*/
use axum::{
    http::Method,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Origin};

#[tokio::main]
async fn main() {
    //
    // todo x: 前端
    //
    let frontend = async {
        let app = Router::new().route("/", get(html));

        //
        // todo x: 前端 start
        //
        serve(app, 3000).await;
    };

    //----------------------------------------------//

    //
    // todo x:  后端 api, 跨域
    //
    let backend = async {
        // TODO X: 注册 API handler
        let app = Router::new().route("/json", get(json)).layer(
            // see https://docs.rs/tower-http/latest/tower_http/cors/index.html
            // for more details
            CorsLayer::new()
                .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()))
                .allow_methods(vec![Method::GET]),
        );

        //
        // todo x: 后端
        //
        serve(app, 4000).await;
    };

    //----------------------------------------------//

    //
    // todo x: 多线程, join! 是宏定义
    //
    tokio::join!(frontend, backend);
}

////////////////////////////////////////////////////////////////////////

async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    // todo x: 绑定
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/*
    TODO X:
        - HTML 嵌入的 js, 会 call 后端 API, 拉取数据
*/
async fn html() -> impl IntoResponse {
    Html(
        r#"
        <script>
            fetch('http://localhost:4000/json')
              .then(response => response.json())
              .then(data => console.log(data));
        </script>
        "#,
    )
}

// todo x: 后端 API
async fn json() -> impl IntoResponse {
    //
    // todo x: 返回 JSON
    //
    Json(vec!["one", "two", "three"])
}
