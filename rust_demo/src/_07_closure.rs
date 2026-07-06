#![allow(dead_code)]
//! # 07 - 闭包
//!
//! 闭包是可以捕获环境的匿名函数，类似 C++ lambda。
//! Rust 闭包有三种 trait：Fn、FnMut、FnOnce，对应不同的捕获方式。
//!
//! C++ 对比：
//! - C++ lambda：`[capture](params) { body }`
//! - Rust 闭包：`|params| { body }`，捕获方式自动推导

/// 闭包基础语法
pub fn closure_basics() {
    println!("=== 闭包基础 ===");

    // 基本闭包
    let add = |x, y| x + y;
    println!("add: {}", add(1, 2));

    // 带类型标注的闭包
    let multiply = |x: i32, y: i32| -> i32 { x * y };
    println!("multiply: {}", multiply(3, 4));

    // 多行闭包
    let complex = |x: i32| {
        let doubled = x * 2;
        
        doubled + 10
    };
    println!("complex: {}", complex(5));

    // 闭包作为变量
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),
        Box::new(|x| x * 2),
        Box::new(|x| x * x),
    ];

    for op in &operations {
        print!("{} ", op(5));
    }
    println!();
}

/// 闭包捕获环境
pub fn closure_capture() {
    println!("\n=== 捕获环境 ===");

    let name = String::from("Alice");
    let age = 30;

    // 不可变借用（Fn）：可以多次调用
    let greet = || println!("  hello, {} (age {})", name, age);
    greet();
    greet();  // 可以再次调用
    println!("name still valid: {}", name);

    // 可变借用（FnMut）：可以修改捕获的变量
    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("  count: {}", count);
    };
    increment();
    increment();
    println!("final count: {}", count);

    // 所有权转移（FnOnce）：只能调用一次
    let data = vec![1, 2, 3];
    let consume = move || {
        println!("  consumed: {:?}", data);
        // data 的所有权已转移到闭包中
    };
    consume();
    // println!("{:?}", data); // 编译错误：data 已被 move

    // 强制 move
    let value = 42;
    let closure = move || println!("  moved value: {}", value);
    closure();
    println!("value still valid: {}", value); // i32 是 Copy，所以仍有效
}

/// 三种闭包 trait
pub fn closure_traits() {
    println!("\n=== 三种闭包 Trait ===");

    // Fn：不可变借用捕获，可多次调用
    fn call_fn(f: &dyn Fn()) {
        f();
        f();  // 可以多次调用
    }

    // FnMut：可变借用捕获，可多次调用
    fn call_fn_mut(mut f: impl FnMut()) {
        f();
        f();
    }

    // FnOnce：获取所有权，只能调用一次
    fn call_fn_once(f: impl FnOnce()) {
        f();
        // f(); // 编译错误：FnOnce 只能调用一次
    }

    let msg = String::from("hello");

    // Fn：不可变借用
    let fn_closure = || println!("  Fn: {}", msg);
    call_fn(&fn_closure);

    // FnMut：可变借用
    let mut counter = 0;
    let mut fn_mut_closure = || {
        counter += 1;
        println!("  FnMut: {}", counter);
    };
    call_fn_mut(&mut fn_mut_closure);

    // FnOnce：所有权转移
    let owned = String::from("owned");
    let fn_once_closure = move || {
        println!("  FnOnce: {}", owned);
    };
    call_fn_once(fn_once_closure);
}

/// 闭包作为参数和返回值
pub fn closure_as_param_return() {
    println!("\n=== 闭包作为参数和返回值 ===");

    // 闭包作为参数（impl Fn 语法）
    fn apply(f: impl Fn(i32) -> i32, x: i32) -> i32 {
        f(x)
    }

    let double = |x| x * 2;
    let square = |x| x * x;
    println!("apply double: {}", apply(double, 5));
    println!("apply square: {}", apply(square, 5));

    // 闭包作为返回值（必须使用 impl Fn 或 dyn Fn）
    fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
        move |x| x + n
    }

    let add5 = make_adder(5);
    let add10 = make_adder(10);
    println!("add5(3) = {}", add5(3));
    println!("add10(3) = {}", add10(3));

    // 返回闭包的另一种方式（Box<dyn Fn>）
    fn make_operation(op: &str) -> Box<dyn Fn(i32, i32) -> i32> {
        match op {
            "add" => Box::new(|a, b| a + b),
            "mul" => Box::new(|a, b| a * b),
            _ => Box::new(|a, b| a - b),
        }
    }

    let add_op = make_operation("add");
    println!("custom add: {}", add_op(3, 4));
}

/// 闭包与迭代器（最常见的用法）
pub fn closure_with_iterators() {
    println!("\n=== 闭包与迭代器 ===");

    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map：变换
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    // filter：过滤
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // fold：归约
    let sum = numbers.iter().sum::<i32>();
    println!("sum: {}", sum);

    // 链式调用
    let result: i32 = numbers.iter()
        .filter(|&&x| x % 2 == 0)  // 偶数
        .map(|x| x * x)             // 平方
        .sum();                      // 求和
    println!("sum of even squares: {}", result);

    // sort_by 自定义排序
    let mut data = vec!["banana", "apple", "cherry", "date"];
    data.sort_by_key(|a| a.len());
    println!("sorted by length: {:?}", data);

    // for_each
    numbers.iter().for_each(|x| print!("{} ", x));
    println!();
}

/// 闭包性能：零成本抽象
pub fn closure_performance() {
    println!("\n=== 闭包性能 ===");

    // Rust 闭包是零成本抽象
    // 编译器会将闭包内联，与手写函数性能相同

    // 这个闭包会被内联，没有函数调用开销
    let result: i32 = (1..=100)
        .map(|x| x * x)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("optimized computation: {}", result);

    // 闭包大小在编译时确定
    let x = 42;
    let closure = move || x;  // 捕获一个 i32，闭包大小 = sizeof(i32)
    println!("closure size: {}", std::mem::size_of_val(&closure));
}

/// 运行所有闭包示例
pub fn run() {
    closure_basics();
    closure_capture();
    closure_traits();
    closure_as_param_return();
    closure_with_iterators();
    closure_performance();
}
