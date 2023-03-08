use std::process;
use std::thread;
use std::vec;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ShirtColor {
    Red,
    Blue,
}

//库存
pub struct Inventory {
    pub shirts: Vec<ShirtColor>,
}

impl Inventory {
    //提供shirt的样式，若没有要求，默认按照库存中最多的颜色处理
    pub fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //这个闭包类似一个缺省处理，提供库存中最多的shirt
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

//简单的闭包
pub fn simple_closure_demo() {
    let x = 10;
    let add = |y| x + y;

    println!("{}", add(5));
}

//不可变引用的闭包 demo
pub fn immutabl_reference_borrow_closure_demo() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //定义一个闭包，这个闭包仅仅通过宏 获取一个list的不可变引用
    let only_borrows = || println!("From closure: {:?}", list);
    only_borrows();
    println!("After defining closure: {:?}", list);
}

//一个可变引用的闭包 demo
pub fn mutable_reference_borrow_closure_demo() {
    let mut list = vec![1, 2, 3]; //定义一个变量list
    println!("Before defining closure: {:?}", list);
    //声明一个可变引用的闭包
    // let mut borrows_mutably = move || {
    let mut borrows_mutably = || {
        //通过关键字move移动所有权到闭包
        list.push(100);
        println!("After defining closure: {:?}", list); //list的不可变引用
    };
    // println!("After defining closure: {:?}", list); //cannot borrow `list` as immutable because it is also borrowed as mutable
    borrows_mutably(); //闭包调用
    println!("After defining closure: {:?}", list); //当通过move移动所有权到闭包后，此处borrow of moved value: `list`
}

//闭包与多线程
pub fn concurrent_closure_demo() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    //通过move关键字将list所有权移动到工作线程中，防止持有所有权的线程结束后引用失效
    // thread::spawn(move || println!("From thread: {:?}", list))
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

//闭包的类型: FnOnce
pub fn option_unwrap_or_else() {
    let x: Result<i32, &str> = Ok(2);

    let closure_ = |err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    };
    //
    x.unwrap_or_else(closure_);
}

//闭包的类型: FnMut
pub fn slice_sort_by_key() {
    //定义一个slice
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
