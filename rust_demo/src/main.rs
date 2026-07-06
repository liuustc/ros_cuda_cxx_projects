fn main() {
    println!("Hello, Rust!");

    // 变量与可变性
    let x = 5;
    let mut y = 10;
    println!("x = {}, y = {}", x, y);
    y += 5;
    println!("y after mutation = {}", y);

    // 元组与数组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    println!("tuple: ({}, {}, {})", a, b, c);

    let arr = [1, 2, 3, 4, 5];
    println!("array: {:?}", arr);

    // 函数调用
    let result = add(3, 7);
    println!("add(3, 7) = {}", result);

    // 控制流
    let number = 7;
    if number > 5 {
        println!("{} is greater than 5", number);
    } else {
        println!("{} is not greater than 5", number);
    }

    // 循环
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();

    // 所有权与借用
    let s1 = String::from("hello");
    let s2 = &s1; // 借用
    println!("s1 = {}, s2 = {}", s1, s2);

    // 结构体
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect area = {}", rect.area());

    // 枚举与模式匹配
    let coin = Coin::Quarter;
    println!("coin value = {}", value_in_cents(coin));

    // 错误处理
    match safe_divide(10.0, 3.0) {
        Some(result) => println!("10 / 3 = {:.2}", result),
        None => println!("division by zero"),
    }

    // 迭代器
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("sum of {:?} = {}", numbers, sum);

    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);

    println!("\nAll examples completed!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}
