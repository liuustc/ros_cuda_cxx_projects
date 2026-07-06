#![allow(dead_code)]
//! # 17 - Unsafe Rust
//!
//! Rust 的 unsafe 不是"关闭所有安全检查"，而是在编译器无法验证时，
//! 把安全责任交给程序员。unsafe 代码块中仍然享有大部分安全检查。
//!
//! unsafe 的五大超级能力：
//! 1. 解引用裸指针（dereference raw pointers）
//! 2. 调用 unsafe 函数或方法
//! 3. 访问或修改可变静态变量
//! 4. 实现 unsafe trait
//! 5. 访问 union 的字段
//!
//! 关键原则：unsafe 代码应封装在安全抽象之后。

/// 裸指针（Raw Pointer）：*const T 和 *mut T
///
/// 与引用(&T)的区别：
/// - 裸指针可以忽略借用规则（同时存在多个可变指针）
/// - 裸指针不保证指向有效内存
/// - 裸指针可以为 null
/// - 裸指针没有自动清理
pub fn raw_pointers() {
    println!("=== 裸指针 ===");

    let mut value = 42;

    // 从引用创建裸指针（安全操作）
    let r1: *const i32 = &value;
    let r2: *mut i32 = &mut value;

    println!("裸指针地址: r1={:?}, r2={:?}", r1, r2);

    // 解引用裸指针（必须放在 unsafe 块中）
    unsafe {
        println!("  *r1 = {}", *r1);
        *r2 = 100; // 通过裸指针修改值
        println!("  after *r2 = 100: value = {}", value);
    }

    // 裸指针可以为 null（引用不行）
    let null_ptr: *const i32 = std::ptr::null();
    println!("null ptr: {:?}", null_ptr);
    // unsafe { println!("{}", *null_ptr); } // 未定义行为！段错误！

    // 从整数创建裸指针（极不安全，仅用于 FFI 等场景）
    let arbitrary: *const u8 = 0x42 as *const u8;
    println!("arbitrary address: {:?}", arbitrary);

    println!();
    println!("裸指针使用场景：");
    println!("  1. FFI：调用 C 函数时传递指针");
    println!("  2. 底层内存操作");
    println!("  3. 实现安全抽象的内部实现");
    println!("安全原则：安全 Rust 代码中不出现裸指针");
}

/// 调用 unsafe 函数
#[allow(unnecessary_transmutes)]
pub fn unsafe_functions() {
    println!("\n=== Unsafe 函数 ===");

    // 标记为 unsafe 的函数，必须在 unsafe 块中调用
    unsafe fn dangerous() {
        println!("  this function requires careful memory management");
    }

    unsafe {
        dangerous();
    }

    // 常见场景：让用户自己保证不重叠
    // std::ptr::copy_nonoverlapping, std::ptr::swap_nonoverlapping 等

    // 创建指向值的裸指针并读取
    let data = [1u8, 2, 3, 4];
    let ptr = data.as_ptr();
    unsafe {
        // 读取指针指向的值（需要保证指针有效 + 对齐正确）
        let first = std::ptr::read(ptr);
        println!("  ptr::read first: {}", first);
    }

    // std::mem::transmute：重新解释位模式（极度危险，仅用于演示）
    // 实际项目中，safe 的 f32::from_ne_bytes 应优先使用
    let bytes: [u8; 4] = [0x00, 0x00, 0x80, 0x3F]; // 1.0f32 的 IEEE 754 表示
    let float_val: f32 = unsafe { std::mem::transmute::<[u8; 4], f32>(bytes) };
    println!("  transmute([00,00,80,3F]) = {:.1}", float_val);
    println!("  (1.0f32 的 IEEE 754 字节表示)");
}

