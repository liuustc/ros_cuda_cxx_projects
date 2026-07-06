#![allow(dead_code)]
//! # 05 - 泛型
//!
//! 泛型让代码可以适用于多种类型，类似 C++ 模板。
//! 但 Rust 泛型有 trait 约束，更安全，错误信息更友好。
//!
//! C++ 对比：
//! - C++ 模板：编译期展开，错误信息复杂
//! - Rust 泛型：编译期单态化 + trait 约束，错误信息清晰

/// 函数泛型
pub fn generic_function() {
    println!("=== 函数泛型 ===");

    // 泛型函数：T 必须实现 PartialOrd
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in &list[1..] {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    println!("largest number: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("largest char: {}", largest(&chars));

    // 多个泛型参数
    fn pair<T: std::fmt::Debug, U: std::fmt::Debug>(first: T, second: U) {
        println!("  pair: ({:?}, {:?})", first, second);
    }

    pair(1, "hello");
    pair(std::f64::consts::PI, true);
}

/// 结构体泛型
pub fn generic_struct() {
    println!("\n=== 结构体泛型 ===");

    // 单一泛型参数
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // 为特定类型实现方法
    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    // 为所有类型实现方法
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }

        fn x(&self) -> &T {
            &self.x
        }
    }

    let integer_point = Point::new(5, 10);
    println!("integer point: {:?}", integer_point);

    let float_point = Point::new(1.0, 4.0);
    println!("float point: {:?}", float_point);
    println!("distance: {:.2}", float_point.distance_from_origin());

    // 多个泛型参数
    #[derive(Debug)]
    struct KeyValue<K, V> {
        key: K,
        value: V,
    }

    let entry = KeyValue { key: "name", value: 42 };
    println!("entry: {:?}", entry);
}

/// 枚举泛型
pub fn generic_enum() {
    println!("\n=== 枚举泛型 ===");

    // 标准库中的 Option 和 Result 就是泛型枚举
    // enum Option<T> { Some(T), None }
    // enum Result<T, E> { Ok(T), Err(E) }

    // 自定义泛型枚举
    #[derive(Debug)]
    enum Either<L, R> {
        Left(L),
        Right(R),
    }

    impl<L: std::fmt::Display, R: std::fmt::Display> Either<L, R> {
        fn describe(&self) -> String {
            match self {
                Either::Left(l) => format!("Left({})", l),
                Either::Right(r) => format!("Right({})", r),
            }
        }
    }

    let left: Either<i32, String> = Either::Left(42);
    let right: Either<i32, String> = Either::Right(String::from("hello"));

    println!("{}", left.describe());
    println!("{}", right.describe());
}

/// Trait 泛型约束
pub fn trait_bounds() {
    println!("\n=== Trait 约束 ===");

    // 简写形式
    fn print_debug<T: std::fmt::Debug>(item: &T) {
        println!("  debug: {:?}", item);
    }

    // 多重约束
    fn print_debug_and_display<T: std::fmt::Debug + std::fmt::Display>(item: &T) {
        println!("  debug: {:?}", item);
        println!("  display: {}", item);
    }

    // where 子句（更清晰的写法）
    fn process<T>(item: T) -> String
    where
        T: std::fmt::Debug + Clone,
    {
        let cloned = item.clone();
        format!("{:?} -> {:?}", item, cloned)
    }

    print_debug(&42);
    print_debug_and_display(&"hello");
    println!("process: {}", process(vec![1, 2, 3]));
}

/// 泛型性能：单态化
pub fn monomorphization() {
    println!("\n=== 单态化 ===");

    // Rust 泛型在编译时展开为具体类型，零运行时开销
    // 这与 C++ 模板相同

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }

    // 编译器会生成两个版本：
    // add_i32(i32, i32) -> i32
    // add_f64(f64, f64) -> f64
    println!("add i32: {}", add(1, 2));
    println!("add f64: {}", add(1.5, 2.5));

    // 泛型结构体也是单态化的
    #[derive(Debug)]
    struct Wrapper<T>(T);

    impl<T: std::fmt::Display> Wrapper<T> {
        fn show(&self) {
            println!("  wrapper: {}", self.0);
        }
    }

    let w1 = Wrapper(42);
    let w2 = Wrapper("hello");
    w1.show();
    w2.show();
}

/// 常量泛型（const generics）
pub fn const_generics() {
    println!("\n=== 常量泛型 ===");

    // 常量泛型允许在编译期使用常量值作为泛型参数
    fn array_sum<T, const N: usize>(arr: &[T; N]) -> T
    where
        T: std::iter::Sum<T> + Copy,
    {
        arr.iter().copied().sum()
    }

    let arr3 = [1, 2, 3];
    let arr5 = [10, 20, 30, 40, 50];
    println!("sum of 3 elements: {}", array_sum(&arr3));
    println!("sum of 5 elements: {}", array_sum(&arr5));

    // 常量泛型结构体
    #[derive(Debug)]
    struct Matrix<T, const ROWS: usize, const COLS: usize> {
        data: [[T; COLS]; ROWS],
    }

    impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
        fn new() -> Self {
            Matrix {
                data: [[T::default(); COLS]; ROWS],
            }
        }
    }

    let m: Matrix<i32, 2, 3> = Matrix::new();
    println!("matrix: {:?}", m);
}

/// 运行所有泛型示例
pub fn run() {
    generic_function();
    generic_struct();
    generic_enum();
    trait_bounds();
    monomorphization();
    const_generics();
}
