/// # 所有权系统示例
///
/// Rust 的所有权系统是其最核心的特性，编译器在编译期检查内存安全，
/// 无需垃圾回收器。核心规则：
/// 1. 每个值有且只有一个所有者
/// 2. 所有者离开作用域时，值被自动 drop
/// 3. 可以通过 move 转移所有权，或通过 borrow 借用
///
/// 所有权转移（move）示例
pub fn ownership_move() {
    println!("=== 所有权转移 ===");

    // String 是堆分配的类型，赋值会转移所有权
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权转移给 s2，s1 不再有效

    // 编译错误：s1 已经失效
    // println!("{}", s1);
    println!("s2 = {}", s2);

    // clone 显式深拷贝，两个变量都有效
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // i32 是栈上的 Copy 类型，赋值会复制而非转移
    let x = 42;
    let y = x; // x 仍然有效
    println!("x = {}, y = {}", x, y);
}

/// 函数传参会转移所有权
pub fn ownership_function() {
    println!("\n=== 函数与所有权 ===");

    let s = String::from("world");

    // 传入函数时所有权转移
    takes_ownership(s);
    // println!("{}", s); // 编译错误：s 已被转移

    // 函数返回值也可以转移所有权
    let s2 = gives_ownership();
    println!("received: {}", s2);

    // 借用：传引用而不转移所有权
    let s3 = String::from("borrowed");
    let len = calculate_length(&s3); // 借用 s3
    println!("\"{}\" has length {}", s3, len); // s3 仍然有效
}

fn takes_ownership(s: String) {
    println!("took ownership of: {}", s);
} // s 在这里被 drop

fn gives_ownership() -> String {
    String::from("given value")
} // 返回值的所有权转移给调用者

fn calculate_length(s: &str) -> usize {
    s.len()
} // s 是引用，不拥有所有权，不会 drop

/// 可变借用示例
pub fn mutable_borrow() {
    println!("\n=== 可变借用 ===");

    let mut s = String::from("hello");

    // 可变借用：可以修改数据
    change(&mut s);
    println!("after change: {}", s);

    // 限制：同一时刻只能有一个可变借用，或多个不可变借用
    // 这防止了数据竞争
    let r1 = &s; // 不可变借用
    let r2 = &s; // 另一个不可变借用，OK
    println!("r1 = {}, r2 = {}", r1, r2);
    // let r3 = &mut s; // 编译错误：已有不可变借用时不能可变借用

    // 不可变借用的作用域在最后一次使用后结束（NLL）
    println!("r1 last use: {}", r1);
    // 此后可以可变借用
    let r3 = &mut s;
    r3.push_str(" world");
    println!("r3 = {}", r3);
}

fn change(s: &mut String) {
    s.push_str(", world");
}

/// 切片（slice）借用示例
pub fn slice_borrow() {
    println!("\n=== 切片借用 ===");

    let s = String::from("hello world");

    // 字符串切片：借用字符串的一部分
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 简写语法
    let hello2 = &s[..5];   // 从开头到索引5（不含）
    let world2 = &s[6..];   // 从索引6到结尾
    let whole = &s[..];     // 整个字符串
    println!("{} {} {}", hello2, world2, whole);

    // 数组切片
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // [2, 3]
    println!("slice: {:?}", slice);
}

/// 生命周期标注示例
pub fn lifetime_demo() {
    println!("\n=== 生命周期 ===");

    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("xyz");
        // 函数签名要求两个参数和返回值的生命周期一致
        result = longest(string1.as_str(), string2.as_str());
        println!("longest: {}", result);
    }
    // 注意：result 在 string2 被 drop 后仍然可以使用，
    // 因为返回值的生命周期与 string1 一致（更长的那个）

    // 结构体中的生命周期标注
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        excerpt = ImportantExcerpt {
            part: &novel[..i],
        };
        println!("excerpt: {}", excerpt.part);
    }
    // excerpt 在 novel 有效期间都可以使用
    println!("excerpt still valid: {}", excerpt.part);
}

/// 生命周期标注：告诉编译器返回值的生命周期与参数的关系
/// 'a 表示返回值的生命周期与 x、y 中较短的那个一致
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

/// 结构体中的生命周期：告诉编译器 ImportantExcerpt 不能比其引用的数据活得更久
struct ImportantExcerpt<'a> {
    part: &'a str,
}

/// 所有权系统综合示例：展示实际应用中的所有权模式
pub fn ownership_patterns() {
    println!("\n=== 所有权模式 ===");

    // 模式1：Option 转移所有权
    let names = vec![String::from("Alice"), String::from("Bob")];
    let first = names.into_iter().next(); // into_iter 消耗 vec，转移所有权
    println!("first name: {:?}", first);

    // 模式2：借用集合中的元素
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum(); // iter() 借用，不消耗
    println!("numbers: {:?}, sum: {}", numbers, sum);

    // 模式3：Cow（Clone on Write）：需要时才克隆
    use std::borrow::Cow;
    let s: Cow<str> = Cow::Borrowed("hello");
    let owned: String = s.into_owned(); // 转为拥有所有权的 String
    println!("owned: {}", owned);
}

/// 运行所有所有权示例
pub fn run() {
    ownership_move();
    ownership_function();
    mutable_borrow();
    slice_borrow();
    lifetime_demo();
    ownership_patterns();
}
