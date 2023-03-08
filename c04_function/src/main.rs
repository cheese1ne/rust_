fn main() {
    println!("main function.");
    another_function();
    print_num(33);
    print_dict("waterlemon", -100);
    block();
    let sum = sum(100, 200);
    println!("the value of sum is {}", sum);
}

/**
 * my first rust function
 */
fn another_function() {
    println!("another function.");
}

/**
 * 需要一个数值类型参数
 */
fn print_num(num: u32) {
    println!("the value of num is {num}");
}

fn print_dict(key: &str, value: i32) {
    println!("the value of dict is {} : {}", key, value);
}

fn block() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

/*
    tip:通过fn关键字来定义新函数，(){}结构与其他语言一致
        1.参数: 指定类型 x : dataType，栗子print_dict
        2.语句和表达式: 执行但不返回的指令，如定义变量，执行方法等；表达式的结尾没有分号。如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值!!!
        3.返回值: 需要在() -> 后声明他的类型，函数的返回值等于最后一个表达式的值

    总结：函数与其他语言基本一致，但是需要注意的是 语句末尾加";" 和不加 ";"的逻辑是不同的，一个是普通语句，一个是返回值表达式，指定类型与python有些类似
*/
