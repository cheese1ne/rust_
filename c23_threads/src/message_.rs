use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn simple_channel_demo() {
    //mpsc: multiple producer single consumer
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        //新建线程通过信道发送者发送消息
        sender.send(val).unwrap();
    });

    //主线程通过信道接收者来获取发送者传递的数据
    let val = receiver.recv().unwrap(); //阻塞式的获取信息的方式

    // thread::sleep(Duration::from_millis(1)); //主线程停顿保证新建线程发送消息
    // let val = receiver.try_recv().unwrap(); //非阻塞的获取信息方式
    println!("Got: {}", val);
}

pub fn multiple_message_one_sender_demo() {
    //一个生成发送多条消息
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            //新建线程通过信道发送者发送消息
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //可以将receiver作为一个迭代器使用，隐式的调用recv方法
    for val in receiver {
        println!("Got: {}", val);
    }
}

pub fn muliple_sender_more_message_demo() {
    //多个生产者发送更多的消息
    let (sender, receiver) = mpsc::channel();
    let sender_clone = sender.clone();
    //一个生产者绑定一个线程
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            //新建线程通过信道发送者发送消息
            sender.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    //
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            //新建线程通过信道发送者发送消息
            sender_clone.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    //可以将receiver作为一个迭代器使用，隐式的调用recv方法
    for val in receiver {
        println!("Got: {}", val);
    }
}

/*
  tip: 线程间的通信 不要通过共享内存来通讯，而是通过通讯来共享内存
    信道: 信道由发送者和接收者组成，当发送者和接收者任一被丢弃时可以认为信道被关闭了
      创建一个信道: std::sync::mpsc::channel() 返回一个元组，包括一个发送者和一个接收者
      发送消息: Sender.send(T) send方法回获取参数的所有权并移动这个值归接收者所有
      接收消息: Receiver.recv() 用于阻塞式的接收消息，Receiver.try_recv() 不会阻塞

      多个发送者: 通过clone方法来克隆多个发送者

        Send:
            1.Send trait是一个标记，它表明类型值的所有权可以在线程间转移的
            2.几乎所有类型都实现了Send，也有特例，比如Rc，仅适用于单线程场景
            3.Send trait用于并发场景
        Sync:
            1.Sync trait是一个标记，它表明实现Sync的类型可以安全的在多个线程中拥有值的引用
            2.Mutex<T>是Sync的实现，Rc<T>、RefCell<T>不是Sync的实现
            3.Sync用于并发下的共享数据访问

*/
