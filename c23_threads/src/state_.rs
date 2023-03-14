use std::sync::{Arc, Mutex};
use std::thread;

pub fn simple_mutex_demo() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);
}

pub fn thread_mutex_state_demo() {
    //Rc不能安全的在线程间共享!!! 使用原子性的引用计数器Arc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

/*
    tip:线程间共享状态
      共享内存类似于多所有权: 多个线程可以同时访问相同的内存位置。Rust中多所有权使用Rc，但这仅限于在单线程的环境下,因为Rc在并发情况下无法保证修改计数的安全
        线程安全的计数器使用Arc<T> 意义为一个atomic的Rc (原子性的引用计数器)
      互斥器: 在并发的情况下，再同一个时刻只允许一个线程访问数据 类似其他语言中的锁处理
        1.创建互斥器: Mutex::new()
        2.Mutex.lock()
            lock方法会阻塞当前线程 直到当前线程拥有锁为止
            返回一个MutexGuard<T> 智能指针，此指针实现了Deref来指向内部的数据，他也提供了一个Drop实现在MutexGuard<T>离开作用域时自动释放锁
      原子引用计数器:参考Java中的原子类
        使用RefCell<T>改变Rc<T>中的内容，Mutex提供了内部可变性
        使用Mutex<T>改变Arc<T>中的内容，Mutex提供了内部可变性


    other:Mutex使用时存在死锁的风险，请注意


*/
