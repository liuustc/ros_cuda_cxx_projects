//! # Rust 学习示例
//!
//! 本项目包含 Rust 核心特性的学习示例，按学习顺序组织：
//!
//! ## 🔰 第一阶段：基础（必学）
//! | 模块 | 主题 | 关键概念 |
//! |------|------|----------|
//! | 01_ownership | 所有权系统 | move、借用、生命周期、省略规则 |
//! | 02_concurrency | 多线程与共享 | Arc、Mutex、channel、Send/Sync |
//! | 03_std_components | 标准库组件 | Vec、HashMap、迭代器、Option/Result |
//!
//! ## 🎯 第二阶段：核心特性（重点）
//! | 模块 | 主题 | 关键概念 |
//! |------|------|----------|
//! | 04_trait | Trait 系统 | 静态/动态分发、关联类型、运算符重载 |
//! | 05_generic | 泛型 | 单态化、trait 约束、const 泛型 |
//! | 06_pattern_matching | 模式匹配 | match、解构、守卫、@绑定、穷尽检查 |
//! | 07_closure | 闭包 | Fn/FnMut/FnOnce、捕获环境、零成本抽象 |
//! | 08_smart_pointer | 智能指针 | Box/Rc/Cell/RefCell/Cow/Weak/Drop/Deref |
//! | 09_async | 异步编程 | Future、async/await、Pin、运行时 |
//! | 10_error_handling | 错误处理 | Result、?运算符、thiserror/anyhow |
//! | 11_macro | 宏 | 声明宏、derive 宏、卫生性 |
//!
//! ## 🚀 第三阶段：工程实践（进阶）
//! | 模块 | 主题 | 关键概念 |
//! |------|------|----------|
//! | 12_testing | 测试 | 单元测试、集成测试、文档测试 |
//! | 13_networking | 网络编程 | reqwest、tokio、axum、serde |
//! | 14_logging | 日志 | log、env_logger、tracing |
//! | 15_ffi | C/C++ 互操作 | extern "C"、#[repr(C)]、bindgen |
//! | 16_cargo | 依赖管理 | Cargo.toml、feature、workspace |
//! | 17_unsafe | Unsafe Rust | 裸指针、UnsafeCell、FFI安全、Send/Sync |

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
mod _17_unsafe;

fn main() {
    println!("╔══════════════════════════════════════╗");
    println!("║       Rust 学习示例                  ║");
    println!("╚══════════════════════════════════════╝");

    println!("\n\n🔰 第一阶段：基础");
    println!("\n【1. 所有权系统】");
    _01_ownership::run();

    println!("\n\n【2. 多线程与共享数据】");
    _02_concurrency::run();

    println!("\n\n【3. 标准库组件】");
    _03_std_components::run();

    println!("\n\n🎯 第二阶段：核心特性");
    println!("\n【4. Trait 系统】");
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

    println!("\n\n🚀 第三阶段：工程实践");
    println!("\n【12. 单元测试】");
    _12_testing::run();

    println!("\n\n【13. 网络库】");
    _13_networking::run();

    println!("\n\n【14. 日志库】");
    _14_logging::run();

    println!("\n\n【15. C/C++ 互操作】");
    _15_ffi::run();

    println!("\n\n【16. Cargo 依赖管理】");
    _16_cargo::run();

    println!("\n\n【17. Unsafe Rust】");
    _17_unsafe::run();

    println!("\n\n✅ 所有示例运行完成！");
    println!("提示：用 cargo test 运行测试，cargo clippy 检查代码风格");
}
