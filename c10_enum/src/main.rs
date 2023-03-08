use std::os::macos::raw::stat;

fn main() {
    //创建枚举示例
    create_enum_demo();
    //枚举作为属性成员的示例
    simple_member_demo();
    //具有成员属性的枚举
    member_value_enum_demo();
    //修改任务的状态
    change_task_status_demo();
    //Option枚举
    option_enum_demo();
}

fn create_enum_demo() {
    let v4 = IpAddrKind::V4;
    let v6 = IpAddrKind::V6;
    println!("{:?}", v4);
    println!("{:?}", v6);
}

fn simple_member_demo() {
    let ip_addr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    println!("the ipAddr is {:?}", ip_addr);
}

fn member_value_enum_demo() {
    let v4 = IpAddrNew::V4(String::from("127.0.0.1"));
    let v6 = IpAddrNew::V6(String::from("::1"));
    println!("{:?}", v4);
    println!("{:?}", v6);
}

fn change_task_status_demo() {
    let mut task = Task {
        id: 32,
        name: String::from("clean"),
        status: Status::Create,
    };
    println!("the task info is {:?}", task);
    //修改任务状态为暂停
    Task::change_status(&mut task, Status::Pause);
    println!("the task info is {:?}", task);
}

fn option_enum_demo() {
    //Option会推断数据的泛型
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
}

// rust中定义枚举通过关键字 enum
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

//结构体成员中使用枚举
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

//具有参数的枚举
#[derive(Debug)]
enum IpAddrNew {
    V4(String),
    V6(String),
}

//一个信息的枚举
#[derive(Debug)]
enum Status {
    //创建状态
    Create,
    //运行状态
    Running,
    //暂停状态
    Pause,
    //完成状态
    Completed,
}

//任务
#[derive(Debug)]
struct Task {
    id: u64,
    name: String,
    status: Status,
}

//定义task的关联函数
impl Task {
    //修改task的状态
    fn change_status(&mut self, status: Status) {
        self.status = status;
    }
}

/*
    tip:枚举 enum
        1.


*/
