#!/usr/bin/env coffee

fetch "https://annota-api.corust.ai/v1/annotations",
  method: "POST"
  headers:
    "accept": "application/json, text/plain, */*"
    "accept-language": "zh-CN,zh;q=0.9,en;q=0.8"
    "authorization": "Bearer eyJhbGciOiJSUzI1NiIsImtpZCI6Inc5aHR3T3JoNHhqYnFDWmdaQUxDQWpXWUFWUGk2TFJpIn0.eyJpYXQiOjE3NjU0MjI5NDUsImlkIjoiZmZvOVhIZGNNbGNMTEM2cHF0cXpjamJiNFFOaUxTUW8iLCJzdWIiOiJmZm85WEhkY01sY0xMQzZwcXRxemNqYmI0UU5pTFNRbyIsImV4cCI6MTc2NTQyNjU0NSwiaXNzIjoiQW5ub3RhIiwiYXVkIjoiQW5ub3RhIn0.mpsEK0H5pHjn_YVKeXRFXEXzbZQMa0XV4LucDVo2gYxTLRIhrvjGTRl9ARlwXWY_QgRy0egyP6FtGDi9YjiSpp3KOYJWgvnVx0Fo6fo2Y1XrapSnEL3ax0z-w1Qd8PSif5jtbBBTSv6qavDQN0ypHCVo7lZAP4gkbN5XZOPNFs39QA06XsoTJ8T6Nh3j76RkuBwVquZ-VaWI5WSuJ0LSIwguDP1ytcpF3fEIG-5PILfboHk_5Nf8AvCHhVYfUr3frB12xAaKHx7ZaR4uH95ong3H5Cp4ds9zv3E5DvCxcDEppDlq_uSZBTeNrszpKL8gHoOHbKPA7NxgTnTd284FdA"
    "content-type": "application/json"
    "priority": "u=1, i"
    "sec-ch-ua": "\"Chromium\";v=\"142\", \"Google Chrome\";v=\"142\", \"Not_A Brand\";v=\"99\""
    "sec-ch-ua-mobile": "?0"
    "sec-ch-ua-platform": "\"macOS\""
    "sec-fetch-dest": "empty"
    "sec-fetch-mode": "cors"
    "sec-fetch-site": "same-site"
    "Referer": "https://annota.corust.ai/"
  body: JSON.stringify({
    title: "parking_lot send_guard 特性的使用演示和编译错误分析"
    description: "演示 parking_lot 库的 send_guard 特性的正确使用方法，以及在不启用该特性时会遇到的编译错误。这对于理解 Rust 的线程安全机制和 parking_lot 库的高级功能非常重要。主要困惑在于：\n1) send_guard 特性的具体作用和使用场景；\n2) 为什么默认情况下 MutexGuard 不能跨线程传递；\n3) 启用该特性后可以实现哪些高级同步模式。"
    category_id: "0199e8af-c451-7531-8c2b-a93778a3a4ab"
    submitter_name: "i18n"
    submitter_wechat: "i18n-site"
    submitter_email: "i18n.site@gmail.com"
    industry: "Rust 系统编程中的并发同步原语库使用"
    background: "在 Rust 并发编程中，标准库的 std::sync::Mutex 的 MutexGuard 不能在线程间传递，这限制了某些高级同步模式的实现。parking_lot 库提供了一个可选的 send_guard 特性，允许 MutexGuard 实现 Send trait，从而可以在线程间安全传递。然而，如果不启用此特性，尝试在线程间传递 guard 会导致编译错误。"
    problem: """
      // Example project with send_guard feature enabled
      // Cargo.toml:
      [dependencies]
      parking_lot = { version = "0.12", features = ["send_guard"] }

      // Working code:
      use parking_lot::Mutex;
      use std::sync::Arc;
      use std::thread;

      pub fn demonstrate_send_guard() {
          let counter = Arc::new(Mutex::new(0i32));
          let counter_clone = Arc::clone(&counter);

          let handle = thread::spawn(move || {
              let mut guard = counter_clone.lock();
              *guard += 42;
              // Guard can be moved across thread boundaries with send_guard feature
          });

          handle.join().unwrap();
          assert_eq!(*counter.lock(), 42);
      }

      // Error project without send_guard feature
      // This code will fail to compile:
      let (sender, receiver) = std::sync::mpsc::channel();
      let handle1 = thread::spawn(move || {
          let guard = counter_clone.lock();
          sender.send(guard).expect("Failed to send guard"); // ERROR!
      });
    """
    error: """
      error[E0277]: `*mut ()` cannot be sent between threads safely
         --> src/lib.rs:21:33
          |
       21 |       let handle1 = thread::spawn(move || {
          |  ___________________-------------_^
          | |                   |
          | |                   required by a bound introduced by this call
       22 | |         println!("Thread 1: Acquiring lock...");
       23 | |         let guard = counter_clone.lock();
       24 | |         println!("Thread 1: Lock acquired, current value: {}", *guard);
       ...
       32 | |         println!("Thread 1: This line will never be reached due to compilation error");
       33 | |     });
          | |_____^ `*mut ()` cannot be sent between threads safely
          |
          = help: within `MutexGuard<'_, RawMutex, i32>`, the trait `Send` is not implemented for `*mut ()`
          = note: required because it appears within the type `GuardNoSend`
          = note: required because it appears within the type `MutexGuard<'_, RawMutex, i32>`
          = note: required for `Sender<MutexGuard<'_, RawMutex, i32>>` to implement `Send`
    """
    solution: """
      1. 首先理解问题本质：parking_lot 的 MutexGuard 默认不实现 Send trait，这是为了防止潜在的死锁和竞态条件。

      2. 创建两个演示项目：
         - example/ 项目：启用 send_guard 特性，展示正确用法
         - error/ 项目：不启用特性，展示编译错误

      3. 在 example 项目中使用 `cargo add parking_lot --features send_guard` 添加依赖

      4. 在 error 项目中使用 `cargo add parking_lot` 添加不带特性的依赖

      5. 编写演示代码：
         - 基础用法：在线程中获取锁并修改数据
         - 高级模式：展示 send_guard 特性允许的灵活性

      6. 验证结果：
         - example 项目成功编译运行
         - error 项目产生预期的编译错误

      7. 关键发现：
         - send_guard 特性允许 MutexGuard 实现 Send trait
         - 这使得更复杂的同步模式成为可能
         - 但也需要开发者更加小心避免死锁
    """
    code_prompt: """
      Create a Rust demonstration of parking_lot's send_guard feature with two projects:

      1. example/ - Shows correct usage with send_guard feature enabled
         - Add parking_lot with send_guard feature
         - Demonstrate basic mutex usage across threads
         - Show that guards can be moved between threads
      2. error/ - Shows compilation error without send_guard feature
         - Add parking_lot without send_guard feature
         - Attempt to send MutexGuard across threads
         - Document the resulting compilation error

      Requirements:
      - All code and comments in English
      - Include README.md with environment info (Rust version, toolchain, OS)
      - Use cargo add for dependencies (no manual Cargo.toml editing)
      - Create working examples that demonstrate the feature's benefits
      - Show clear error messages when feature is missing

      The demonstration should help developers understand:
      - Why MutexGuard is not Send by default
      - When and how to use the send_guard feature
      - What advanced synchronization patterns become possible
      - The trade-offs and safety considerations involved
    """
  })
