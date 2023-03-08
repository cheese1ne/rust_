fn main() {
    //引用
    reference_and_borrow_demo();
    //可变引用
    mutable_reference_demo();
    //悬挂指针测试
    dangling_reference_demo();
    //字符串的字面量类型就是引用
    let str: &str = "我是一个不可变的字符串字面量";
    println!("{str}")
}

fn reference_and_borrow_demo() {
    let s1 = String::from("hello"); //进入作用域

    //创建一个引用的行为被称为借用
    let len = calculate_length(&s1); //这里通过&符号代表传递s1的引用而不是所有权，允许使用变量的值但不获取其所有权
    println!("The length of '{}' is {}.", s1, len); //所以接下来仍热可以继续使用s1
}

// 定义参数时 也需要定义是否转移或者引用 默认为转移，转移的所有权会发生变动
fn calculate_length(s: &String) -> usize {
    //借用的数据在引用作用域内无法改变
    // s.push_str("fake"); // is a `&` reference, so the data it refers to cannot be borrowed as mutable

    //s 是 String的引用
    s.len()
} // s离开了作用域 但因为s不拥有引用值的所有权，故不会发生内存清理的drop操作

fn mutable_reference_demo() {
    let mut str = String::from("hello");
    change_reference(&mut str);
    println!("the mutable reference str is {}", str);
}

fn change_reference(str: &mut String) {
    str.push_str(", world");
}

fn dangling_reference_demo() {
    // 悬挂指针在rust编译时就会报错
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
    println!("the value of no_dangle is {}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

/*
// dangle 返回一个字符串的引用
// fn dangle() -> &String {
fn dangle() -> &String {
    let s = String::from("hello"); // s 是一个新字符串
    &s // 返回字符串 s 的引用
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 会产生悬挂指针

*/

/*
    tip:引用和借用
    1.引用：类似一个指针，可以访问储存在该地址的属于其他变量的数据。与指针不同，引用确保指向某个类型的有效值，使用时通过&符号来完成引用或定义引用
        a.引用可以使用指向数据的有效值但不获取其所有权，所以所有权不会发生变化
        b.Rust避免了悬垂引用的产生 (具有指针的语言中存在的 通过释放内存时保留指向它的指针造成的错误情况)
    2.借用：创建引用的行为被称为借用
        a.可变引用：通常情况下，借用的数据在引用作用域内无法改变，若要改变引用数据的有效值，则需要可变引用，可变引用通过&mut 进行修饰
            - 限制：
                在一个作用域内不能对一个变量创建两个可变引用!!! tip:在java中 由于java虚拟机对指令重排的影响可能对程序运行结果造成较大的差异，这个问题在Rust这钟安全策略下避免了 （java 中通过volatile 来显示的处理）
                在同一个作用域内不能同时拥有可变引用和不可变引用!!!
                总结：在同一个作用域中 读和写只能进行一个 且写入操作的主体只能有一个

    总结：
        a.一个作用域中，要么只能有一个可变引用，要么只能有多个不可变引用
        b.引用必须是有效的 (数据安全)
*/
