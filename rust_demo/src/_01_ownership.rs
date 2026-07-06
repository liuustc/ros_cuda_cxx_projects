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

    let mut s = String::from("hello world");

    // 字符串切片：借用字符串的一部分
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    // 简写语法
    let hello2 = &s[..5]; // 从开头到索引5（不含）
    let world2 = &s[6..]; // 从索引6到结尾
    let whole = &s[..]; // 整个字符串
    println!("{} {} {}", hello2, world2, whole);

    // 可变字符串切片：通过 &mut str 安全原地修改（make_ascii_uppercase 内部即逐字节改）
    let borrow_mut_from_s: &mut str = &mut s[..5];
    borrow_mut_from_s.make_ascii_uppercase();
    println!("borrow_mut_from_s: {}", borrow_mut_from_s);
    println!("s: {}", s);

    // str 是 UTF-8 变长编码，没有 chars_mut 方法，无法“逐个 &mut char 原地替换”。
    // 若要逐字符原地修改，需先转成 Vec<char>：每个 char 固定 4 字节，可安全 iter_mut。
    let mut chars: Vec<char> = s.chars().collect();
    for c in chars.iter_mut() {
        *c = c.to_ascii_uppercase();
    }
    println!(
        "chars uppercased: {}",
        chars.into_iter().collect::<String>()
    );

    let mut my_data: Vec<i32> = vec![1, 2, 3, 4, 5];
    let data_slice: &mut [i32] = &mut my_data[1..3]; // [2, 3]
    println!("data_slice: {:?}", data_slice);

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
    // 注意：result 的生命周期是 string1 和 string2 中较短的那个（即 string2），
    // 因为 longest 的签名要求 x 和 y 共享相同的生命周期 'a。
    // 所以 result 只能在内层作用域内使用，不能在 string2 drop 后使用。

    // 结构体中的生命周期标注
    let novel = String::from("Call me Ishmael. Some years ago...");
    let excerpt;
    {
        let i = novel.find('.').unwrap_or(novel.len());
        excerpt = ImportantExcerpt { part: &novel[..i] };
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

/// 生命周期省略规则（Lifetime Elision）
///
/// Rust 编译器有三条省略规则，满足条件时无需手动标注生命周期：
/// 1. 每个引用参数都有自己的生命周期参数
/// 2. 若只有一个输入生命周期参数，则将其赋给所有输出生命周期参数
/// 3. 若方法中有 &self 或 &mut self，则 self 的生命周期赋给所有输出生命周期参数
pub fn lifetime_elision() {
    // ===== 前置概念：生命周期标注的本质 =====
    // 1. 它是「契约」而非「提示」：编写者向编译器声明引用之间的存活关系，
    //    编译器据此双向校验——既查函数体是否违约，也查调用方是否违约。
    // 2. 它约束的是「引用能用多久」，而非「数据活多久」：值活多久由作用域/Drop
    //    决定；标注只确保「引用永远不会比它指向的数据活得更久」。
    // 3. 零运行时成本：编译期完全擦除，二进制里无任何痕迹，是静态分析工具。
    // 4. 它防护编写者错误：宁可拒绝部分运行安全的代码，也不放过一个潜在悬垂引用；
    //    被拒时通常需重构（返回所有权、用 Rc/Arc、或让引用有明确来源），而非改标注。
    println!("\n=== 生命周期省略规则 ===");

    // 规则 1：每个引用参数获得独立的生命周期
    // fn foo(x: &i32, y: &i32) 等价于 fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

    // 规则 2：只有 1 个输入引用 → 输出也用该生命周期（无需标注）
    fn first_word(s: &str) -> &str {
        // 编译器自动补充为：fn first_word<'a>(s: &'a str) -> &'a str
        match s.find(' ') {
            Some(pos) => &s[..pos],
            None => s,
        }
    }
    println!("first word of 'hello world': {}", first_word("hello world"));

    // 规则 3：&self/&mut self 方法 → 输出引用获得 self 的生命周期
    struct Person {
        name: String,
    }
    impl Person {
        // 省略后等价于：fn get_name<'a>(&'a self) -> &'a str
        fn get_name(&self) -> &str {
            &self.name
        }
    }
    let p = Person { name: String::from("Alice") };
    println!("name: {}", p.get_name());

    // 需要手动标注的情况：多个输入生命周期 + 输出与参数有关联
    // fn longest(x: &str, y: &str) -> &str { ... } // 编译错误！编译器不知道返回哪个参数的生命周期
    println!("当有多个输入引用且输出引用需要与输入关联时，必须手动标注生命周期。");
    println!("例如 fn longest<'a>(x: &'a str, y: &'a str) -> &'a str");

    // ===== 进阶 case 1：多个输入引用，输出只借用“其中一个” =====
    // 返回值只可能引用 x —— 因为 y 没标 'a，类型上无法用于返回的 &'a str。
    // 故只给 x 标注 'a，y 由编译器分配独立匿名生命周期，与返回值无关。
    fn pick_first<'a>(x: &'a str, _y: &str) -> &'a str {
        x
    }
    println!("case1 pick_first: {}", pick_first("hello", "ignored"));

    // ===== 进阶 case 2：输出同时借用多个输入 → 多个生命周期 + 边界 =====
    struct Pair<'a, 'b> {
        x: &'a str,
        y: &'b str,
    }
    // 返回的 Pair 内部同时持有对 x、y 的引用；
    // 边界 'b: 'a 要求“y 至少活得和 x 一样久”，否则 Pair.y 会悬垂。
    fn make_pair<'a, 'b: 'a>(x: &'a str, y: &'b str) -> Pair<'a, 'b> {
        Pair { x, y }
    }
    let p2 = make_pair("short", "long lived string");
    println!("case2 pair: ({}, {})", p2.x, p2.y);

    // ===== 进阶 case 3：输出完全不借用任何输入 → 不需要标注 =====
    // 返回拥有所有权的 String，与所有入参生命周期无关。
    fn build_label(_a: &str, _b: &str, n: i32) -> String {
        format!("count={}", n)
    }
    // 返回 &'static str（字面量，存活整个程序），同样与入参无关。
    fn fallback(_a: &str, _b: &str) -> &'static str {
        "default"
    }
    println!("case3 build_label: {}", build_label("x", "y", 42));
    println!("case3 fallback: {}", fallback("x", "y"));

    // ===== 进阶 case 4：想“返回内部新建数据的引用”？不可能，改为返回所有权 =====
    // ❌ 错误写法（无法编译）：tmp 是局部变量，函数结束即 drop，返回其引用会悬垂。
    // fn bad() -> &str { let tmp = String::from("hi"); &tmp }
    // ✅ 正确写法：返回拥有所有权的 String，生命周期由调用方掌控。
    fn good() -> String {
        String::from("hi")
    }
    println!("case4 owned returned: {}", good());

    // ===== 进阶 case 5：高阶生命周期 for<'a>（HRTB）=====
    // 当函数接受一个“对任意生命周期的引用都返回同生命周期引用”的函数/闭包时使用。
    // for<'b> 表示“对所有的 'b 都成立”，与任何具体输入生命周期无关。
    // 注意：这里必须用“函数项”而非普通闭包——普通闭包不会自动推断为高阶生命周期类型，
    // 而函数项 fn(&str)->&str 天然满足 for<'b> Fn(&'b str) -> &'b str。
    fn call_with<'a, F>(s: &'a str, f: F) -> &'a str
    where
        F: for<'b> Fn(&'b str) -> &'b str,
    {
        f(s) // s: &'a str 实例化 'b = 'a，返回 &'a str
    }
    fn id(x: &str) -> &str { x } // 函数项天然满足 for<'b> Fn(&'b str) -> &'b str
    println!("case5 hrtb: {}", call_with("hello", id));

    println!();
    println!("省略规则总结：");
    println!("  输入引用 → 自动获得独立生命周期");
    println!("  1 个输入 + 输出 → 自动关联");
    println!("  &self/&mut self + 输出 → 自动关联 self");
    println!("  其他情况 → 必须手动标注");
}

