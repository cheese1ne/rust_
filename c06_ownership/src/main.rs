fn main() {
    //一个所有权的demo 字符串申请运行时内存
    string_demo();
    //为了确保内存安全，当一个变量也指向一个已经被指向内存的数据时，先前一个指向内存空间的变量就会被视为无效引用！
    move_demo();
    //heap clone 堆上数据的克隆，将所有的内容全部拷贝一份
    heap_clone_demo();
    //stack clone 栈上数据的克隆，编译时期已知大小的类型被整个储存在栈上，所以拷贝是快速的，可以理解为其他语言中的浅拷贝
    stack_clone_demo();
    //所有权与函数
    ownership_method_demo();
    //返回值、作用域与所有权
    ownership_return_demo();
}

fn string_demo() {
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{s}")
}

fn move_demo() {
    //s2复制s1的指针、长度和容量，在Rust中被称为移动 (相交于其他语言中的浅拷贝)
    let s1 = String::from("hello");
    let _s2 = s1;
    // println!("{}, world!", s1); // 移动后使用开始的变量会出错
}

fn heap_clone_demo() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn stack_clone_demo() {
    let x: i32 = 1;
    let y = x;
    println!("{x}, {y}")
}

fn ownership_method_demo() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s); // s 的值移动到函数里
                        // 所以到这里不再有效
                        // println!("{s}"); 会出错
    let x = 5; // x 进入作用域
    makes_copy(x); // x 应该移动函数里，
                   // 但i32 是 Copy 的，所以在后面可继续使用 x
                   // println!("{x}"); 可以使用
} // 这里，x 先移出了作用域，然后是 s。但s 的值已被移走

fn takes_ownership(some_string: String) {
    // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) {
    // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域

fn ownership_return_demo() {
    let s1 = gives_ownership(); // gives_ownership 将返回值
                                // 转移给 s1
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到
                                       // takes_and_gives_back 中，
                                       // 它也将返回值移给 s3
                                       // println!("{s2}") 无法使用
} // 这里，s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {
    // gives_ownership 会将
    // 返回值移动给
    // 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域。
    some_string // 返回 some_string
                // 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String {
    // a_string 进入作用域
    a_string // 返回 a_string 并移出给调用的函数
}
/*
    tip:所有权(ownership),Rust与其他语言最与众不同的特性，通过此特性保证系统的内存的安全
        1.管理程序内存：与其他语言处理计算内存管理的方式不同，不使用垃圾回收机制和程序猿手动分配和释放操作，而是通过所有权系统进行管理，违反所有权规则甚至不能编译！！！
        2.所有者：
            a.Rust中每一个值都有一个所有者;
            b.值在任一时刻有且只有一个所有者;
            c.当所有者(变量)离开作用域，这个值将被丢弃.
            d.为了保证内存安全，一块内存空间只能被一个所有者使用，否则会panic (could not compile `ownership` due to previous error)
        3.变量作用域：
            a.变量在程序中有效的范围，与其他语言类似，
            b.内存在拥有他的变量离开作用域之后会被自动释放!!! Rust会在变量离开作用域时调用一个名为drop的特殊函数用于释放内存
        4.特殊情况：
            Copy trait：Rust中一个特殊的注解，如果一个类型实现了Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用
                已知的实现了Copy trait的类型：
                    - 所有整数类型：如u32
                    - 布尔类型
                    - 所有浮点数类型：如f64
                    - 字符类型：char
                    - 元组：当且仅当元组包含的类型全部实现Copy trait时


    Rust中的字符串：
        1.&str类型与String不同，&str是字符串字面量类型，属于编译时就确定的内容，这种类型的字符串是不可变的 (字符串不可变性可参考java)，具有多线程安全、效率高的优点
        2.String类型在运行时向内存分配器请求内存，需要一个处理完String时将内存返还给内存分配器的方法!!!


*/
