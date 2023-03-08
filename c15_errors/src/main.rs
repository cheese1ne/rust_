use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

fn main() {
    //简单的异常
    // simple_panic_demo();

    //panic!的backtrace
    // panic_backtrace_demo();

    //result处理panic
    // use_result_resolve_panic_demo();

    //result.unwrap
    // use_result_unwrap_demo();

    //result.expect
    // use_result_expect_demo();

    //传播错误
    // spread_painc_demo();
}

fn simple_panic_demo() {
    panic!("crash and burn")
}

fn panic_backtrace_demo() {
    //定义一个Vector列表
    let v = vec![1, 2, 3];
    //制造一个panic
    v[99];
}

fn use_result_resolve_panic_demo() {
    //File::open的返回值的Result
    let file_io_result = File::open("/Users/sobann/code-nav/rust/errors_/hello.txt");
    let file_io = match file_io_result {
        Ok(file) => file,
        //基础的异常处理，只要读取不到就panic
        // Err(error) => panic!("Problem opening the file: {:?}", error),

        //针对不同异常进行匹配
        Err(error) => match error.kind() {
            //not_found == ErrorKind::NotFound
            not_found => match File::create("/Users/sobann/code-nav/rust/errors_/hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error);
            }
        },
    };
}

fn use_result_unwrap_demo() {
    let file_io_result = File::open("/Users/sobann/code-nav/rust/errors_/hello.txt");
    let file_io = file_io_result.unwrap();
}

fn use_result_expect_demo() {
    let file_io_result = File::open("/Users/sobann/code-nav/rust/errors_/hello.txt");
    let file_io = file_io_result.expect("hello.txt should be included in this project");
}

fn spread_painc_demo() {
    //传播错误
    let username = read_username_from_file();

    //传播错误的简写 ? 运算符
    let username = read_username_from_file_shorthand();
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("/Users/sobann/code-nav/rust/errors_/hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        //文件读取错误时传出错误
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    //返回读取的字符串
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        //io读取错误时传出错误
        Err(e) => Err(e),
    }
}

//read_username_from_file通过?运算符简写
fn read_username_from_file_shorthand() -> Result<String, io::Error> {
    //通过?运算符向调用者返回错误的函数 如果Result是Ok，表达式返回Ok的值并让程序继续执行，若果是Err则会将Err作为整个函数的返回值
    let mut username_file = File::open("/Users/sobann/code-nav/rust/errors_/hello.txt")?;
    let mut username = String::new();
    //通过?运算符向调用者返回错误的函数
    username_file.read_to_string(&mut username)?;
    Ok(username)
    //进一步简写
    // let mut username = String::new();
    //链式
    // File::open("hello.txt")?.read_to_string(&mut username)?;
    // Ok(username)

    //使用fs::read_to_string的更简洁的写法
    // fs::read_to_string("") //它会打开文件，新建String，读取内容，将内容放入String，返回String
}

fn option_shorthand_demo() -> Option<i32> {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 20);
    scores.entry(String::from("Blue")).or_insert(10);

    // let x = scores.get(String::from("Blue"))
    scores.get(&String::from("xx")).copied()
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // let first_line_option = text.lines().next();
    // let first_line = match first_line_option {
    //     Some(line) => line,
    //     None => return None,
    // };

    // let chars = first_line.chars();
    // chars.last()

    //若next正确则继续向下 简化写法
    text.lines().next()?.chars().last()
}

/*
    tip:异常
        1.panic
            panic的方式：
                a.代码执行造成代码panic的操作：例如从数组中获取一个越界索引的数据
                b.显示调用panic!宏：panic!("crash and burn")
            栈展开或终止：出现panic时，Rust对于内存处理的执行策略
                a.栈展开：默认的策略，Rust回溯栈并清理它遇到的每一个函数的数据
                b.终止：不清理数据直接退出程序，程序使用的内存交给操作系统来处理，在Cargo.toml中[profile]下配置panic = 'abort'
            panic的backtrace：通过手动置顶RUST_BACKTRACE环境变量运行cargo run，可以获取到panic中所有被调用函数的列表
        2.Result：用来处理可恢复的错误而不是让程序停止
            File::open返回的Err成员是io:Error，这个结构体中包含一个kind方法，可以获取错误的具体类型io::ErrorKind，这是一个枚举，我们可以根据具体的错误枚举完成match匹配
            通过Result的辅助函数unwrap
            通过Result的辅助函数expect
            错误传播：所有成功的信息和错误的情况通过Result向上传播，让调用者进行处理
            ?运算符：?运算符只能用于返回值与?作用的值相兼容的函数，栗子中?运算符的语句返回一个Result实例，且方法返回值类型为Result<String, io::Error>
                Result使用?
                Option使用?
                main方法也可以使用?，结合trait使用，栗子：fn main() -> Result<(), Box<dyn Error>> {...?; Ok(())}----- Box<dyn Error>是一个trait对象
*/
