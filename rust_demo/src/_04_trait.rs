#![allow(dead_code)]
//! # 04 - Trait 系统
//!
//! Trait 是 Rust 的核心抽象机制，类似于 C++ 的抽象类 + 概念(Concepts)。
//! 但更强大：支持默认实现、关联类型、约束继承等。
//!
//! C++ 对比：
//! - C++ 虚函数：运行时多态（vtable）
//! - Rust trait：编译时单态化（静态分发）或 dyn Trait（动态分发）

/// 基础 trait 定义与实现
pub fn basic_trait() {
    println!("=== 基础 Trait ===");

    // 定义 trait
    trait Summary {
        // 必须实现的方法
        fn summarize(&self) -> String;

        // 默认实现（C++ 中需要在基类中实现）
        fn preview(&self) -> String {
            format!("{}...", &self.summarize()[..20.min(self.summarize().len())])
        }
    }

    // 为类型实现 trait
    struct Article {
        title: String,
        content: String,
    }

    impl Summary for Article {
        fn summarize(&self) -> String {
            format!("{}: {}", self.title, &self.content[..50.min(self.content.len())])
        }
        // preview 使用默认实现
    }

    struct Tweet {
        username: String,
        text: String,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("@{}: {}", self.username, self.text)
        }

        // 覆盖默认实现
        fn preview(&self) -> String {
            format!("@{}: {}...", self.username, &self.text[..20.min(self.text.len())])
        }
    }

    let article = Article {
        title: String::from("Rust vs C++"),
        content: String::from("Rust provides memory safety without garbage collection..."),
    };

    let tweet = Tweet {
        username: String::from("rustlang"),
        text: String::from("Rust 1.96 released with amazing features!"),
    };

    println!("article: {}", article.summarize());
    println!("article preview: {}", article.preview());
    println!("tweet: {}", tweet.summarize());
    println!("tweet preview: {}", tweet.preview());
}

/// Trait 作为参数（静态分发 vs 动态分发）
pub fn trait_as_parameter() {
    println!("\n=== Trait 作为参数 ===");

    trait Drawable {
        fn draw(&self);
    }

    struct Circle { radius: f64 }
    struct Rectangle { width: f64, height: f64 }

    impl Drawable for Circle {
        fn draw(&self) { println!("  drawing circle r={:.1}", self.radius); }
    }
    impl Drawable for Rectangle {
        fn draw(&self) { println!("  drawing rect {:.1}x{:.1}", self.width, self.height); }
    }

    // 静态分发：编译时单态化，性能更好（类似 C++ 模板）
    fn draw_static(item: &impl Drawable) {
        item.draw();
    }

    // 语法糖：trait bound 形式
    fn draw_static2<T: Drawable>(item: &T) {
        item.draw();
    }

    // 多个 trait 约束
    fn draw_and_debug(item: &(impl Drawable + std::fmt::Debug)) {
        item.draw();
    }

    // 动态分发：运行时通过 vtable 调用（类似 C++ 虚函数）
    fn draw_dynamic(item: &dyn Drawable) {
        item.draw();
    }

    let circle = Circle { radius: 5.0 };
    let rect = Rectangle { width: 3.0, height: 4.0 };

    println!("静态分发:");
    draw_static(&circle);
    draw_static(&rect);

    println!("动态分发:");
    draw_dynamic(&circle);
    draw_dynamic(&rect);

    // dyn Trait 可以存储在集合中（类似 C++ 基类指针）
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 1.0 }),
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
    ];
    println!("动态分发集合:");
    for shape in &shapes {
        shape.draw();
    }
}

/// 关联类型：trait 中定义的类型占位符
pub fn associated_types() {
    println!("\n=== 关联类型 ===");

    // 关联类型让 trait 更简洁（每个类型只能有一个实现）
    trait Container {
        type Item;  // 关联类型

        fn get(&self) -> Option<&Self::Item>;
        fn set(&mut self, item: Self::Item);
    }

    struct SingleValue<T> {
        value: Option<T>,
    }

    impl<T> Container for SingleValue<T> {
        type Item = T;

        fn get(&self) -> Option<&T> {
            self.value.as_ref()
        }

        fn set(&mut self, item: T) {
            self.value = Some(item);
        }
    }

    let mut v = SingleValue { value: None };
    v.set(42);
    println!("value: {:?}", v.get());
}

