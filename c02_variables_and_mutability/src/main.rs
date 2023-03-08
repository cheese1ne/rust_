fn main() {
    // 不可变变量
    let x = 5;
    println!("The value of x is: {x}");
    println!("The value of x is: {x}");

    //可变变量
    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

    //常量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    //隐藏
    let z = 1;
    let z = z + 1;
    {
        let z = z * 2;
        println!("The value of z in the inner scope is: {z}");
    }
    println!("The value of z is: {z}");

    //mut 与 隐藏
    let spaces_shadow = "  ";
    let spaces_shadow = spaces_shadow.len(); //隐藏可以改变变量的类型
    println!("The value of spaces_shadow is: {spaces_shadow}");

    let mut spaces_mut = " '2' ";
    // spaces_mut = spaces_mut.len(); //mut 不能改变变量的类型
    println!("The value of spaces_mut is: {spaces_mut}");
}
