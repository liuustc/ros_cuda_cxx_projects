//! # 12 - 单元测试
//!
//! Rust 内置测试框架，通过 cargo test 运行。
//! 支持单元测试、集成测试、文档测试。
//!
//! C++ 对比：
//! - C++ 需要外部框架（gtest、catch2）
//! - Rust 内置，无需配置

/// 基础单元测试
#[cfg(test)]
mod basic_tests {
    // 被测试的函数
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err("division by zero".to_string())
        } else {
            Ok(a / b)
        }
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0).unwrap(), 5.0);
        assert!(divide(10.0, 0.0).is_err());
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_panic() {
        assert_eq!(1, 2, "assertion failed: 1 != 2");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        let result = divide(10.0, 2.0)?;
        if result != 5.0 {
            return Err(format!("expected 5.0, got {}", result));
        }
        Ok(())
    }
}

/// 测试辅助宏和工具
#[cfg(test)]
mod helper_tests {
    // 自定义断言宏
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a, $b, 1e-6);
        };
        ($a:expr, $b:expr, $eps:expr) => {
            let (a, b) = ($a as f64, $b as f64);
            assert!((a - b).abs() < $eps, "{} ≈ {} (diff={})", a, b, (a - b).abs());
        };
    }

    #[test]
    fn test_approx() {
        assert_approx_eq!(1.0, 1.0 + 1e-10);
        assert_approx_eq!(3.14159, std::f64::consts::PI, 0.001);
    }

    // 测试私有函数
    fn private_helper(x: i32) -> i32 {
        x * 2
    }

    #[test]
    fn test_private() {
        assert_eq!(private_helper(5), 10);
    }
}

/// 测试结构体方法
#[cfg(test)]
mod struct_tests {
    #[derive(Debug, PartialEq)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    #[test]
    fn test_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_can_hold() {
        let large = Rectangle::new(10, 20);
        let small = Rectangle::new(5, 10);
        assert!(large.can_hold(&small));
        assert!(!small.can_hold(&large));
    }
}

/// 参数化测试（使用测试函数生成器）
#[cfg(test)]
mod parametric_tests {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }

    // 测试多个用例
    #[test]
    fn test_is_even() {
        let cases = vec![
            (0, true),
            (1, false),
            (2, true),
            (3, false),
            (-2, true),
            (-1, false),
        ];

        for (input, expected) in cases {
            assert_eq!(
                is_even(input),
                expected,
                "is_even({}) should be {}",
                input,
                expected
            );
        }
    }

    // 使用迭代器测试
    #[test]
    fn test_even_numbers() {
        let evens: Vec<i32> = (0..10).filter(|&x| is_even(x)).collect();
        assert_eq!(evens, vec![0, 2, 4, 6, 8]);
    }
}

/// 测试错误处理
#[cfg(test)]
mod error_tests {
    #[derive(Debug, PartialEq)]
    enum Error {
        InvalidInput,
        Overflow,
    }

    // 用 checked_add 防止溢出
    fn safe_add(a: i32, b: i32) -> Result<i32, Error> {
        a.checked_add(b).ok_or(Error::Overflow)
    }

    // 带输入校验：拒绝负数
    fn positive_add(a: i32, b: i32) -> Result<i32, Error> {
        if a < 0 || b < 0 {
            return Err(Error::InvalidInput);
        }
        a.checked_add(b).ok_or(Error::Overflow)
    }

    #[test]
    fn test_success() {
        assert_eq!(safe_add(1, 2), Ok(3));
    }

    #[test]
    fn test_overflow() {
        assert_eq!(safe_add(i32::MAX, 1), Err(Error::Overflow));
    }

    #[test]
    fn test_invalid_input() {
        // 负数被拒绝，触发 InvalidInput
        assert_eq!(positive_add(-1, 5), Err(Error::InvalidInput));
        assert_eq!(positive_add(1, -5), Err(Error::InvalidInput));
    }

    #[test]
    fn test_positive_add_success() {
        assert_eq!(positive_add(10, 20), Ok(30));
    }
}

/// 测试并发代码
#[cfg(test)]
mod concurrency_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;

    #[test]
    fn test_counter() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_channel() {
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();
        let handle = thread::spawn(move || {
            for i in 0..5 {
                tx.send(i).unwrap();
            }
        });

        let received: Vec<i32> = rx.iter().collect();
        handle.join().unwrap();
        assert_eq!(received, vec![0, 1, 2, 3, 4]);
    }
}

/// 性能测试（基准测试的简单替代）
#[cfg(test)]
mod bench_tests {
    use std::time::Instant;

    #[test]
    fn test_performance() {
        let start = Instant::now();

        // 测试代码
        let sum: u64 = (0..1_000_000).sum();

        let duration = start.elapsed();
        println!("sum = {}, time = {:?}", sum, duration);

        // 基本断言
        assert_eq!(sum, 499_999_500_000);
        assert!(duration.as_millis() < 1000, "too slow: {:?}", duration);
    }
}

/// 集成测试提示
pub fn testing_tips() {
    println!("=== 测试最佳实践 ===");
    println!("1. 单元测试：放在 #[cfg(test)] mod 中");
    println!("2. 集成测试：放在 tests/ 目录下");
    println!("3. 文档测试：在 /// 注释中写代码示例");
    println!("4. 运行测试：cargo test");
    println!("5. 运行特定测试：cargo test test_name");
    println!("6. 显示输出：cargo test -- --nocapture");
}

/// 集成测试 vs 单元测试
pub fn integration_test_demo() {
    println!("\n=== 集成测试 ===");
    println!("集成测试放在项目的 tests/ 目录下（与 src/ 同级）：");
    println!();
    println!("  tests/");
    println!("  ├── integration_test.rs  // 每个文件是一个独立的测试 crate");
    println!("  └── common/");
    println!("      └── mod.rs           // 共享的测试辅助模块");
    println!();
    println!("集成测试示例 (tests/api_test.rs)：");
    println!("  use my_crate::some_function;");
    println!();
    println!("  #[test]");
    println!("  fn test_workflow() {{");
    println!("      let result = some_function(42);");
    println!("      assert_eq!(result, 84);");
    println!("  }}");
    println!();
    println!("运行集成测试：cargo test --test integration_test");
    println!();
    println!("单元测试 vs 集成测试：");
    println!("  单元测试: #[cfg(test)] mod tests {{ ... }} 在源码文件中");
    println!("    - 可以访问私有函数");
    println!("    - 与源码紧密耦合");
    println!("    - 运行更快");
    println!("  集成测试: tests/ 目录");
    println!("    - 只能访问公开 API");
    println!("    - 测试完整功能流程");
    println!("    - 模拟真实使用场景");
}

/// 文档测试示例
pub fn doc_test_demo() {
    println!("\n=== 文档测试 ===");
    println!("文档测试写在 /// 注释中，cargo test 会编译并运行它们：");
    println!();
    println!("  /// 返回两个数的和");
    println!("  ///");
    println!("  /// # Examples");
    println!("  ///");
    println!("  /// ```");
    println!("  /// let result = my_crate::add(2, 3);");
    println!("  /// assert_eq!(result, 5);");
    println!("  /// ```");
    println!("  pub fn add(a: i32, b: i32) -> i32 {{ a + b }}");
    println!();
    println!("文档测试的好处：");
    println!("  1. 代码示例自动验证，不会过时");
    println!("  2. 生成的文档中包含可运行的示例");
    println!("  3. 运行：cargo test --doc");
}

/// 运行示例
pub fn run() {
    testing_tips();
    integration_test_demo();
    doc_test_demo();
    println!("\n运行测试：cargo test");
}
