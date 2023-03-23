use std::sync::{mpsc, Arc, Mutex};
use std::thread;

// 当前模块别名定义
// 任务是一个只能执行一次的闭包，需要在线程间通信，生命周期与线程一致
type Job = Box<dyn FnOnce() + Send + 'static>;

// struct Job;

// oop的设计 池子中没必要直接存放线程对象，而是带有一些工作能力和其他属性的请求处理者
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // 创建一个内部循环处理job的线程交给work进行管理
        let thread = thread::spawn(move || loop {
            // 接收要处理的任务，实际是个闭包，recv方法阻塞的调用用于获取消息
            let message = receiver.lock().unwrap().recv();

            // 循环内部需要根据消息判断结果，若阻塞获取的数据为None代表信道被关闭，此时中断线程的循环
            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");
                    job();
                } // 当获取到任务时执行闭包
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                } // 当信道被关闭时，走这一分支，此时停止线程的循环任务
            }

            println!("Worker {id} got a job; executing.");
        });
        Worker {
            id,
            thread: Some(thread), //使用Option进行包装
        }
    }
}
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);
        // 线程池初始化时，创建一个信道
        let (sender, receiver) = mpsc::channel();

        // 通过Arc智能指针提供 安全的能共享的 多个所有权接收者 此处使用到两个智能指针 Arc Mutex
        // Arc 让多个Worker具有接收者的所有权
        // Mutex 确保一次只有一个Worker能从接收端得到任务
        let receiver = Arc::new(Mutex::new(receiver));

        // 初始化一个工作者列表
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // 这里可以完成工作者一些属性的构建
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    // 线程的闭包是一个FnOnce类型的闭包
    where
        F: FnOnce() + Send + 'static,
    {
        // 构建任务，并通过发送者向线程池分发信息，由于FnOnce的特殊性，最后只有一个线程会执行这个被封装的闭包
        let job = Box::new(f);
        // 通过发送者发送任务信息
        self.sender.as_ref().unwrap().send(job).unwrap(); // Option.as_ref()将 Option<T> 转化为 Option<&T>, 此处为 &Sender<Box<dyn FnOnce() + Send>>
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // 通过drop函数来销毁sender，丢弃sender会关闭信道，因为关闭了信道，接收者部分也需要做出调整
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            // worker.thread.join().unwrap(); //只有worker的可变引用但是join方法会获取所有权，需要将thread的所有权从worker中移出使得可以通过join方法消费

            if let Some(thread) = worker.thread.take() {
                //Option.take方法转移所有权
                thread.join().unwrap();
            }
        }
    }
}

/*
    tip: 多线程的设计 每一步演变都是有具体的原因的
      单线程处理性能低下 =>
        请求就创建一个线程处理请求 =>
          每次请求都创建线程，内存是有上限的，且创建和销毁的过程是消耗资源的 =>
            设计一个线程池，每次请求都从池子中获取已有的线程处理请求 =>
              通过OO的设计引入Worker来完成线程工作的处理，每一个worker初始化一个已运行的线程和其他属性，等待分配工作任务完成工作! =>
                分配工作通过 线程间通信完成，线程池通过信道来向线程发送请求，线程池本身维护一个信道并作为消息发送者，每一个worker是接收者(可以视作一个特殊的观察者模式？)，新建Job结构体来存放信道中的闭包，线程池发送期望执行的任务，Worker遍历接收者并处理任务!!!
                    线程停机和清理，让Worker和Thread实例解耦，通过Option.take方法来将变量所有权进行转移，并在线程循环内部根据信道内容终止thread中的循环闭包
*/
