//! # Rust 学习示例
//!
//! 本项目包含 Rust 核心特性的学习示例，每个模块对应一个特性：
//!
//! 基础部分：
//! - 01_ownership: 所有权系统（move、借用、生命周期）
//! - 02_concurrency: 多线程与共享数据（Arc、Mutex、channel）
//! - 03_std_components: 常用标准库组件（Vec、HashMap、迭代器等）
//!
//! 核心特性：
//! - 04_trait: Trait 系统（抽象、多态、关联类型）
//! - 05_generic: 泛型（函数泛型、结构体泛型、const泛型）
//! - 06_pattern_matching: 模式匹配（match、解构、守卫）
//! - 07_closure: 闭包（Fn/FnMut/FnOnce、捕获环境）
//! - 08_smart_pointer: 智能指针（Box、Rc、Cell、Cow、Weak）
//! - 09_async: 异步编程（Future、async/await）
//! - 10_error_handling: 错误处理（Result、Option、?运算符）
//! - 11_macro: 宏（声明宏、过程宏、derive宏）
//!
//! 生态工具：
//! - 12_testing: 单元测试
//! - 13_networking: 网络库
//! - 14_logging: 日志库
//! - 15_ffi: C/C++ 互操作
//! - 16_cargo: 依赖管理

mod _01_ownership;
mod _02_concurrency;
mod _03_std_components;
mod _04_trait;
mod _05_generic;
mod _06_pattern_matching;
mod _07_closure;
mod _08_smart_pointer;
mod _09_async;
mod _10_error_handling;
mod _11_macro;
mod _12_testing;
mod _13_networking;
mod _14_logging;
mod _15_ffi;
mod _16_cargo;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║       Rust 学习示例                  ║");
    println!("╚══════════════════════════════════════╝");

    println!("\n\n【1. 所有权系统】");
    _01_ownership::run();

    println!("\n\n【2. 多线程与共享数据】");
    _02_concurrency::run();

    println!("\n\n【3. 标准库组件】");
    _03_std_components::run();

    println!("\n\n【4. Trait 系统】");
    _04_trait::run();

    println!("\n\n【5. 泛型】");
    _05_generic::run();

    println!("\n\n【6. 模式匹配】");
    _06_pattern_matching::run();

    println!("\n\n【7. 闭包】");
    _07_closure::run();

    println!("\n\n【8. 智能指针】");
    _08_smart_pointer::run();

    println!("\n\n【9. 异步编程】");
    _09_async::run();

    println!("\n\n【10. 错误处理】");
    _10_error_handling::run();

    println!("\n\n【11. 宏】");
    _11_macro::run();

    println!("\n\n【12. 单元测试】");
    _12_testing::run();

    println!("\n\n【13. 网络库】");
    _13_networking::run();

    println!("\n\n【14. 日志库】");
    _14_logging::run();

    println!("\n\n【15. C/C++ 互操作】");
    _15_ffi::run();

    println!("\n\n【16. Cargo 依赖管理】");
    _16_cargo::run();

    println!("\n\n所有示例运行完成！");
}
