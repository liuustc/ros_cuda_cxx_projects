/// # 常用标准库组件示例
///
/// 涵盖 Rust 标准库中最常用的集合类型、迭代器、错误处理等。
use std::collections::{HashMap, BTreeMap, HashSet, VecDeque, BinaryHeap};

/// Vec：动态数组，最常用的集合类型
pub fn vec_demo() {
    println!("=== Vec 动态数组 ===");

    // 创建方式
    let mut v1: Vec<i32> = Vec::new();           // 空 vec
    let v2 = vec![1, 2, 3];                       // 宏创建
    let _v3: Vec<i32> = Vec::with_capacity(100);    // 预分配容量

    // 添加元素
    v1.push(10);
    v1.push(20);
    v1.push(30);
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);

    // 访问元素
    let third = v2[2];                            // 直接索引（越界会 panic）
    let safe = v2.get(99);                        // 安全访问（返回 Option）
    println!("third: {}, safe: {:?}", third, safe);

    // 遍历
    for val in &v2 {
        print!("{} ", val);
    }
    println!();

    // 可变遍历
    let mut v4 = vec![1, 2, 3];
    for val in &mut v4 {
        *val *= 2;
    }
    println!("doubled: {:?}", v4);

    // 用枚举存储不同类型
    #[derive(Debug)]
    #[allow(dead_code)]
    enum Cell { Int(i32), Float(f64), Text(String) }
    let row = vec![Cell::Int(1), Cell::Float(std::f64::consts::PI), Cell::Text(String::from("hello"))];
    println!("row: {:?}", row);

    // 常用方法
    let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    v.sort();                                     // 排序
    v.dedup();                                    // 去重（需先排序）
    println!("sorted & deduped: {:?}", v);
    println!("len: {}, capacity: {}", v.len(), v.capacity());
    println!("sum: {}, first: {:?}", v.iter().sum::<i32>(), v.first());
}

/// String 与 &str：字符串类型
pub fn string_demo() {
    println!("\n=== String 字符串 ===");

    // 创建方式
    let _s1 = String::new();                      // 空字符串（下划线前缀表示允许未使用）
    let s2 = String::from("hello");               // 从字面量创建
    let s3 = "world".to_string();                 // 另一种写法
    let s4 = format!("{} {}", s2, s3);            // 格式化

    println!("s4: {}", s4);

    // 字符串拼接
    let mut s = String::from("hello");
    s.push(' ');                                  // 追加字符
    s.push_str("world");                          // 追加字符串
    s += "!";                                     // 运算符重载
    println!("s: {}", s);

    // 注意：String 是 UTF-8 编码，不能按索引直接访问
    let hello = "你好";
    println!("len: {} bytes, {} chars", hello.len(), hello.chars().count());

    // 遍历字符
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();

    // 字符串切片（注意边界必须在字符边界上）
    let s = String::from("hello world");
    let hello = &s[0..5];
    println!("slice: {}", hello);

    // 常用方法
    let s = "  Hello, World!  ";
    println!("trim: '{}'", s.trim());
    println!("contains 'World': {}", s.contains("World"));
    println!("replace: {}", s.replace("World", "Rust"));
    println!("split: {:?}", "a,b,c".split(',').collect::<Vec<_>>());
}

/// HashMap：哈希映射
pub fn hashmap_demo() {
    println!("\n=== HashMap ===");

    // 创建和插入
    let mut scores = HashMap::new();
    scores.insert("Alice", 100);
    scores.insert("Bob", 85);
    scores.insert("Charlie", 92);

    // 访问
    let alice_score = scores.get("Alice");
    println!("Alice: {:?}", alice_score);

    // 遍历
    for (name, score) in &scores {
        println!("  {}: {}", name, score);
    }

    // 更新：不存在时才插入
    scores.entry("David").or_insert(88);

    // 更新：基于旧值计算
    let text = "hello world hello rust hello";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("word count: {:?}", word_count);

    // 从迭代器创建
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let people: HashMap<_, _> = names.into_iter().zip(ages).collect();
    println!("people: {:?}", people);
}

/// BTreeMap：有序映射（基于红黑树）
pub fn btreemap_demo() {
    println!("\n=== BTreeMap 有序映射 ===");

    let mut map = BTreeMap::new();
    map.insert(3, "three");
    map.insert(1, "one");
    map.insert(4, "four");
    map.insert(1, "ONE"); // 覆盖旧值

    // 按 key 有序遍历
    for (k, v) in &map {
        println!("  {}: {}", k, v);
    }

    // 范围查询
    let range: Vec<_> = map.range(2..=4).collect();
    println!("range 2..=4: {:?}", range);
}

/// HashSet：哈希集合
pub fn hashset_demo() {
    println!("\n=== HashSet ===");

    let mut fruits = HashSet::new();
    fruits.insert("apple");
    fruits.insert("banana");
    fruits.insert("cherry");
    fruits.insert("apple"); // 重复插入无效

    println!("fruits: {:?}", fruits);
    println!("contains 'banana': {}", fruits.contains("banana"));

    // 集合运算
    let set_a: HashSet<i32> = vec![1, 2, 3, 4].into_iter().collect();
    let set_b: HashSet<i32> = vec![3, 4, 5, 6].into_iter().collect();

    let union: Vec<_> = set_a.union(&set_b).collect();
    let intersection: Vec<_> = set_a.intersection(&set_b).collect();
    let difference: Vec<_> = set_a.difference(&set_b).collect();

    println!("union: {:?}", union);
    println!("intersection: {:?}", intersection);
    println!("difference: {:?}", difference);
}

