#![allow(dead_code)]
//! # 06 - 模式匹配
//!
//! Rust 的模式匹配比 C++ 的 switch 强大得多：
//! - 可以解构枚举、元组、结构体
//! - 可以绑定变量
//! - 可以使用守卫条件
//! - 编译器检查穷尽性

/// match 基础用法
pub fn basic_match() {
    println!("=== 基础 match ===");

    // match 类似 switch，但更强大
    let number = 13;
    match number {
        1 => println!("one"),
        2 | 3 => println!("two or three"),  // 多模式
        4..=10 => println!("four to ten"),   // 范围
        11..=20 => println!("eleven to twenty"),
        _ => println!("other"),               // 默认分支
    }

    // match 返回值
    let description = match number {
        1 => "one",
        2..=10 => "small",
        11..=100 => "medium",
        _ => "large",
    };
    println!("{} is {}", number, description);
}

/// 解构枚举
pub fn destructure_enum() {
    println!("\n=== 解构枚举 ===");

    #[derive(Debug)]
    enum Shape {
        Circle(f64),                    // 半径
        Rectangle(f64, f64),           // 宽、高
        Triangle { base: f64, height: f64 },
    }

    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 6.0, height: 8.0 },
    ];

    for shape in &shapes {
        let area = match shape {
            Shape::Circle(r) => {
                println!("  circle with radius {}", r);
                std::f64::consts::PI * r * r
            }
            Shape::Rectangle(w, h) => {
                println!("  rectangle {}x{}", w, h);
                w * h
            }
            Shape::Triangle { base, height } => {
                println!("  triangle base={} height={}", base, height);
                0.5 * base * height
            }
        };
        println!("  area = {:.2}", area);
    }
}

/// 解构结构体和元组
pub fn destructure_struct_tuple() {
    println!("\n=== 解构结构体和元组 ===");

    struct Point { x: i32, y: i32 }

    let point = Point { x: 10, y: 20 };

    // 解构结构体
    let Point { x, y } = point;
    println!("point: ({}, {})", x, y);

    // 部分解构
    let Point { x, .. } = point;
    println!("x only: {}", x);

    // 解构元组
    let tuple = (1, "hello", std::f64::consts::PI);
    let (a, b, c) = tuple;
    println!("tuple: ({}, {}, {})", a, b, c);

    // 嵌套解构
    let nested = ((1, 2), (3, 4));
    let ((a, b), (c, d)) = nested;
    println!("nested: ({}, {}, {}, {})", a, b, c, d);
}

/// 模式守卫（match guard）
pub fn match_guard() {
    println!("\n=== 模式守卫 ===");

    let num = Some(4);

    match num {
        Some(x) if x < 0 => println!("{} is negative", x),
        Some(0) => println!("zero"),
        Some(x) if x % 2 == 0 => println!("{} is positive even", x),
        Some(x) => println!("{} is positive odd", x),
        None => println!("none"),
    }

    // 守卫中可以使用外部变量
    let threshold = 10;
    let value = 15;
    match value {
        x if x > threshold => println!("{} above threshold {}", x, threshold),
        x if x == threshold => println!("{} at threshold", x),
        x => println!("{} below threshold", x),
    }
}

/// if let 和 while let
pub fn if_while_let() {
    println!("\n=== if let / while let ===");

    // if let：只关心一种模式
    let config_max: Option<u8> = Some(3);
    if let Some(max) = config_max {
        println!("max is {}", max);
    }

    // match 写法（等价于上面的 if let，可读性更好）
    // 单臂 match 等价于 if let，clippy 建议用 if let
    if let Some(max) = config_max {
        println!("max is {} (via match)", max);
    }

    // while let：循环直到模式不匹配
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();

    // let else：模式不匹配时提前返回
    fn get_first_char(s: &str) -> Option<char> {
        let first = s.chars().next()?;
        Some(first)
    }
    println!("first char: {:?}", get_first_char("hello"));
    println!("first char of empty: {:?}", get_first_char(""));
}

/// @ 绑定
pub fn at_binding() {
    println!("\n=== @ 绑定 ===");

    let age = 25;
    match age {
        // @ 绑定：既检查范围，又绑定值
        n @ 0..=12 => println!("child: {}", n),
        n @ 13..=17 => println!("teenager: {}", n),
        n @ 18..=64 => println!("adult: {}", n),
        n => println!("senior: {}", n),
    }

    // 在枚举解构中使用 @
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_val @ 3..=7 } => {
            println!("found id in range: {}", id_val);
        }
        Message::Hello { id } => {
            println!("other id: {}", id);
        }
    }
}

/// 穷尽性检查
pub fn exhaustiveness() {
    println!("\n=== 穷尽性检查 ===");

    // Rust 编译器确保 match 覆盖所有可能的值
    // 这是 C++ switch 不具备的安全特性

    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;

    // 必须处理所有变体，否则编译错误
    let name = match dir {
        Direction::North => "north",
        Direction::South => "south",
        Direction::East => "east",
        Direction::West => "west",
    };
    println!("direction: {}", name);

    // 使用 _ 通配符处理剩余情况
    let number: u8 = 42;
    match number {
        0 => println!("zero"),
        1..=100 => println!("between 1 and 100"),
        _ => println!("other"),
    }
}

/// 模式匹配的实际应用
pub fn practical_patterns() {
    println!("\n=== 实际应用 ===");

    // 1. 处理 Result
    fn parse_and_double(s: &str) -> Result<i32, String> {
        let num: i32 = s.parse().map_err(|e| format!("parse error: {}", e))?;
        Ok(num * 2)
    }

    match parse_and_double("21") {
        Ok(n) => println!("result: {}", n),
        Err(e) => println!("error: {}", e),
    }

    // 2. 解构函数参数
    fn print_point((x, y): (i32, i32)) {
        println!("point: ({}, {})", x, y);
    }
    print_point((10, 20));

    // 3. 解构迭代器
    let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
    for (num, ch) in &pairs {
        println!("  {} -> {}", num, ch);
    }

    // 4. 忽略不需要的值
    let (_, second, _) = (1, 2, 3);
    println!("second: {}", second);

    // 5. 结构体更新语法
    #[derive(Debug)]
    struct Config {
        width: u32,
        height: u32,
        title: String,
    }

    let base = Config {
        width: 800,
        height: 600,
        title: String::from("Default"),
    };

    let custom = Config {
        width: 1024,
        ..base  // 其余字段从 base 复制
    };
    println!("custom config: {:?}", custom);
}

/// 运行所有模式匹配示例
pub fn run() {
    basic_match();
    destructure_enum();
    destructure_struct_tuple();
    match_guard();
    if_while_let();
    at_binding();
    exhaustiveness();
    practical_patterns();
}
