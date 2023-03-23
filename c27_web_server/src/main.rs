pub mod multi_thread_server;
pub mod single_thread_server;
pub mod thread_pool_server;

fn main() {
    // 单线程的web server
    // single_thread_server::single_thread_server_demo();

    // 多线程的web server
    // multi_thread_server::multi_thread_server_demo();

    // 线程池的web server
    thread_pool_server::thread_pool_server_demo();
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
