pub mod message_;
pub mod state_;
pub mod thread_;

fn main() {
    //线程创建、休眠和等待
    thread_::simple_thread_demo();
    //线程和闭包
    thread_::thread_move_closure_demo();

    //线程间的通信
    message_::simple_channel_demo();
    message_::multiple_message_one_sender_demo();
    message_::muliple_sender_more_message_demo();

    //线程间共享内存
    state_::simple_mutex_demo();
}