/// 'static 生命周期详解
pub fn lifetime_static() {
    println!("\n=== 'static 生命周期 ===");

    // 'static 表示引用在整个程序运行期间都有效
    // 最常见的来源：字符串字面量（编译进二进制）
    let s: &'static str = "I live forever";
    println!("static str: {}", s);

    // 'static 不等于"全局变量"，它只是一个生命周期约束
    // 拥有所有权的类型（String, Vec 等）内部数据也满足 'static
    // 因为它们不受引用生命周期限制
    let _owned = String::from("owned data");
    // &owned 不是 'static，它借用 owned，owned drop 后引用失效

    // 泛型约束中的 'static：要求 T 不包含非 'static 引用
    fn print_static<T: std::fmt::Display + 'static>(val: T) {
        println!("static value: {}", val);
    }
    print_static(42);           // i32 满足 'static
    print_static("hello".to_string()); // String 满足 'static
    // print_static(&owned);   // 编译错误：&String 不满足 'static

    // 什么时候用 'static？
    // 1. 线程：thread::spawn 要求闭包捕获的数据是 'static
    // 2. 错误处理：Box<dyn Error + 'static>
    // 3. 懒初始化：lazy_static! / OnceLock
    println!();
    println!("'static 使用场景：");
    println!("  - 字符串字面量");
    println!("  - thread::spawn 的闭包捕获");
    println!("  - dyn Error + 'static");
    println!("  - 泛型约束 T: 'static");
}

