use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

pub fn reference_cycle_demo() {
    //创建一个叶子节点
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    //创建父节点
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    //修改leaf使其拥有指向父节点的Weak<Node>的引用
    let x = Rc::downgrade(&branch);
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); //使用borrow_mut方法获取一个可变引用，通过Rc::downgrade函数从branch中Rc<Node>的值创建了一个指向branch的Weak<Node>引用, *用于修改值

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

pub fn weak_reference_count() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // let x = Rc::downgrade(&branch);
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // let x = Rc::strong_count(&branch);
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }
}

/*
    tip:Weak<T> 用于解决引用计数无法解决的循环依赖问题
    使用方式: 将原本的Rc<T> 用 Weak<T> 代替
    一些函数:
      1.Weak::new() 创建一个空的弱引用
      2.Rc::downgrade() 传入一个引用，返回一个弱引用
      3.Rc::strong_count() 传入一个引用，返回强引用的数量
      4.Rc::weak_count() 传入一个引用，返回弱引用的数量

*/
