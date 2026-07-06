//! # 13 - 网络库
//!
//! Rust 网络编程主要依赖 tokio（异步运行时）和 reqwest（HTTP 客户端）。
//!
//! 常用 crate：
//! - tokio: 异步运行时
//! - reqwest: HTTP 客户端
//! - hyper: 底层 HTTP 库
//! - axum: Web 框架
//! - tungstenite: WebSocket

/// HTTP 客户端基础（同步方式，使用 ureq）
pub fn http_client_basics() {
    println!("=== HTTP 客户端基础 ===");

    // 注意：这里展示概念，实际使用需要添加依赖
    // reqwest = { version = "0.12", features = ["json"] }

    println!("reqwest 基础用法：");
    println!("  // GET 请求");
    println!("  let resp = reqwest::blocking::get(url)?;");
    println!("  let body = resp.text()?;");
    println!();
    println!("  // POST JSON");
    println!("  let client = reqwest::blocking::Client::new();");
    println!("  let resp = client.post(url)");
    println!("      .json(&data)");
    println!("      .send()?;");
    println!();
    println!("  // 异步版本");
    println!("  let resp = reqwest::get(url).await?;");
    println!("  let body = resp.text().await?;");
}

/// 异步 HTTP 客户端
pub fn async_http() {
    println!("\n=== 异步 HTTP ===");

    // tokio 异步运行时
    println!("tokio 基础：");
    println!("  #[tokio::main]");
    println!("  async fn main() {{");
    println!("      // 异步代码");
    println!("  }}");
    println!();
    println!("  // 或者手动创建运行时");
    println!("  let rt = tokio::runtime::Runtime::new()?;");
    println!("  rt.block_on(async {{ ... }})");
    println!();
    println!("tokio 常用功能：");
    println!("  tokio::spawn(async {{ ... }})  // 生成任务");
    println!("  tokio::time::sleep(dur).await  // 异步等待");
    println!("  tokio::fs::read(path).await    // 异步文件读取");
    println!("  tokio::net::TcpStream::connect(addr).await  // TCP连接");
}

/// TCP/UDP 编程
pub fn tcp_udp() {
    println!("\n=== TCP/UDP 编程 ===");

    // 标准库 TCP（同步）
    println!("标准库 TCP 服务端：");
    println!("  use std::net::TcpListener;");
    println!("  let listener = TcpListener::bind(\"127.0.0.1:8080\")?;");
    println!("  for stream in listener.incoming() {{");
    println!("      let stream = stream?;");
    println!("      // 处理连接");
    println!("  }}");
    println!();
    println!("标准库 TCP 客户端：");
    println!("  use std::net::TcpStream;");
    println!("  use std::io::{{Read, Write}};");
    println!("  let mut stream = TcpStream::connect(\"127.0.0.1:8080\")?;");
    println!("  stream.write_all(b\"hello\")?;");
    println!();
    println!("tokio 异步 TCP：");
    println!("  let listener = tokio::net::TcpListener::bind(\"127.0.0.1:8080\").await?;");
    println!("  loop {{");
    println!("      let (socket, addr) = listener.accept().await?;");
    println!("      tokio::spawn(async move {{");
    println!("          // 处理连接");
    println!("      }});");
    println!("  }}");
}

/// WebSocket
pub fn websocket() {
    println!("\n=== WebSocket ===");

    // tungstenite 库
    println!("tungstenite 用法：");
    println!("  use tungstenite::connect;");
    println!("  let (mut socket, response) = connect(\"ws://localhost:8080\")?;");
    println!("  socket.send(tungstenite::Message::Text(\"hello\".into()))?;");
    println!("  let msg = socket.read()?;");
    println!();
    println!("tokio-tungstenite（异步）：");
    println!("  use tokio_tungstenite::connect_async;");
    println!("  let (ws_stream, _) = connect_async(url).await?;");
    println!("  let (write, read) = ws_stream.split();");
}

/// JSON 处理
pub fn json_handling() {
    println!("\n=== JSON 处理 ===");

    // serde 和 serde_json
    println!("serde 序列化/反序列化：");
    println!("  use serde::{{Serialize, Deserialize}};");
    println!();
    println!("  #[derive(Serialize, Deserialize, Debug)]");
    println!("  struct User {{");
    println!("      name: String,");
    println!("      age: u32,");
    println!("      email: Option<String>,");
    println!("  }}");
    println!();
    println!("  // 序列化");
    println!("  let user = User {{ name: \"Alice\".into(), age: 30, email: None }};");
    println!("  let json = serde_json::to_string(&user)?;");
    println!();
    println!("  // 反序列化");
    println!("  let user: User = serde_json::from_str(&json)?;");
    println!();
    println!("  // 从文件读取");
    println!("  let data = std::fs::read_to_string(\"user.json\")?;");
    println!("  let user: User = serde_json::from_str(&data)?;");
}

/// Web 框架简介
pub fn web_frameworks() {
    println!("\n=== Web 框架 ===");

    // axum 框架
    println!("axum 基础：");
    println!("  use axum::{{routing::get, Router}};");
    println!();
    println!("  let app = Router::new()");
    println!("      .route(\"/\", get(|| async {{ \"Hello!\" }}))");
    println!("      .route(\"/users/:id\", get(get_user));");
    println!();
    println!("  let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await?;");
    println!("  axum::serve(listener, app).await?;");
    println!();
    println!("actix-web 基础：");
    println!("  use actix_web::{{web, App, HttpServer, HttpResponse}};");
    println!();
    println!("  HttpServer::new(|| {{");
    println!("      App::new()");
    println!("          .route(\"/\", web::get().to(|| async {{ HttpResponse::Ok().body(\"Hello!\") }}))");
    println!("  }})");
    println!("  .bind(\"127.0.0.1:8080\")?");
    println!("  .run()");
    println!("  .await?;");
}

/// 运行示例
pub fn run() {
    http_client_basics();
    async_http();
    tcp_udp();
    websocket();
    json_handling();
    web_frameworks();
}