/// 创建安全的抽象封装
///
/// unsafe 代码的核心原则：用安全 API 封装 unsafe 实现。
/// 调用者无需写 unsafe 块即可安全使用。
pub fn safe_abstractions() {
    println!("\n=== 安全抽象封装 ===");

    // 示例：实现一个安全的分割切片函数
    // 标准库的 split_at_mut 就是这样实现的
    fn split_slice_at_mut<T>(slice: &mut [T], mid: usize) -> (&mut [T], &mut [T]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len, "mid must be <= len");

        // 安全 Rust 无法同时返回两个不重叠的可变引用
        // 但在 unsafe 中我们可以手动保证这一点
        unsafe {
            (
                std::slice::from_raw_parts_mut(ptr, mid),
                std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut arr = [1, 2, 3, 4, 5, 6];
    let (left, right) = split_slice_at_mut(&mut arr, 3);
    println!("left: {:?}, right: {:?}", left, right);

    left[0] = 100;
    right[0] = 200;
    println!("after modify: {:?}", arr);

    println!();
    println!("安全抽象模式：");
    println!("  1. unsafe 实现 → 安全 API 暴露");
    println!("  2. 在安全 API 中验证所有前提条件");
    println!("  3. 调用者无需接触 unsafe");
    println!("  标准库中大量使用此模式：Vec, String, RefCell, Mutex...");
}

/// 可变静态变量
pub fn static_mutables() {
    println!("\n=== 可变静态变量 ===");

    // 静态变量（不可变）是安全的
    static GREETING: &str = "Hello";
    println!("static GREETING: {}", GREETING);

    // 可变静态变量：读取和写入都需要 unsafe
    // （多线程环境中有数据竞争风险）
    // static mut COUNTER: u32 = 0;

    // unsafe {
    //     COUNTER += 1;
    //     println!("COUNTER: {}", COUNTER);
    // }

    println!("static mut 需要 unsafe 访问 → 避免数据竞争");
    println!("推荐用 std::sync::atomic 替代 static mut");

    // 更好的做法：原子类型
    use std::sync::atomic::{AtomicU32, Ordering};
    static SAFE_COUNTER: AtomicU32 = AtomicU32::new(0);
    SAFE_COUNTER.fetch_add(1, Ordering::SeqCst);
    println!("SAFE_COUNTER (atomic, no unsafe): {}", SAFE_COUNTER.load(Ordering::SeqCst));
}

/// Unsafe trait 实现
pub fn unsafe_traits() {
    println!("\n=== Unsafe Trait ===");

    // Send 和 Sync 是 unsafe trait
    // 编译器自动为大多数类型推导 Send + Sync
    // 但当类型包含裸指针时，编译器不自动实现 Send/Sync
    // 需要手动 unsafe impl 来担保安全性

    // 演示：包装裸指针的类型 —— 需要手动担保线程安全
    // 注意：unsafe impl 告诉编译器"我已人工验证过安全性"
    // 错误使用会导致数据竞争和未定义行为

    println!("Send/Sync 是 unsafe trait — 需要手动担保线程安全");
    println!("示例：包含 *const T 的结构体不自动实现 Send");

    // 定义一个含裸指针的结构体（移到模块级演示，因为函数内定义限制较多）
    println!();
    println!("  struct RawWrapper {{ ptr: *const i32 }}");
    println!("  // 编译器不会自动为 RawWrapper 实现 Send");
    println!("  unsafe impl Send for RawWrapper {{}}");
    println!("  // 现在 RawWrapper 可以跨线程传递了");
    println!();
    println!("但手动实现 Send 的代码必须保证：");
    println!("  1. 裸指针确实指向有效且独占的数据");
    println!("  2. 或者有外部机制保证线程安全");
    println!();
    println!("实际场景中，更常见的做法是：");
    println!("  - 使用 NonNull<T> 替代 *mut T（保证非空）");
    println!("  - 配合 PhantomData 标记所有权/生命周期关系");
    println!("  - 实现 Send 前先验证所有字段的线程安全性");
}

/// Union（联合体）
pub fn unions_demo() {
    println!("\n=== Union 联合体 ===");

    // Union 类似 C 的 union，所有字段共享同一块内存
    #[repr(C)]
    union IntOrFloat {
        i: i32,
        f: f32,
    }

    let u = IntOrFloat { i: 42 };
    // 读取 union 字段需要 unsafe（因为无法在编译期确定哪个字段有效）
    unsafe {
        println!("u.i = {}", u.i);
        // println!("u.f = {}", u.f); // 未定义行为：读取不活跃字段
    }

    // 带标记的联合体（Tagged Union）—— 安全模式
    enum TaggedIntOrFloat {
        Int(i32),
        Float(f32),
    }

    let tagged = TaggedIntOrFloat::Int(42);
    match tagged {
        TaggedIntOrFloat::Int(i) => println!("tagged int: {}", i),
        TaggedIntOrFloat::Float(f) => println!("tagged float: {}", f),
    }

    println!("推荐：能用 enum 就不要用 union");
    println!("union 主要用于 FFI 和底层位操作");
}

/// UnsafeCell：内部可变性的基石
pub fn unsafe_cell_demo() {
    println!("\n=== UnsafeCell ===");

    // UnsafeCell<T> 是 Cell、RefCell、Mutex 等所有内部可变性类型的底层实现
    // 它告诉编译器："这里可能发生通过共享引用的修改，不要优化掉"

    use std::cell::UnsafeCell;

    struct MyCell<T> {
        value: UnsafeCell<T>,
    }

    impl<T> MyCell<T> {
        fn new(value: T) -> Self {
            MyCell { value: UnsafeCell::new(value) }
        }

        // 安全 API：获取值（要求 T: Copy 保证简单复制）
        fn get(&self) -> T
        where
            T: Copy,
        {
            unsafe { *self.value.get() }
        }

        // 安全 API：设置值
        fn set(&self, value: T) {
            unsafe { *self.value.get() = value; }
        }
    }

    let cell = MyCell::new(42);
    println!("cell.get(): {}", cell.get());
    cell.set(100);
    println!("after set(100): {}", cell.get());

    println!();
    println!("UnsafeCell 是内部可变性的基础：");
    println!("  - Cell<T> = UnsafeCell<T> + Copy 优化");
    println!("  - RefCell<T> = UnsafeCell<T> + 运行时借用检查");
    println!("  - Mutex<T> = UnsafeCell<T> + 操作系统锁");
}

/// FFI 中的 unsafe
pub fn ffi_unsafe() {
    println!("\n=== FFI 中的 Unsafe ===");

    // 所有 extern 函数调用都是 unsafe 的
    // 因为 Rust 无法验证 C 代码的安全性

    // extern "C" {
    //     fn abs(input: i32) -> i32; // C 标准库函数
    // }

    // unsafe {
    //     println!("abs(-3) = {}", abs(-3));
    // }

    // 安全封装模式：
    // 1. 创建安全的包装函数
    // 2. 在包装函数中验证所有参数
    // 3. 在 unsafe 块中调用 C 函数
    // 4. 验证返回值

    fn safe_abs(x: i32) -> i32 {
        if x == i32::MIN {
            // 处理边界情况：i32::MIN 的绝对值无法用 i32 表示
            panic!("abs(i32::MIN) overflow");
        }
        // 安全的封装内部调用 unsafe
        // unsafe { abs(x) }
        x.abs() // 演示：用标准库替代
    }

    println!("safe_abs(-42) = {}", safe_abs(-42));

    println!();
    println!("FFI unsafe 最佳实践：");
    println!("  1. 为每个 C 函数提供安全包装");
    println!("  2. 在包装中验证参数和返回值");
    println!("  3. 用 RAII 管理 C 资源（实现 Drop）");
    println!("  4. 使用 PhantomData 标记所有权关系");
}

/// Unsafe 最佳实践总结
pub fn unsafe_best_practices() {
    println!("\n=== Unsafe 最佳实践 ===");

    println!("原则 1：最小化 unsafe 代码范围");
    println!("  // 好：小块 unsafe");
    println!("  fn safe_fn() {{ ... unsafe {{ one_line(); }} ... }}");
    println!();
    println!("  // 不好：大块 unsafe");
    println!("  unsafe {{ huge_block(); }}");
    println!();

    println!("原则 2：用安全 API 封装 unsafe 实现");
    println!("  pub fn safe_wrapper(x: i32) -> i32 {{");
    println!("      assert!(x >= 0, \"x must be non-negative\");");
    println!("      unsafe {{ internal_unsafe_op(x) }}");
    println!("  }}");
    println!();

    println!("原则 3：用 SAFETY 注释解释为什么 unsafe 是安全的");
    println!("  // SAFETY: ptr 来自刚创建的 Vec，保证非空且对齐正确");
    println!("  unsafe {{ ptr.read() }}");
    println!();

    println!("原则 4：尽可能使用经过验证的模式");
    println!("  - std::ptr::read/write 代替手动解引用");
    println!("  - MaybeUninit 代替未初始化内存");
    println!("  - NonNull 代替裸指针的 null 检查");
    println!();

    println!("原则 5：用 Miri 检测未定义行为");
    println!("  cargo +nightly miri test");
    println!("  Miri 可以在测试中捕获许多 UB 问题");
}

/// 运行所有 unsafe 示例
pub fn run() {
    raw_pointers();
    unsafe_functions();
    safe_abstractions();
    static_mutables();
    unsafe_traits();
    unions_demo();
    unsafe_cell_demo();
    ffi_unsafe();
    unsafe_best_practices();
}
