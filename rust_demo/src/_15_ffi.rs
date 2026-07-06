#![allow(dead_code)]
//! # 15 - FFI（外部函数接口）
//!
//! Rust 可以与 C/C++ 互操作，这是系统编程的重要能力。
//!
//! 方式：
//! 1. extern "C"：手动声明 C 函数
//! 2. bindgen：自动生成绑定
//! 3. cbindgen：从 Rust 生成 C 头文件

/// 调用 C 函数
pub fn call_c_functions() {
    println!("=== 调用 C 函数 ===");

    // 声明 C 函数
    // extern "C" {
    //     fn strlen(s: *const c_char) -> usize;
    //     fn abs(n: c_int) -> c_int;
    // }

    // 使用标准库的 C 绑定
    use std::ffi::CString;
    use std::os::raw::c_char;

    // 创建 C 字符串
    let c_string = CString::new("Hello, C!").expect("CString::new failed");
    let _ptr: *const c_char = c_string.as_ptr();

    // 安全地调用 C 函数
    // unsafe {
    //     // 注意：这里只是演示，实际需要 extern "C" 声明
    //     // let len = strlen(ptr);
    //     println!("C string pointer: {:?}", ptr);
    // }

    println!("C 字符串转换：");
    println!("  CString::new(\"hello\") -> C 字符串");
    println!("  CStr::from_ptr(ptr) -> Rust 引用");
}

/// 导出 Rust 函数给 C 调用
pub fn export_to_c() {
    println!("\n=== 导出给 C ===");

    // 使用 #[no_mangle] 保持函数名
    // 使用 extern "C" 使用 C ABI

    // #[no_mangle]
    // pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    //     a + b
    // }

    // #[no_mangle]
    // pub extern "C" fn rust_process(data: *const u8, len: usize) -> i32 {
    //     let slice = unsafe { std::slice::from_raw_parts(data, len) };
    //     slice.iter().map(|&x| x as i32).sum()
    // }

    println!("导出 Rust 函数：");
    println!("  #[no_mangle]");
    println!("  pub extern \"C\" fn rust_add(a: i32, b: i32) -> i32 {{");
    println!("      a + b");
    println!("  }}");
    println!();
    println!("对应的 C 声明：");
    println!("  int rust_add(int a, int b);");
}

/// 类型映射
pub fn type_mapping() {
    println!("\n=== 类型映射 ===");

    println!("Rust -> C 类型映射：");
    println!("  i8      -> char / int8_t");
    println!("  i16     -> short / int16_t");
    println!("  i32     -> int / int32_t");
    println!("  i64     -> long long / int64_t");
    println!("  u8      -> unsigned char / uint8_t");
    println!("  u16     -> unsigned short / uint16_t");
    println!("  u32     -> unsigned int / uint32_t");
    println!("  u64     -> unsigned long long / uint64_t");
    println!("  f32     -> float");
    println!("  f64     -> double");
    println!("  bool    -> _Bool / int");
    println!("  *const T -> const T*");
    println!("  *mut T  -> T*");
    println!();
    println!("std::os::raw 类型：");
    println!("  c_char, c_int, c_long, c_float, c_double, etc.");
}

/// 结构体传递
pub fn struct_passing() {
    println!("\n=== 结构体传递 ===");

    // C 兼容结构体
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    struct Point {
        x: f64,
        y: f64,
    }

    // #[repr(C)] 保证内存布局与 C 兼容
    let p = Point { x: 1.0, y: 2.0 };
    println!("Point: {:?}", p);
    println!("size: {} bytes", std::mem::size_of::<Point>());

    // 传递给 C
    // extern "C" {
    //     fn process_point(p: Point) -> f64;
    // }

    // 从 C 接收
    // extern "C" {
    //     fn create_point(x: f64, y: f64) -> Point;
    // }

    println!("\n#[repr(C)] 保证：");
    println!("  - 字段顺序不变");
    println!("  - 对齐方式与 C 一致");
    println!("  - 没有填充字节（除非 C 也有）");
}

/// 回调函数
pub fn callbacks() {
    println!("\n=== 回调函数 ===");

    // 定义回调类型
    type Callback = extern "C" fn(i32) -> i32;

    // Rust 实现回调
    extern "C" fn double_callback(x: i32) -> i32 {
        x * 2
    }

    // 传递给 C
    // extern "C" {
    //     fn apply_callback(data: *const i32, len: usize, cb: Callback) -> i32;
    // }

    let result = double_callback(21);
    println!("callback result: {}", result);

    // 闭包作为回调（需要 trampoline）
    // extern "C" fn trampoline<F: Fn(i32) -> i32>(x: i32) -> i32 {
    //     // 需要存储闭包的上下文
    //     unimplemented!()
    // }
}

/// bindgen 使用
pub fn bindgen_usage() {
    println!("\n=== bindgen ===");

    // bindgen 从 C 头文件生成 Rust 绑定
    println!("bindgen 用法：");
    println!("  1. 安装：cargo install bindgen-cli");
    println!("  2. 生成：bindgen wrapper.h -o bindings.rs");
    println!();
    println!("build.rs 集成：");
    println!("  fn main() {{");
    println!("      println!(\"cargo:rerun-if-changed=wrapper.h\");");
    println!("      let bindings = bindgen::Builder::default()");
    println!("          .header(\"wrapper.h\")");
    println!("          .generate()");
    println!("          .expect(\"Unable to generate bindings\");");
    println!("      bindings.write_to_file(\"src/bindings.rs\").unwrap();");
    println!("  }}");
}

/// cbindgen 使用
pub fn cbindgen_usage() {
    println!("\n=== cbindgen ===");

    // cbindgen 从 Rust 生成 C 头文件
    println!("cbindgen 用法：");
    println!("  1. 安装：cargo install cbindgen");
    println!("  2. 生成：cbindgen --crate my_crate --output my_crate.h");
    println!();
    println!("cbindgen.toml 配置：");
    println!("  language = \"C\"");
    println!("  include_guard = \"MY_CRATE_H\"");
    println!("  autogen_warning = \"/* Warning: auto-generated */\"");
}

/// 安全封装
pub fn safe_wrapping() {
    println!("\n=== 安全封装 ===");

    // 为不安全的 C API 创建安全包装
    println!("安全封装模式：");
    println!("  // 不安全的原始绑定");
    println!("  extern \"C\" {{");
    println!("      fn c_process(data: *const u8, len: usize) -> i32;");
    println!("  }}");
    println!();
    println!("  // 安全包装");
    println!("  pub fn process(data: &[u8]) -> Result<i32, Error> {{");
    println!("      if data.is_empty() {{");
    println!("          return Err(Error::EmptyInput);");
    println!("      }}");
    println!("      let result = unsafe {{ c_process(data.as_ptr(), data.len()) }};");
    println!("      Ok(result)");
    println!("  }}");
}

/// 运行示例
pub fn run() {
    call_c_functions();
    export_to_c();
    type_mapping();
    struct_passing();
    callbacks();
    bindgen_usage();
    cbindgen_usage();
    safe_wrapping();
}
