#![allow(dead_code)]
//! # 10 - 错误处理
//!
//! Rust 没有异常，使用 Result<T, E> 和 Option<T> 进行错误处理。
//! 编译器强制处理错误，避免运行时意外崩溃。
//!
//! C++ 对比：
//! - C++ 异常：运行时，可能被忽略
//! - Rust Result：编译时，必须处理
//! - C++ std::expected (C++23)：类似 Result

use std::fmt;
use std::num::ParseIntError;
use std::io;

/// 自定义错误类型
#[derive(Debug)]
enum AppError {
    Io(io::Error),
    Parse(ParseIntError),
    Custom(String),
}

// 实现 Display trait
impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO error: {}", e),
            AppError::Parse(e) => write!(f, "Parse error: {}", e),
            AppError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

// 实现 From trait 以支持 ? 运算符自动转换
impl From<io::Error> for AppError {
    fn from(e: io::Error) -> Self {
        AppError::Io(e)
    }
}

impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::Parse(e)
    }
}

/// Result 基础用法
pub fn result_basics() {
    println!("=== Result 基础 ===");

    // Result<T, E> 表示可能失败的操作
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    // match 处理 Result
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("error: {}", e),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("result: {}", result),
        Err(e) => println!("error: {}", e),
    }

    // unwrap_or：提供默认值
    let result = divide(10.0, 0.0).unwrap_or(0.0);
    println!("unwrap_or: {}", result);

    // unwrap_or_else：延迟计算默认值
    let result = divide(10.0, 0.0).unwrap_or_else(|e| {
        eprintln!("  error occurred: {}, using default", e);
        0.0
    });
    println!("unwrap_or_else: {}", result);

    // map：变换成功值
    let result = divide(10.0, 2.0).map(|x| x * 100.0);
    println!("map: {:?}", result);

    // and_then：链式操作
    let result = divide(10.0, 2.0)
        .and_then(|x| {
            if x > 3.0 {
                Ok(x * 2.0)
            } else {
                Err("result too small".to_string())
            }
        });
    println!("and_then: {:?}", result);
}

/// ? 运算符：错误传播
pub fn question_mark() {
    println!("\n=== ? 运算符 ===");

    // ? 运算符自动传播错误
    fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
        let num: i32 = s.parse()?;  // 如果失败，提前返回 Err
        Ok(num * 2)
    }

    println!("parse '21': {:?}", parse_and_double("21"));
    println!("parse 'abc': {:?}", parse_and_double("abc"));

    // 链式使用 ?
    fn process(input: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let num: i32 = input.parse()?;
        let doubled = num * 2;
        let result = format!("{}", doubled);
        Ok(result.parse()?)
    }

    println!("process '21': {:?}", process("21"));

    // 自定义错误类型 + ?
    fn read_config(path: &str) -> Result<String, AppError> {
        let content = std::fs::read_to_string(path)?;  // io::Error -> AppError
        let _value: i32 = content.trim().parse()?;      // ParseIntError -> AppError
        Ok(content)
    }

    match read_config("/tmp/test_config") {
        Ok(content) => println!("config: {}", content),
        Err(e) => println!("error: {}", e),
    }
}

/// Option 高级用法
pub fn option_advanced() {
    println!("\n=== Option 高级用法 ===");

    // Option 表示可能不存在的值
    fn find_user(id: u32) -> Option<String> {
        match id {
            1 => Some("Alice".to_string()),
            2 => Some("Bob".to_string()),
            _ => None,
        }
    }

    // map：变换内部值
    let name_len = find_user(1).map(|name| name.len());
    println!("name length: {:?}", name_len);

    // and_then：链式操作
    let result = find_user(1)
        .and_then(|name| {
            if name.starts_with('A') {
                Some(name.to_uppercase())
            } else {
                None
            }
        });
    println!("and_then: {:?}", result);

    // filter：条件过滤
    let result = find_user(1).filter(|name| name.len() > 3);
    println!("filter: {:?}", result);

    // zip：组合两个 Option
    let name = find_user(1);
    let age = Some(30);
    let combined = name.zip(age);
    println!("zip: {:?}", combined);

    // flatten：展平嵌套 Option
    let nested: Option<Option<i32>> = Some(Some(42));
    let flat = nested.flatten();
    println!("flatten: {:?}", flat);
}

/// 错误处理最佳实践
pub fn best_practices() {
    println!("\n=== 最佳实践 ===");

    // 1. 使用 ? 运算符而不是 unwrap
    fn good_practice(s: &str) -> Result<i32, ParseIntError> {
        let num = s.parse::<i32>()?;  // 推荐
        Ok(num * 2)
    }

    // 2. 避免 unwrap，除非确定不会失败
    fn safe_unwrap(v: &[i32]) -> i32 {
        *v.first().unwrap_or(&0)  // 安全的 unwrap
    }

    // 3. 使用 expect 提供错误信息
    fn with_expect(v: &[i32]) -> i32 {
        *v.first().expect("vector should not be empty")
    }

    // 4. 组合多个可能失败的操作
    fn process_values(values: &[&str]) -> Result<Vec<i32>, ParseIntError> {
        values.iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
    }

    println!("good_practice: {:?}", good_practice("21"));
    println!("safe_unwrap: {}", safe_unwrap(&[1, 2, 3]));
    println!("safe_unwrap empty: {}", safe_unwrap(&[]));

    let values = vec!["1", "2", "3"];
    println!("process_values: {:?}", process_values(&values));

    let values = vec!["1", "abc", "3"];
    println!("process_values with error: {:?}", process_values(&values));
}

/// 使用 thiserror 和 anyhow（推荐的错误处理 crate）
pub fn error_crates() {
    println!("\n=== 错误处理 Crate ===");

    // thiserror：派生宏简化自定义错误
    // #[derive(thiserror::Error)]
    // enum MyError {
    //     #[error("IO error: {0}")]
    //     Io(#[from] std::io::Error),
    //     #[error("Parse error: {0}")]
    //     Parse(#[from] std::num::ParseIntError),
    // }

    // anyhow：应用级错误处理
    // use anyhow::{Result, Context};
    //
    // fn process() -> Result<()> {
    //     let content = std::fs::read_to_string("config.txt")
    //         .context("failed to read config")?;
    //     Ok(())
    // }

    println!("thiserror: 简化自定义错误类型的定义");
    println!("anyhow: 应用级错误处理，自动转换错误类型");
    println!("建议: 库用 thiserror，应用用 anyhow");
}

/// 运行所有错误处理示例
pub fn run() {
    result_basics();
    question_mark();
    option_advanced();
    best_practices();
    error_crates();
}
