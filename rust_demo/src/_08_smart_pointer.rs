#![allow(dead_code)]
//! # 08 - 智能指针
//!
//! 智能指针拥有数据的所有权或引用计数，自动管理内存。
//! Rust 的智能指针通过所有权系统保证内存安全。
//!
//! C++ 对比：
//! - Box ≈ unique_ptr（独占所有权）
//! - Rc ≈ shared_ptr（引用计数）
//! - Arc ≈ shared_ptr + atomic（线程安全）
//! - Cell/RefCell ≈ 内部可变性模式

/// Box<T>：堆分配，独占所有权
pub fn box_demo() {
    println!("=== Box 堆分配 ===");

    // Box 在堆上分配数据，栈上只存指针
    let boxed = Box::new(42);
    println!("boxed: {}", boxed);
    println!("size on stack: {} bytes", std::mem::size_of_val(&boxed));

    // 递归类型必须使用 Box（编译时需要知道大小）
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("list: {:?}", list);

    // Box 实现了 Deref，可以像引用一样使用
    let boxed_vec = Box::new(vec![1, 2, 3]);
    println!("boxed vec length: {}", boxed_vec.len()); // 自动解引用

    // Box 实现了 Drop，离开作用域时自动释放
    {
        let _temp = Box::new(String::from("will be dropped"));
        println!("  box created");
    } // _temp 在这里被 drop
    println!("  box dropped");
}

/// Rc<T>：引用计数，单线程共享所有权
pub fn rc_demo() {
    println!("\n=== Rc 引用计数 ===");

    use std::rc::Rc;

    // Rc 允许多个所有者共享同一数据
    let shared = Rc::new(String::from("shared data"));
    println!("reference count: {}", Rc::strong_count(&shared));

    // clone 增加引用计数（不深拷贝数据）
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);
    println!("reference count after clones: {}", Rc::strong_count(&shared));

    // 所有克隆指向同一数据
    println!("shared: {}", shared);
    println!("clone1: {}", clone1);
    println!("clone2: {}", clone2);

    // 引用计数减少
    drop(clone1);
    println!("reference count after drop: {}", Rc::strong_count(&shared));

    // 共享数据的图结构
    #[derive(Debug)]
    struct Node {
        value: i32,
        neighbors: Vec<Rc<Node>>,
    }

    let node1 = Rc::new(Node { value: 1, neighbors: vec![] });
    let node2 = Rc::new(Node { value: 2, neighbors: vec![Rc::clone(&node1)] });
    println!("node2 neighbors: {:?}", node2.neighbors);
}

/// Cell 和 RefCell：内部可变性
pub fn cell_refcell_demo() {
    println!("\n=== Cell/RefCell 内部可变性 ===");

    use std::cell::{Cell, RefCell};

    // Cell：适用于 Copy 类型，运行时借用检查
    let cell = Cell::new(42);
    cell.set(100);
    println!("cell value: {}", cell.get());

    // RefCell：适用于非 Copy 类型，运行时借用检查
    let refcell = RefCell::new(vec![1, 2, 3]);

    // borrow() 获取不可变引用
    {
        let borrowed = refcell.borrow();
        println!("borrowed: {:?}", borrowed);
    } // borrowed 在这里离开作用域

    // borrow_mut() 获取可变引用
    {
        let mut borrowed_mut = refcell.borrow_mut();
        borrowed_mut.push(4);
        println!("borrowed_mut: {:?}", borrowed_mut);
    }

    // 运行时借用检查：如果同时存在可变和不可变引用，会 panic
    // let r1 = refcell.borrow();
    // let r2 = refcell.borrow_mut(); // panic!

    // Rc + RefCell 组合：共享可变数据
    use std::rc::Rc;
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));

    let clone1 = Rc::clone(&shared_data);
    let clone2 = Rc::clone(&shared_data);

    clone1.borrow_mut().push(4);
    clone2.borrow_mut().push(5);

    println!("shared data: {:?}", shared_data.borrow());
}

/// Cow<T>：写时克隆
pub fn cow_demo() {
    println!("\n=== Cow 写时克隆 ===");

    use std::borrow::Cow;

    // Cow 在需要修改时才克隆数据，否则借用原始数据
    fn process(input: &str) -> Cow<'_, str> {
        if input.contains("bad") {
            // 需要修改，克隆并替换
            Cow::Owned(input.replace("bad", "good"))
        } else {
            // 不需要修改，直接借用
            Cow::Borrowed(input)
        }
    }

    let good = process("hello world");
    let fixed = process("bad word");

    println!("good: {} (borrowed: {})", good, matches!(good, Cow::Borrowed(_)));
    println!("fixed: {} (borrowed: {})", fixed, matches!(fixed, Cow::Borrowed(_)));

    // Cow 用于避免不必要的 String 分配
    fn greet(name: &str) -> Cow<'_, str> {
        if name == "World" {
            Cow::Borrowed("Hello, World!")
        } else {
            Cow::Owned(format!("Hello, {}!", name))
        }
    }

    let msg1 = greet("World");
    let msg2 = greet("Alice");
    println!("{}", msg1);
    println!("{}", msg2);
}

/// Weak<T>：弱引用，打破循环引用
pub fn weak_demo() {
    println!("\n=== Weak 弱引用 ===");

    use std::rc::{Rc, Weak};
    use std::cell::RefCell;

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // 设置 leaf 的 parent 为 branch 的弱引用
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent: {:?}", leaf.parent.borrow().upgrade());
    println!("branch strong count: {}", Rc::strong_count(&branch));
    println!("branch weak count: {}", Rc::weak_count(&branch));
}

/// Drop trait：自定义析构
pub fn drop_demo() {
    println!("\n=== Drop 自定义析构 ===");

    struct DatabaseConnection {
        name: String,
    }

    impl DatabaseConnection {
        fn new(name: &str) -> Self {
            println!("  opening connection: {}", name);
            DatabaseConnection { name: name.to_string() }
        }

        fn query(&self, sql: &str) {
            println!("  {} executing: {}", self.name, sql);
        }
    }

    // Drop trait：离开作用域时自动调用
    impl Drop for DatabaseConnection {
        fn drop(&mut self) {
            println!("  closing connection: {}", self.name);
        }
    }

    {
        let conn = DatabaseConnection::new("db1");
        conn.query("SELECT * FROM users");
        println!("  using connection...");
    } // conn 在这里被 drop

    println!("  connection closed automatically");

    // std::mem::drop 显式释放
    let conn2 = DatabaseConnection::new("db2");
    conn2.query("INSERT INTO logs ...");
    drop(conn2); // 显式释放
    println!("  conn2 dropped early");
}

/// 运行所有智能指针示例
pub fn run() {
    box_demo();
    rc_demo();
    cell_refcell_demo();
    cow_demo();
    weak_demo();
    drop_demo();
}
