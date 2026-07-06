//! # 09 - 异步编程
//!
//! Rust 异步编程基于 async/await 语法和 Future trait。
//! 生态系统以 tokio 为主流运行时。
//!
//! C++ 对比：
//! - C++20 co_await/co_yield/co_return
//! - Rust async fn + .await
//!
//! 注意：本示例展示异步编程概念，实际项目推荐 tokio。

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// 简易的执行器：阻塞等待一个 Future 完成
/// 仅用于演示，实际项目请使用 tokio 等正式运行时
fn block_on<F: Future>(future: F) -> F::Output {
    // noop waker：不唤醒任何任务（适用于不依赖唤醒的简单 Future）
    fn noop_waker() -> Waker {
        use std::task::{RawWaker, RawWakerVTable};
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);
    // 用 Box::pin 将 Future 固定在堆上，避免 unsafe 的手动 Pin 操作
    let mut future = Box::pin(future);
    loop {
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(result) => return result,
            Poll::Pending => continue,
        }
    }
}

/// Future trait 基础
pub fn future_basics() {
    println!("=== Future 基础 ===");

    // Future 是异步计算的抽象
    // trait Future {
    //     type Output;
    //     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
    // }

    // 手动实现一个简单的 Future
    struct Ready<T>(T);

    impl<T: Copy> Future for Ready<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
            Poll::Ready(self.0)
        }
    }

    // 使用简单的阻塞执行器
    let future = Ready(42);
    let result = block_on(future);
    println!("ready future result: {}", result);
}

/// async/await 语法
pub fn async_await_syntax() {
    println!("\n=== async/await 语法 ===");

    // async fn 返回一个 Future
    async fn fetch_data(id: u32) -> String {
        // 模拟异步操作
        format!("data_{}", id)
    }

    async fn process() {
        // .await 等待 Future 完成
        let data1 = fetch_data(1).await;
        let data2 = fetch_data(2).await;
        println!("  processed: {} and {}", data1, data2);
    }

    // 简单的阻塞执行器
    block_on(process());
}

/// async 块和闭包
pub fn async_blocks() {
    println!("\n=== async 块 ===");

    // async 块创建匿名 Future
    let future = async {
        println!("  executing async block");
        42
    };

    let result = block_on(future);
    println!("async block result: {}", result);

    // async move 块获取所有权
    let name = String::from("Alice");
    let future = async move {
        println!("  hello, {}", name);
        name.len()
    };

    let len = block_on(future);
    println!("name length: {}", len);
}

/// Future 组合器概念
pub fn future_combinators() {
    println!("\n=== Future 组合器 ===");

    // 实际项目中使用 futures crate 或 tokio 的组合器
    // 这里展示概念

    async fn double_async(x: i32) -> i32 {
        x * 2
    }

    async fn add_async(a: i32, b: i32) -> i32 {
        a + b
    }

    async fn chain() {
        // 顺序执行
        let a = double_async(5).await;
        let b = double_async(10).await;
        let sum = add_async(a, b).await;
        println!("  chain result: {}", sum);
    }

    block_on(chain());
}

/// 实际应用场景
pub fn practical_patterns() {
    println!("\n=== 实际应用场景 ===");

    // 模拟 HTTP 请求
    async fn fetch_url(url: &str) -> Result<String, String> {
        println!("  fetching: {}", url);
        // 模拟网络延迟
        Ok(format!("response from {}", url))
    }

    // 顺序执行多个异步操作
    async fn fetch_multiple() {
        let urls = vec![
            "https://api.example.com/users",
            "https://api.example.com/posts",
            "https://api.example.com/comments",
        ];

        for url in &urls {
            match fetch_url(url).await {
                Ok(response) => println!("  got: {}", response),
                Err(e) => println!("  error: {}", e),
            }
        }
    }

    block_on(fetch_multiple());

    // 错误处理
    async fn process_with_error() -> Result<(), String> {
        let data = fetch_url("https://api.example.com/data").await?;
        println!("  processed: {}", data);
        Ok(())
    }

    match block_on(process_with_error()) {
        Ok(()) => println!("  success"),
        Err(e) => println!("  error: {}", e),
    }
}

/// Pin 和 Unpin 概念
pub fn pin_concept() {
    println!("\n=== Pin 概念 ===");

    // Pin 确保 Future 不会被移动
    // 这对于自引用结构很重要

    // 大多数类型实现了 Unpin，可以安全移动
    let mut value = 42;
    let pinned = Pin::new(&mut value);
    println!("pinned value: {}", *pinned);

    // !Unpin 的类型需要 Pin 来保证不被移动
    // async fn 生成的 Future 通常是 !Unpin

    println!("Pin 保证数据不会在内存中移动");
    println!("这对于自引用结构和 async 很重要");
}

/// 异步运行时简介
pub fn async_runtime() {
    println!("\n=== 异步运行时 ===");

    // 异步运行时负责：
    // 1. 调度 Future
    // 2. 提供 I/O 事件循环
    // 3. 管理线程池

    println!("常用异步运行时：");
    println!("  tokio: 最流行，功能完整");
    println!("  async-std: 类似标准库的异步 API");
    println!("  smol: 轻量级运行时");

    println!("\ntokio 示例：");
    println!("  tokio::spawn(async {{ ... }})");
    println!("  tokio::time::sleep(Duration::from_secs(1)).await");
    println!("  tokio::fs::read_to_string(\"file.txt\").await");

    println!("\n选择建议：");
    println!("  一般项目：tokio");
    println!("  嵌入式/轻量：smol");
    println!("  学习/简单：async-std");
}

/// 运行所有异步示例
pub fn run() {
    future_basics();
    async_await_syntax();
    async_blocks();
    future_combinators();
    practical_patterns();
    pin_concept();
    async_runtime();
}
