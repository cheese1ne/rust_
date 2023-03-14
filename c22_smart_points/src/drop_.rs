pub fn simple_drop_demo() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn manual_drop_demo() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); //通过std::mem::drop方法来提前销毁值

    //不能使用c.drop()，因为结构体结束时也回调用一次
    // c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
}
