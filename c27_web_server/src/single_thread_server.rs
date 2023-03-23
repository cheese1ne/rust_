use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn single_thread_server_demo() {
    // 创建web端口的监听器
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 仅在主线程中处理请求，性能低
        handle_connection(stream);
        println!("Connection established!");
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

/*
    tip:
        HTTP:
            请求格式:
                Method Request-URI HTTP-Version CRLF
                headers CRLF
                message-body
            响应格式:
                HTTP-Version Status-Code Reason-Phrase CRLF
                headers CRLF
                message-body


*/
