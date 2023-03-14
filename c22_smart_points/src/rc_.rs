use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::rc_::List::{Cons, Nil};

pub fn simple_rc_demo() {
    //打印出引用计数
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

/*
  tip:引用计数，Rust中使用Rc显示的启用多所有权
  Rc是多所有权、Box和RefCell是单所有权
  在其他语言中作为垃圾回收的算法，用于记录一个值引用的数量从而知晓这个值是否被使用
  举个例子：相交链表，单个值存在多个所有者的情况。
    引用计数存在问题：如果存在循环引用，那么数据永远会被判断为使用中，内存无法被释放，最终导致内存泄漏的问题
    解决方案：将Rc<T> 变为 Weak<T>


*/
