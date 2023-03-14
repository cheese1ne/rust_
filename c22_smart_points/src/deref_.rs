use std::ops::Deref;

pub fn simple_deref_demo() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}

pub fn diy_deref_demo() {
    let x = 5;
    let y = MyBox::<i32>(x);

    assert_eq!(5, x);
    //*y在底层被转换称为*(y.deref())
    assert_eq!(5, *y);
}

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn hidden_deref_demo() {
    let m = MyBox::<String>(String::from("Rust"));
    //String强转为str
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

/*
  tip:
    解引用运算符:*，用于追踪指针的值，也可以解实现了Deref trait的结构体，例如Box，底层调用了deref方法
    自定义智能指针：模仿Box，参考栗子中的MyBox
    隐式的转换：Deref 强制转换可以将 &String 转换为 &str

    强制转换规则：
      1.当 T: Deref<Target=U> 时从 &T 到 &U
      2.当 T: DerefMut<Target=U> 时从 &mut T 到 &mut U
      3.当 T: Deref<Target=U> 时从 &mut T 到 &U


    String实现了Deref返回&str




*/
