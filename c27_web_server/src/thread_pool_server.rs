use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

use c27_web_server::ThreadPool;

pub fn thread_pool_server_demo() {
    // 创建web端口的监听器
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // 创建一个线程池
    let thread_pool = ThreadPool::new(10);
    // 创建一个线程池

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 每次请求进来创建一个线程处理请求，不太合理需要修改
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });
        // println!("Connection established!");

        // 每次请求进来通过已经初始化好的线程池向其他线程发送一条信息
        thread_pool.execute(|| handle_connection(stream))
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    // let response = "HTTP/1.1 200 OK\r\n\r\n";
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("c27_web_server/hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
