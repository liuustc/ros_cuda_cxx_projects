//! # 14 - 日志库
//!
//! Rust 日志生态：
//! - log: 日志门面（facade）
//! - env_logger: 环境变量配置的日志实现
//! - tracing: 结构化日志（推荐）
//! - tracing-subscriber: tracing 的订阅者

/// log 门面
pub fn log_facade() {
    println!("=== log 门面 ===");

    // log crate 提供日志宏接口
    println!("log 宏：");
    println!("  error!(\"错误信息\");");
    println!("  warn!(\"警告信息\");");
    println!("  info!(\"信息\");");
    println!("  debug!(\"调试信息\");");
    println!("  trace!(\"跟踪信息\");");
    println!();
    println!("  // 带格式化");
    println!("  info!(\"user {{}} logged in\", username);");
    println!("  error!(\"failed: {{:?}}\", err);");
    println!();
    println!("  // 带键值对");
    println!("  info!(user = \"alice\", action = \"login\");");
}

/// env_logger
pub fn env_logger_demo() {
    println!("\n=== env_logger ===");

    // 环境变量配置
    println!("env_logger 用法：");
    println!("  // 初始化");
    println!("  env_logger::init();");
    println!();
    println!("  // 或者自定义");
    println!("  env_logger::Builder::new()");
    println!("      .filter_level(log::LevelFilter::Debug)");
    println!("      .init();");
    println!();
    println!("环境变量：");
    println!("  RUST_LOG=info           # 全局级别");
    println!("  RUST_LOG=my_crate=debug # 特定 crate");
    println!("  RUST_LOG=debug          # 调试级别");
}

/// tracing（推荐）
pub fn tracing_demo() {
    println!("\n=== tracing（推荐）===");

    // tracing 提供结构化日志
    println!("tracing 基础：");
    println!("  use tracing::{{info, warn, error, debug, trace, instrument}};");
    println!();
    println!("  #[instrument]");
    println!("  fn process_request(id: u64) {{");
    println!("      info!(\"processing request\");");
    println!("      // 自动记录函数名和参数");
    println!("  }}");
    println!();
    println!("  // 带字段");
    println!("  info!(user = \"alice\", count = 42, \"user action\");");
    println!();
    println!("  // span（跟踪范围）");
    println!("  let span = tracing::info_span!(\"my_span\", id = 1);");
    println!("  let _guard = span.enter();");
    println!("  info!(\"inside span\");");
}

/// tracing-subscriber
pub fn tracing_subscriber() {
    println!("\n=== tracing-subscriber ===");

    // 订阅者配置
    println!("tracing-subscriber 配置：");
    println!("  use tracing_subscriber::{{fmt, EnvFilter, Registry}};");
    println!("  use tracing_subscriber::prelude::*;");
    println!();
    println!("  // 简单配置");
    println!("  tracing_subscriber::fmt()");
    println!("      .with_env_filter(EnvFilter::from_default_env())");
    println!("      .init();");
    println!();
    println!("  // 自定义格式");
    println!("  tracing_subscriber::fmt()");
    println!("      .with_target(false)");
    println!("      .with_thread_ids(true)");
    println!("      .with_file(true)");
    println!("      .with_line_number(true)");
    println!("      .init();");
    println!();
    println!("  // JSON 格式（生产环境）");
    println!("  tracing_subscriber::fmt()");
    println!("      .json()");
    println!("      .with_env_filter(EnvFilter::from_default_env())");
    println!("      .init();");
}

/// 结构化日志示例
pub fn structured_logging() {
    println!("\n=== 结构化日志 ===");

    // 结构化日志最佳实践
    println!("结构化日志示例：");
    println!("  // 请求处理");
    println!("  info!(");
    println!("      method = \"GET\",");
    println!("      path = \"/api/users\",");
    println!("      status = 200,");
    println!("      latency_ms = 42,");
    println!("      \"request completed\"");
    println!("  );");
    println!();
    println!("  // 错误记录");
    println!("  error!(");
    println!("      error = %err,");
    println!("      user_id = user.id,");
    println!("      \"failed to process order\"");
    println!("  );");
    println!();
    println!("  // 使用 span 跟踪请求");
    println!("  #[instrument(skip(db))]");
    println!("  async fn handle_request(req: Request, db: &Db) -> Response {{");
    println!("      info!(\"handling request\");");
    println!("      // 自动记录 req 的字段");
    println!("  }}");
}

/// 日志级别选择
pub fn log_levels() {
    println!("\n=== 日志级别 ===");

    println!("日志级别（从高到低）：");
    println!("  error! - 错误：需要立即关注的问题");
    println!("  warn!  - 警告：潜在问题，但不致命");
    println!("  info!  - 信息：关键业务事件");
    println!("  debug! - 调试：开发调试信息");
    println!("  trace! - 跟踪：详细执行路径");
    println!();
    println!("使用建议：");
    println!("  - 生产环境：info 或 warn");
    println!("  - 开发环境：debug");
    println!("  - 性能分析：trace");
}

/// 运行示例
pub fn run() {
    log_facade();
    env_logger_demo();
    tracing_demo();
    tracing_subscriber();
    structured_logging();
    log_levels();
}
