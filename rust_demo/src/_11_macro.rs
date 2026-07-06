#![allow(dead_code)]
//! # 11 - 宏
//!
//! Rust 宏在编译时展开代码，分为声明宏和过程宏。
//! 宏可以减少重复代码，实现元编程。
//!
//! C++ 对比：
//! - C++ #define：文本替换，不安全
//! - Rust 宏：语法树操作，类型安全

/// 声明宏（macro_rules!）
pub fn declarative_macros() {
    println!("=== 声明宏 ===");

    // 基本声明宏
    macro_rules! say_hello {
        () => {
            println!("Hello!");
        };
    }

    say_hello!();

    // 带参数的宏
    macro_rules! create_function {
        ($func_name:ident) => {
            fn $func_name() {
                println!("Function: {}", stringify!($func_name));
            }
        };
    }

    create_function!(foo);
    create_function!(bar);
    foo();
    bar();

    // 重复模式宏
    macro_rules! vec_of_strings {
        ($($s:expr),*) => {
            vec![$($s.to_string()),*]
        };
    }

    let names = vec_of_strings!["Alice", "Bob", "Charlie"];
    println!("names: {:?}", names);

    // 计算宏
    macro_rules! calculate {
        ($a:expr, +, $b:expr) => {
            $a + $b
        };
        ($a:expr, -, $b:expr) => {
            $a - $b
        };
        ($a:expr, *, $b:expr) => {
            $a * $b
        };
    }

    println!("calculate 1 + 2 = {}", calculate!(1, +, 2));
    println!("calculate 5 - 3 = {}", calculate!(5, -, 3));
    println!("calculate 4 * 6 = {}", calculate!(4, *, 6));
}

/// 常用标准库宏
pub fn stdlib_macros() {
    println!("\n=== 标准库宏 ===");

    // vec! 宏
    let v = vec![1, 2, 3];
    let v2 = vec![0; 5]; // 创建5个0
    println!("vec: {:?}", v);
    println!("vec of zeros: {:?}", v2);

    // println! 和 format! 宏
    let name = "Alice";
    let age = 30;
    println!("{} is {} years old", name, age);
    let s = format!("{} is {} years old", name, age);
    println!("formatted: {}", s);

    // dbg! 宏（调试用）
    let x = 5;
    let y = dbg!(x * 2); // 打印表达式和值
    println!("y = {}", y);

    // todo! 和 unimplemented! 宏
    // fn not_implemented() { todo!("implement this") }

    // include_str! 和 include_bytes! 宏
    // let content = include_str!("file.txt");

    // cfg! 和 #[cfg] 宏
    println!("is linux: {}", cfg!(target_os = "linux"));
    println!("is debug: {}", cfg!(debug_assertions));
}

/// 自定义实用宏
pub fn custom_utility_macros() {
    println!("\n=== 自定义实用宏 ===");

    // HashMap 字面量宏
    macro_rules! hashmap {
        ($($key:expr => $value:expr),* $(,)?) => {
            {
                let mut map = std::collections::HashMap::new();
                $(map.insert($key, $value);)*
                map
            }
        };
    }

    let scores = hashmap! {
        "Alice" => 100,
        "Bob" => 85,
        "Charlie" => 92,
    };
    println!("scores: {:?}", scores);

    // 测试辅助宏
    macro_rules! assert_approx_eq {
        ($a:expr, $b:expr) => {
            assert_approx_eq!($a, $b, 1e-6);
        };
        ($a:expr, $b:expr, $eps:expr) => {
            let a: f64 = $a;
            let b: f64 = $b;
            let diff = (a - b).abs();
            assert!(
                diff < $eps,
                "assertion failed: |{} - {}| = {} >= {}",
                a, b, diff, $eps
            );
        };
    }

    assert_approx_eq!(1.0, 1.0 + 1e-10);
    println!("approx eq passed");

    // 批量函数定义宏
    macro_rules! define_functions {
        ($($name:ident => $result:expr),*) => {
            $(
                fn $name() -> i32 { $result }
            )*
        };
    }

    define_functions! {
        get_one => 1,
        get_two => 2,
        get_three => 3
    }

    println!("one: {}, two: {}, three: {}", get_one(), get_two(), get_three());
}

/// 过程宏简介（derive 宏）
pub fn derive_macros() {
    println!("\n=== 过程宏 ===");

    // 派生宏自动为结构体生成 trait 实现
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = p1.clone();
    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
    println!("p1 == p2: {}", p1 == p2);

    // 常见的 derive 宏：
    // Debug: 调试打印
    // Clone: 深拷贝
    // Copy: 栈上复制
    // PartialEq/Eq: 相等比较
    // PartialOrd/Ord: 排序比较
    // Hash: 哈希计算
    // Default: 默认值

    #[derive(Debug, Default)]
    struct Config {
        width: u32,
        height: u32,
        title: String,
    }

    let config = Config::default();
    println!("default config: {:?}", config);

    // 自定义 derive 宏需要使用 proc_macro crate
    // 这需要单独的 crate 来定义

    println!("常用 derive 宏:");
    println!("  Debug: 调试打印");
    println!("  Clone: 深拷贝");
    println!("  Copy: 栈上复制");
    println!("  PartialEq: 相等比较");
    println!("  Hash: 哈希计算");
    println!("  Default: 默认值");
    println!("  serde::Serialize/Deserialize: 序列化");
}

/// 宏的卫生性
pub fn macro_hygiene() {
    println!("\n=== 宏的卫生性 ===");

    // Rust 宏是卫生的，不会意外捕获外部变量
    macro_rules! using_x {
        ($x:expr) => {
            {
                let x = $x;  // 这个 x 不会影响外部的 x
                x + 1
            }
        };
    }

    let x = 10;
    let result = using_x!(5);
    println!("x = {}, result = {}", x, result); // x 仍然是 10

    // 宏可以使用 $crate 引用当前 crate
    macro_rules! my_vec {
        ($($item:expr),*) => {
            vec![$($item),*]
        };
    }

    let v = my_vec![1, 2, 3];
    println!("my_vec: {:?}", v);
}

/// 运行所有宏示例
pub fn run() {
    declarative_macros();
    stdlib_macros();
    custom_utility_macros();
    derive_macros();
    macro_hygiene();
}
