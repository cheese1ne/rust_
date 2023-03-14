pub fn simple_box_deo() {
    let x = Box::new(10); // 创建一个指向堆上分配的整数 10 的 Box
    println!("x = {}", x); // 解引用 Box，并打印其中的值
}

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::box_::List::{Cons, Nil};

pub fn recursion_demo() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

/*
    tip：理解成Java中基本类型的包装类
*/
