fn main() {
    //if-else 基本使用示例
    basic_if_else();
    //在let语句中使用if-else
    let_if_eles();
    //loop 基本使用示例
    basic_loop();
    //loop 返回值
    let_loop();
    //循环标签 用于多重循环消除歧义，与java中循环定义名称功能一致，定义别名时以'开始 ，这个例子可以看出rust中隐藏的特点!!!
    loop_lable();
    //while 基本使用示例 此例子中使用mut，可以从隐藏和mut例子来学习两者的不同点
    basic_while();
    //for 根据元素遍历
    basic_for_element();
    //for 根据索引遍历
    basic_for_index();
}

fn cmp_with_100(num: i32) -> bool {
    num > 100
}

fn basic_if_else() {
    if cmp_with_100(99) {
        println!("the num large than 100!")
    } else {
        println!("the num less than 100!")
    }
}

fn let_if_eles() {
    let description = if cmp_with_100(100) { "large" } else { "less" };
    println!("the value of description is {}", description);
}

fn basic_loop() {
    loop {
        println!("try again!");
        //tip 欺骗编译器 结束死循环
        if true {
            break;
        }
    }
}

fn let_loop() {
    let mut count = 0;
    let loop_value = loop {
        count += 1;
        if cmp_with_100(count) {
            break count * 2;
        }
    };
    println!("The loop result is {loop_value}");
}

fn loop_lable() {
    let x = 1;
    let sum = 0;
    'x_loop: loop {
        let mut y = 1;
        loop {
            let sum = x + y;
            y += 1;

            if cmp_with_100(sum) {
                println!("the value of x is {}", x);
                println!("the value of y is {}", y);
                println!("the value of sum in loop is {}", sum);
                break 'x_loop;
            }
        }
    }
    println!("the value of sum out loop is {}", sum);
}

fn basic_while() {
    let mut num = 103;
    let mut large_than_100 = cmp_with_100(num);
    while large_than_100 {
        println!("the num in while is {num}!");
        num -= 1;
        large_than_100 = cmp_with_100(num)
    }
}

fn basic_for_element() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for item in months {
        println!("the month in for element is: {item}");
    }
}

fn basic_for_index() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    for index in 0..months.len() {
        println!("the month in for index is: {}", months[index]);
    }
}

/*
    tip:Rust中常用的控制流：if-else 、loop 、 while 、 for
        1.if-else：
            a.基本使用与其他语言一致
            b.在let语句中使用，需要使用表达式作为返回值

        2.loop：
            a.基本使用与其他语言一致
            b.在let语句中使用，接收循环的返回值，循环返回值通过break关键字返回
            c.循环标签：一般用于多重循环中指定循环结构体的退出，定义时以'开始，退出循环时候通过break 'xxx;完成，参考java中循环标签的功能

        3.while：与其他语言基本一致

        4.for：遍历数组，与其他语言基本一致，分索引遍历和元素遍历
            a.元素遍历: 参考java中基本的for ---for item in arr {}
            b.索引遍历: 类似python中的range遍历 --- for index in 0..arr.len() {}
*/
