//! # Rust 学习示例
//!
//! 本项目包含 Rust 核心特性的学习示例：
//! - ownership.rs: 所有权系统（move、借用、生命周期）
//! - concurrency.rs: 多线程与共享数据（Arc、Mutex、channel）
//! - std_components.rs: 常用标准库组件（Vec、HashMap、迭代器等）

mod ownership;
mod concurrency;
mod std_components;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║       Rust 学习示例                  ║");
    println!("╚══════════════════════════════════════╝");

    println!("\n\n【1. 所有权系统】");
    ownership::run();

    println!("\n\n【2. 多线程与共享数据】");
    concurrency::run();

    println!("\n\n【3. 标准库组件】");
    std_components::run();

    println!("\n\n所有示例运行完成！");
}
