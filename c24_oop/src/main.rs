use c24_oop::{Button, Draw, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                lable: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/*
    tip:Rust的面向对象特性：
        1.对象：程序由对象组成，各种各样的对象组合调度实现了程序的功能
        2.封装：通过结构体的定义完成封装，隐藏结构体细节，包含数据(成员)和行为(关联函数)，注意Rust中称为结构体而不是对象
        3.继承：Rust中无继承，通过使用trait的默认方法来进行有限的共享


*/