/// VecDeque：双端队列
pub fn vecdeque_demo() {
    println!("\n=== VecDeque 双端队列 ===");

    let mut deque = VecDeque::new();
    deque.push_back(1);    // 尾部插入
    deque.push_back(2);
    deque.push_front(0);   // 头部插入

    println!("deque: {:?}", deque);
    println!("pop_front: {:?}", deque.pop_front());
    println!("pop_back: {:?}", deque.pop_back());
    println!("remaining: {:?}", deque);
}

/// BinaryHeap：优先队列（最大堆）
pub fn binaryheap_demo() {
    println!("\n=== BinaryHeap 优先队列 ===");

    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    // 按优先级弹出（最大值优先）
    print!("sorted: ");
    while let Some(val) = heap.pop() {
        print!("{} ", val);
    }
    println!();

    // 最小堆技巧：取反
    let mut min_heap = BinaryHeap::new();
    min_heap.push(std::cmp::Reverse(3));
    min_heap.push(std::cmp::Reverse(1));
    min_heap.push(std::cmp::Reverse(4));
    println!("min: {:?}", min_heap.pop().unwrap().0);
}

/// 迭代器适配器
pub fn iterator_demo() {
    println!("\n=== 迭代器 ===");

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

    // chain：连接
    let a = [1, 2];
    let b = [3, 4];
    let chained: Vec<&i32> = a.iter().chain(b.iter()).collect();
    println!("chained: {:?}", chained);

    // enumerate：带索引
    for (i, val) in numbers.iter().enumerate().take(3) {
        println!("  [{}] = {}", i, val);
    }

    // zip：并行迭代
    let names = ["Alice", "Bob", "Charlie"];
    let scores = [100, 85, 92];
    let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();
    println!("pairs: {:?}", pairs);

    // flat_map：展平
    let nested = [vec![1, 2], vec![3, 4], vec![5]];
    let flat: Vec<&i32> = nested.iter().flat_map(|v| v.iter()).collect();
    println!("flat: {:?}", flat);

    // take_while / skip_while
    let data = [1, 2, 3, 10, 4, 5];
    let taken: Vec<&i32> = data.iter().take_while(|&&x| x < 5).collect();
    println!("take_while < 5: {:?}", taken);

    // any / all
    println!("any > 9: {}", numbers.iter().any(|&x| x > 9));
    println!("all > 0: {}", numbers.iter().all(|&x| x > 0));

    // min / max / sum / product
    println!("min: {:?}, max: {:?}", numbers.iter().min(), numbers.iter().max());
    println!("sum: {}, product: {}", numbers.iter().sum::<i32>(), numbers.iter().product::<i32>());
}

/// Option 与 Result：错误处理
pub fn error_handling() {
    println!("\n=== Option 与 Result ===");

    // Option：表示可能不存在的值
    fn find_even(n: i32) -> Option<i32> {
        if n % 2 == 0 { Some(n) } else { None }
    }

    // Option 的组合子
    let result = find_even(4)
        .map(|x| x * 2)                    // map：变换内部值
        .unwrap_or(0);                      // unwrap_or：提供默认值
    println!("find_even(4).map(x*2): {}", result);

    // and_then：链式操作
    let result = find_even(4)
        .and_then(|x| if x > 0 { Some(x) } else { None })
        .unwrap_or(0);
    println!("and_then: {}", result);

    // Result：表示可能失败的操作
    fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse::<i32>()
    }

    // Result 的处理
    match parse_number("42") {
        Ok(n) => println!("parsed: {}", n),
        Err(e) => println!("error: {}", e),
    }

    // ? 运算符：提前返回错误
    fn process(s: &str) -> Result<i32, Box<dyn std::error::Error>> {
        let n: i32 = s.parse()?;
        Ok(n * 2)
    }
    println!("process('21'): {:?}", process("21"));
    println!("process('abc'): {:?}", process("abc"));

    // unwrap_or_else：自定义错误处理
    let value = parse_number("abc").unwrap_or_else(|e| {
        eprintln!("  parse error: {}, using default", e);
        0
    });
    println!("value: {}", value);
}

/// 自定义迭代器
pub fn custom_iterator() {
    println!("\n=== 自定义迭代器 ===");

    // 斐波那契数列迭代器
    struct Fibonacci {
        a: u64,
        b: u64,
    }

    impl Fibonacci {
        fn new() -> Self {
            Fibonacci { a: 0, b: 1 }
        }
    }

    impl Iterator for Fibonacci {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            let result = self.a;
            let new_b = self.a + self.b;
            self.a = self.b;
            self.b = new_b;
            Some(result)
        }
    }

    let fib: Vec<u64> = Fibonacci::new().take(10).collect();
    println!("fibonacci: {:?}", fib);

    // 使用迭代器适配器
    // 取前 20 个斐波那契数，从中筛出偶数求和
    let fib_even_sum: u64 = Fibonacci::new()
        .take(20)
        .filter(|x| x % 2 == 0)
        .sum();
    println!("sum of evens among first 20 Fib numbers: {}", fib_even_sum);
}

/// 运行所有标准库示例
pub fn run() {
    vec_demo();
    string_demo();
    hashmap_demo();
    btreemap_demo();
    hashset_demo();
    vecdeque_demo();
    binaryheap_demo();
    iterator_demo();
    error_handling();
    custom_iterator();
}
