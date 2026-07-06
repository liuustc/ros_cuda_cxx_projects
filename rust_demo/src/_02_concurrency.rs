//! # 多线程与共享数据示例
//!
//! Rust 的并发模型基于"无畏并发"（fearless concurrency）：
//! 编译器在编译期防止数据竞争，让你可以安全地使用多线程。
//!
//! 核心概念：
//! - `std::thread::spawn` 创建线程
//! - `Arc`（原子引用计数）跨线程共享所有权
//! - `Mutex`（互斥锁）保护可变数据
//! - `channel` 线程间消息传递

use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// 基础线程创建
pub fn basic_threads() {
    println!("=== 基础线程 ===");

    // spawn 创建新线程，返回 JoinHandle
    let handle = thread::spawn(|| {
        for i in 1..=3 {
            println!("  spawned thread: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    // 主线程继续执行
    for i in 1..=3 {
        println!("  main thread: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // join 等待线程完成
    handle.join().unwrap();
    println!("all threads done");
}

/// move 闭包：将所有权转移到线程中
pub fn move_closure() {
    println!("\n=== move 闭包 ===");

    let data = vec![1, 2, 3];

    // move 关键字将 data 的所有权转移到新线程
    let handle = thread::spawn(move || {
        println!("data in thread: {:?}", data);
    });

    // println!("{:?}", data); // 编译错误：data 已被 move
    handle.join().unwrap();
}

/// Arc + Mutex：跨线程共享可变数据
pub fn arc_mutex() {
    println!("\n=== Arc + Mutex 共享数据 ===");

    // Arc：原子引用计数，允许多个所有者
    // Mutex：互斥锁，保护内部数据的可变访问
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter); // 克隆 Arc（增加引用计数）
        let handle = thread::spawn(move || {
            // lock() 获取锁，返回 MutexGuard
            // MutexGuard 在离开作用域时自动释放锁（RAII）
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("  thread {} incremented to {}", i, *num);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("final counter: {}", *counter.lock().unwrap());
}

/// channel：线程间消息传递
pub fn channel_demo() {
    println!("\n=== channel 消息传递 ===");

    // mpsc：multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    // 生产者线程
    thread::spawn(move || {
        let messages = vec!["hello", "from", "sender"];
        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
        // tx 在离开作用域时自动关闭通道
    });

    // 消费者：迭代接收消息
    for received in rx {
        println!("  received: {}", received);
    }
    println!("channel closed");
}

/// 多生产者 channel
pub fn multi_producer() {
    println!("\n=== 多生产者 channel ===");

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // 创建3个生产者
    for id in 0..3 {
        let tx = tx.clone(); // 克隆发送端
        let handle = thread::spawn(move || {
            for i in 0..2 {
                let msg = format!("producer-{}: msg-{}", id, i);
                tx.send(msg).unwrap();
                thread::sleep(Duration::from_millis(20));
            }
        });
        handles.push(handle);
    }

    // 丢弃原始发送端（所有克隆的发送端也需要丢弃，通道才会关闭）
    drop(tx);

    // 接收所有消息
    for received in rx {
        println!("  {}", received);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// RwLock：读写锁，允许多个读者或一个写者
pub fn rwlock_demo() {
    println!("\n=== RwLock 读写锁 ===");

    use std::sync::RwLock;

    let lock = RwLock::new(vec![1, 2, 3]);
    let mut handles = vec![];

    // 多个读者可以同时读取
    for i in 0..3 {
        let lock = lock.read().unwrap();
        println!("  reader {}: {:?}", i, *lock);
        // 读锁在作用域结束时释放
    }

    // 写者独占访问
    {
        let mut write = lock.write().unwrap();
        write.push(4);
        println!("  writer added 4: {:?}", *write);
    }

    // 多线程读写
    let lock = Arc::new(RwLock::new(0));
    for i in 0..3 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            // 读取
            let val = *lock.read().unwrap();
            println!("  thread {} read: {}", i, val);
            // 写入
            let mut val = lock.write().unwrap();
            *val += 1;
            println!("  thread {} wrote: {}", i, *val);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("final value: {}", *lock.read().unwrap());
}

/// Barrier：线程同步屏障，所有线程到达后才继续
pub fn barrier_demo() {
    println!("\n=== Barrier 同步屏障 ===");

    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            println!("  thread {} before barrier", i);
            // 等待所有线程到达
            barrier.wait();
            println!("  thread {} after barrier", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Condvar：条件变量，线程间通知
pub fn condvar_demo() {
    println!("\n=== Condvar 条件变量 ===");

    use std::sync::Condvar;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = Arc::clone(&pair);

    // 生产者线程
    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let (lock, cvar) = &*pair_clone;
        let mut ready = lock.lock().unwrap();
        *ready = true;
        println!("  producer: data ready");
        cvar.notify_one(); // 通知等待的线程
    });

    // 消费者线程
    let (lock, cvar) = &*pair;
    let mut ready = lock.lock().unwrap();
    while !*ready {
        // wait 会释放锁并阻塞，被唤醒后重新获取锁
        ready = cvar.wait(ready).unwrap();
    }
    println!("  consumer: got notification, ready = {}", *ready);
}

/// 线程池模式：使用 channel 实现简单的线程池
pub fn thread_pool_demo() {
    println!("\n=== 简单线程池 ===");

    let (tx, rx) = mpsc::channel::<Box<dyn FnOnce() + Send>>();
    let rx = Arc::new(Mutex::new(rx));

    // 创建工作线程
    let mut handles = vec![];
    for id in 0..3 {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                // 从通道接收任务
                // 注意：recv() 阻塞期间持有 Mutex 锁，
                // 其他工作线程会被挡在 lock() 处 —— 这是演示用简化实现，
                // 高效的线程池通常用条件变量 + 无锁队列等方案
                let task = rx.lock().unwrap().recv();
                match task {
                    Ok(task) => {
                        println!("  worker {} executing task", id);
                        task();
                    }
                    Err(_) => {
                        println!("  worker {} shutting down", id);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }

    // 提交任务
    for i in 0..6 {
        let task = Box::new(move || {
            println!("    task {} done", i);
        });
        tx.send(task).unwrap();
    }

    // 关闭通道，通知工作线程退出
    drop(tx);

    for handle in handles {
        handle.join().unwrap();
    }
    println!("thread pool shutdown");
}

/// Send 和 Sync：线程安全标记 trait
///
/// 这两个 auto trait 是 Rust 并发安全的基础：
/// - Send：类型可以安全地将所有权转移到另一个线程
/// - Sync：类型的引用可以安全地在多个线程间共享
///
/// 编译器会自动推导它们，大多数类型都是 Send + Sync。
/// 手动实现 unsafe Send/Sync 需要小心（见 _17_unsafe 模块）。
pub fn send_sync_demo() {
    println!("\n=== Send 与 Sync trait ===");

    // Send：可以跨线程转移所有权
    // 几乎所有基础类型都是 Send（i32, String, Vec<T: Send>, Box<T: Send>...）
    // Rc<T> 不是 Send（非原子引用计数，不能跨线程）
    // Arc<T: Send> 才是 Send（原子引用计数）

    println!("Send trait: 所有权可以安全地转移到另一个线程");
    println!("  Send 类型: i32, String, Vec<T: Send>, Arc<T: Send>");
    println!("  !Send 类型: Rc<T>, *const T, *mut T, Cell<T>");
    println!();

    // Sync：可以跨线程共享引用
    // T: Sync 当且仅当 &T: Send
    // Mutex<T: Send>: Sync —— 这就是为什么 Arc<Mutex<T>> 能安全跨线程共享

    println!("Sync trait: 引用可以安全地在多个线程间共享");
    println!("  Sync 类型: i32, &str, Mutex<T: Send>, RwLock<T: Send>");
    println!("  !Sync 类型: Rc<T>, Cell<T>, RefCell<T>");
    println!();

    // 利用 Send 约束在编译期防止错误
    // 下面的代码如果去掉注释，编译器会报错：
    // use std::rc::Rc;
    // let non_send = Rc::new(42);
    // thread::spawn(move || { println!("{}", non_send); });
    //     // 编译错误：Rc<i32> 不是 Send

    println!("编译器自动检查 Send/Sync，在编译期防止数据竞争。");
    println!("标记 trait 总结：");
    println!("  Send = 所有权可跨线程转移");
    println!("  Sync = 引用可跨线程共享（等价于 &T: Send）");
    println!("  Auto trait = 编译器自动推导");
    println!("  unsafe impl = 需要手动担保安全性");
}

/// 运行所有并发示例
pub fn run() {
    basic_threads();
    move_closure();
    arc_mutex();
    channel_demo();
    multi_producer();
    rwlock_demo();
    barrier_demo();
    condvar_demo();
    thread_pool_demo();
    send_sync_demo();
}
