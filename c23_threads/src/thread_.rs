use std::thread;
use std::time::Duration;

pub fn simple_thread_demo() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // handle.join().unwrap(); // 在handle结束后才会执行当前线程(主线程)的内容

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // let x = handle.join();
    handle.join().unwrap();
}

pub fn thread_move_closure_demo() {
    let v = vec![1, 2, 3];
    //闭包判断上下文只需要一个借用，但是闭包无法判断新建线程何时结束，导致闭包中的变量生命周期不可控，不能只用借用 而是 所有权
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        thread::sleep(Duration::from_millis(1));
    });

    handle.join().unwrap();
}

/*
  tip:线程
    线程创建方式:
      1.thread::spawn(|| {}) 此函数返回一个JoinHandle的所有权实例
      2.JoinHandle.join() 阻塞当前线程直到JoinHandle的线程结束 返回一个result

    线程休眠: thread::sleep(dur)
    线程与闭包: 使用move关键字强制转移值的所有权


*/