/// Trait 继承
pub fn trait_inheritance() {
    println!("\n=== Trait 继承 ===");

    // 子 trait 继承父 trait 的方法
    trait Animal {
        fn name(&self) -> &str;
        fn sound(&self) -> &str;
    }

    // Pet 继承 Animal，额外要求 Display
    trait Pet: Animal + std::fmt::Display {
        fn owner(&self) -> &str;
    }

    struct Dog {
        name: String,
        owner: String,
    }

    impl std::fmt::Display for Dog {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Dog({})", self.name)
        }
    }

    impl Animal for Dog {
        fn name(&self) -> &str { &self.name }
        fn sound(&self) -> &str { "Woof!" }
    }

    impl Pet for Dog {
        fn owner(&self) -> &str { &self.owner }
    }

    fn introduce_pet(pet: &impl Pet) {
        println!("  {} says {}! Owner: {}", pet.name(), pet.sound(), pet.owner());
    }

    let dog = Dog {
        name: String::from("Buddy"),
        owner: String::from("Alice"),
    };
    introduce_pet(&dog);
}

/// 运算符重载：通过 trait 实现
pub fn operator_overload() {
    println!("\n=== 运算符重载 ===");

    #[derive(Debug, Clone, Copy)]
    struct Vec2 { x: f64, y: f64 }

    // 实现 Add trait 以支持 + 运算符
    impl std::ops::Add for Vec2 {
        type Output = Vec2;
        fn add(self, other: Vec2) -> Vec2 {
            Vec2 { x: self.x + other.x, y: self.y + other.y }
        }
    }

    // 实现 Mul trait 以支持 * 运算符
    impl std::ops::Mul<f64> for Vec2 {
        type Output = Vec2;
        fn mul(self, scalar: f64) -> Vec2 {
            Vec2 { x: self.x * scalar, y: self.y * scalar }
        }
    }

    // 实现 Display trait 以支持 println!
    impl std::fmt::Display for Vec2 {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({:.1}, {:.1})", self.x, self.y)
        }
    }

    let a = Vec2 { x: 1.0, y: 2.0 };
    let b = Vec2 { x: 3.0, y: 4.0 };
    println!("{} + {} = {}", a, b, a + b);
    println!("{} * 2 = {}", a, a * 2.0);
}

/// From/Into trait：类型转换
pub fn from_into() {
    println!("\n=== From/Into 类型转换 ===");

    #[derive(Debug)]
    struct Celsius(f64);
    #[derive(Debug)]
    struct Fahrenheit(f64);

    // 实现 From<Celsius> for Fahrenheit
    impl From<Celsius> for Fahrenheit {
        fn from(c: Celsius) -> Self {
            Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
        }
    }

    // 实现 From<Fahrenheit> for Celsius
    impl From<Fahrenheit> for Celsius {
        fn from(f: Fahrenheit) -> Self {
            Celsius((f.0 - 32.0) * 5.0 / 9.0)
        }
    }

    let boiling = Celsius(100.0);
    let f: Fahrenheit = boiling.into();  // 使用 Into trait
    println!("Celsius(100) -> {:?}", f);

    let body_temp = Fahrenheit(98.6);
    let c: Celsius = body_temp.into();
    println!("Fahrenheit(98.6) -> {:?}", c);

    // From trait 自动提供 Into，也支持 try_from
    let c2 = Celsius::from(Fahrenheit(212.0));
    println!("Fahrenheit(212) -> {:?}", c2);
}

/// 运行所有 trait 示例
pub fn run() {
    basic_trait();
    trait_as_parameter();
    associated_types();
    trait_inheritance();
    operator_overload();
    from_into();
}