/// 生命周期子类型与型变
pub fn lifetime_variance() {
    println!("\n=== 生命周期子类型与型变 ===");

    // 生命周期子类型：'a: 'b 表示 'a 至少和 'b 一样长
    // 'static 是所有生命周期的子类型（最长的生命周期）
    // 即 'static: 'a 对任意 'a 成立

    // 协变（Covariant）：'a 能用的地方，'b（更长）也能用
    // 如果 'a: 'b，则 &'a T 可以用在需要 &'b T 的地方
    fn longer_lives<'a>(x: &'a str, _y: &str) -> &'a str {
        x // y 的生命周期更短，不影响返回值
    }
    let long = String::from("I live long");
    {
        let short = String::from("short");
        // long 的生命周期比 short 长，所以可以用在需要更长生命周期的地方
        let result = longer_lives(&long, &short);
        println!("longer lives: {}", result);
    }

    // 不变（Invariant）：&mut T 对 T 是不变的
    // 不能把 &'a mut &'long T 当作 &'a mut &'short T 使用
    // 这是为了防止通过可变引用写入不兼容的生命周期
    println!();
    println!("型变规则：");
    println!("  &'a T    对 'a 协变，对 T 协变");
    println!("  &'a mut T 对 'a 协变，对 T 不变");
    println!("  fn(T) -> U  对 T 逆变，对 U 协变");
    println!("  Vec<T>     对 T 协变");
    println!("  Cell<T>    对 T 不变");
}

/// 生命周期与 trait 对象
pub fn lifetime_trait_objects() {
    println!("\n=== 生命周期与 trait 对象 ===");

    trait Greet {
        fn greet(&self) -> String;
    }

    struct Person {
        name: String,
    }
    impl Greet for Person {
        fn greet(&self) -> String {
            format!("Hi, I'm {}", self.name)
        }
    }

    // dyn Trait + 'static：trait 对象不借用外部数据
    fn make_greeter_static() -> Box<dyn Greet> {
        Box::new(Person { name: "Alice".into() })
    }

    // dyn Trait + 'a：trait 对象借用外部数据
    fn make_greeter_ref<'a>(name: &'a str) -> Box<dyn Greet + 'a> {
        struct NameRef<'a>(&'a str);
        impl<'a> Greet for NameRef<'a> {
            fn greet(&self) -> String {
                format!("Hi, I'm {}", self.0)
            }
        }
        Box::new(NameRef(name))
    }

    let g1 = make_greeter_static();
    println!("static greeter: {}", g1.greet());

    let name = String::from("Bob");
    let g2 = make_greeter_ref(&name);
    println!("ref greeter: {}", g2.greet());

    // 默认情况下，dyn Trait 隐含 'static 约束
    // 如果需要借用数据，必须显式标注 'a
    println!();
    println!("dyn Trait 生命周期：");
    println!("  Box<dyn Trait>       ≡ Box<dyn Trait + 'static>");
    println!("  Box<dyn Trait + 'a>  借用生命周期 'a 的数据");
    println!("  &dyn Trait + 'a      引用借用 'a 数据的 trait 对象");
}

