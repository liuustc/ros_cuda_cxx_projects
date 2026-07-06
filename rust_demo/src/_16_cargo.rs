//! # 16 - Cargo 依赖管理
//!
//! Cargo 是 Rust 的构建系统和包管理器。
//! 理解 Cargo 是高效开发的关键。

/// Cargo.toml 结构
pub fn cargo_toml_structure() {
    println!("=== Cargo.toml 结构 ===");

    println!("[package]");
    println!("name = \"my_project\"");
    println!("version = \"0.1.0\"");
    println!("edition = \"2021\"");
    println!("authors = [\"Your Name <email@example.com>\"]");
    println!("description = \"A short description\"");
    println!("license = \"MIT\"");
    println!("repository = \"https://github.com/user/repo\"");
    println!();
    println!("[dependencies]");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("tokio = {{ version = \"1\", features = [\"full\"] }}");
    println!("reqwest = {{ version = \"0.12\", features = [\"json\"] }}");
    println!("log = \"0.4\"");
    println!("tracing = \"0.1\"");
    println!();
    println!("[dev-dependencies]");
    println!("tempfile = \"3\"");
    println!("assert_cmd = \"2\"");
    println!();
    println!("[build-dependencies]");
    println!("bindgen = \"0.69\"");
}

/// 版本规范
pub fn version_specs() {
    println!("\n=== 版本规范 ===");

    println!("版本格式：MAJOR.MINOR.PATCH");
    println!();
    println!("版本指定方式：");
    println!("  \"1.0\"     = \"^1.0\"   = 1.0.0 <= version < 2.0.0");
    println!("  \"^1.2.3\"  = 1.2.3 <= version < 2.0.0");
    println!("  \"~1.2.3\"  = 1.2.3 <= version < 1.3.0");
    println!("  \"=1.2.3\"  = 精确版本");
    println!("  \">=1.0, <2.0\" = 范围");
    println!("  \"*\"       = 任意版本");
    println!();
    println!("预发布版本：");
    println!("  \"1.0.0-alpha\"");
    println!("  \"1.0.0-beta.1\"");
    println!("  \"1.0.0-rc.1\"");
}

/// Features
pub fn features() {
    println!("\n=== Features ===");

    println!("Features 条件编译：");
    println!("[features]");
    println!("default = [\"std\"]");
    println!("std = []");
    println!("json = [\"serde\", \"serde_json\"]");
    println!("full = [\"json\", \"xml\", \"csv\"]");
    println!();
    println!("使用：");
    println!("  [dependencies]");
    println!("  my_crate = {{ version = \"1.0\", features = [\"json\"] }}");
    println!();
    println!("代码中：");
    println!("  #[cfg(feature = \"json\")]");
    println!("  pub mod json {{ ... }}");
    println!();
    println!("  #[cfg(feature = \"json\")]");
    println!("  use serde_json;");
}

/// Workspace
pub fn workspace() {
    println!("\n=== Workspace ===");

    println!("多 crate 项目：");
    println!("my_project/");
    println!("├── Cargo.toml        # workspace 根");
    println!("├── core/             # 核心库");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("├── api/              # Web API");
    println!("│   ├── Cargo.toml");
    println!("│   └── src/");
    println!("└── cli/              # 命令行工具");
    println!("    ├── Cargo.toml");
    println!("    └── src/");
    println!();
    println!("根 Cargo.toml：");
    println!("[workspace]");
    println!("members = [\"core\", \"api\", \"cli\"]");
    println!();
    println!("[workspace.dependencies]");
    println!("serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!();
    println!("子 crate 引用：");
    println!("[dependencies]");
    println!("my_core = {{ path = \"../core\" }}");
    println!("serde.workspace = true");
}

/// 条件编译
pub fn conditional_compilation() {
    println!("\n=== 条件编译 ===");

    println!("cfg 属性：");
    println!("  #[cfg(target_os = \"linux\")]");
    println!("  fn linux_only() {{ ... }}");
    println!();
    println!("  #[cfg(target_arch = \"x86_64\")]");
    println!("  fn x86_64_only() {{ ... }}");
    println!();
    println!("  #[cfg(feature = \"json\")]");
    println!("  pub mod json_support;");
    println!();
    println!("  #[cfg(not(target_os = \"windows\"))]");
    println!("  fn non_windows() {{ ... }}");
    println!();
    println!("  #[cfg(any(target_os = \"linux\", target_os = \"macos\"))]");
    println!("  fn unix_only() {{ ... }}");
    println!();
    println!("运行时检查：");
    println!("  if cfg!(target_os = \"linux\") {{ ... }}");
}

/// 构建脚本
pub fn build_scripts() {
    println!("\n=== 构建脚本 ===");

    println!("build.rs 构建脚本：");
    println!("  // build.rs");
    println!("  fn main() {{");
    println!("      // 设置环境变量");
    println!("      println!(\"cargo:rerun-if-changed=build.rs\");");
    println!("      println!(\"cargo:rustc-link-lib=static=mylib\");");
    println!("      println!(\"cargo:rustc-link-search=native=/usr/local/lib\");");
    println!("  }}");
    println!();
    println!("常用指令：");
    println!("  cargo:rerun-if-changed=FILE  # 文件变化时重新构建");
    println!("  cargo:rustc-link-lib=TYPE=NAME  # 链接库");
    println!("  cargo:rustc-link-search=KIND=PATH  # 库搜索路径");
    println!("  cargo:rustc-cfg=KEY  # 设置 cfg");
    println!("  cargo:rustc-env=KEY=VALUE  # 设置环境变量");
}

/// Cargo 命令
pub fn cargo_commands() {
    println!("\n=== Cargo 命令 ===");

    println!("常用命令：");
    println!("  cargo init my_project    # 创建新项目");
    println!("  cargo build              # 构建");
    println!("  cargo build --release    # Release 构建");
    println!("  cargo run                # 运行");
    println!("  cargo test               # 测试");
    println!("  cargo bench              # 基准测试");
    println!("  cargo doc --open         # 生成文档");
    println!("  cargo clippy             # Lint");
    println!("  cargo fmt                # 格式化");
    println!("  cargo update             # 更新依赖");
    println!("  cargo tree               # 依赖树");
    println!("  cargo publish            # 发布到 crates.io");
    println!();
    println!("高级命令：");
    println!("  cargo check              # 快速检查");
    println!("  cargo expand             # 宏展开");
    println!("  cargo audit              # 安全审计");
    println!("  cargo deny               # 依赖检查");
    println!("  cargo watch -x run       # 文件变化时自动运行");
}

/// 发布流程
pub fn publish_flow() {
    println!("\n=== 发布流程 ===");

    println!("发布到 crates.io：");
    println!("  1. 完善 Cargo.toml 元数据");
    println!("  2. 生成文档：cargo doc");
    println!("  3. 运行测试：cargo test");
    println!("  4. 登录：cargo login <token>");
    println!("  5. 打包检查：cargo package --list");
    println!("  6. 发布：cargo publish");
    println!();
    println!("注意事项：");
    println!("  - 版本号必须递增");
    println!("  - 发布后无法删除（只能 yank）");
    println!("  - 确保 license 和 description 完整");
}

/// 运行示例
pub fn run() {
    cargo_toml_structure();
    version_specs();
    features();
    workspace();
    conditional_compilation();
    build_scripts();
    cargo_commands();
    publish_flow();
}