/// 生命周期与闭包
pub fn lifetime_closures() {
    println!("\n=== 生命周期与闭包 ===");

    // 闭包捕获引用时，引用的生命周期会影响闭包的类型
    let data = vec![1, 2, 3];

    // 这个闭包借用 data，生命周期与 data 绑定
    let closure = || println!("data: {:?}", data);
    closure();
    // data 仍然可用，因为闭包只是借用
    println!("data still available: {:?}", data);

    // move 闭包获取所有权，不再受生命周期限制
    let data2 = vec![4, 5, 6];
    let closure2 = move || println!("data2: {:?}", data2);
    closure2();
    // println!("{:?}", data2); // 编译错误：data2 已被 move

    // 返回闭包时的生命周期处理
    // 如果闭包捕获了引用，返回的闭包必须与引用生命周期绑定
    fn make_printer<'a>(data: &'a Vec<i32>) -> impl Fn() + 'a {
        move || println!("printer: {:?}", data)
    }
    let v = vec![7, 8, 9];
    let p = make_printer(&v);
    p();
    println!("v still available: {:?}", v);

    // 如果闭包需要 'static（如 thread::spawn），必须 move 所有权
    let data3 = vec![10, 11, 12];
    let handle = std::thread::spawn(move || {
        println!("thread data: {:?}", data3);
    });
    handle.join().unwrap();
    // println!("{:?}", data3); // 编译错误：data3 已被 move

    println!();
    println!("闭包生命周期要点：");
    println!("  - 借用捕获：闭包生命周期 ≤ 被借用变量的生命周期");
    println!("  - move 捕获：闭包获得所有权，不再受生命周期限制");
    println!("  - 返回闭包：借用捕获需标注生命周期，move 捕获可用 impl Fn");
}

/// 匿名生命周期 '_
pub fn anonymous_lifetime() {
    println!("\n=== 匿名生命周期 '_ ===");

    // '_ 是生命周期省略的显式写法，告诉编译器"这里有一个生命周期，你来推断"
    // 常见场景：结构体方法返回引用时

    struct Config {
        name: String,
    }
    impl Config {
        // 返回引用时，编译器自动推断生命周期
        // fn name(&self) -> &str 等价于：
        fn name(&self) -> &str {
            &self.name
        }

        // 如果需要显式标注，可以用 '_
        fn name_explicit(&self) -> &'_ str {
            &self.name
        }
    }

    let c = Config { name: "test".into() };
    println!("name: {}", c.name());
    println!("name_explicit: {}", c.name_explicit());

    // '_ 在闭包类型标注中也很有用
    let s = String::from("hello");
    let closure: Box<dyn Fn() -> String> = Box::new(move || s.clone());
    println!("closure result: {}", closure());

    // '_ 不能用于函数签名中的独立引用参数
    // fn bad(_: &'_ str) {} // 编译错误：函数参数必须有明确的生命周期名或被省略
    // fn ok(_: &str) {}     // 正确：省略规则自动推断

    println!();
    println!("'_ 使用场景：");
    println!("  - impl 块中方法返回引用时的显式标注");
    println!("  - let 绑定中闭包/迭代器的生命周期标注");
    println!("  - 不能用于函数签名的独立参数");
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
    lifetime_elision();
    lifetime_static();
    lifetime_variance();
    lifetime_trait_objects();
    lifetime_closures();
    anonymous_lifetime();
    ownership_patterns();
}
