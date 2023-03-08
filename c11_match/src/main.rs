fn main() {
    //一个简单的match使用示例
    simple_match_demo();
    //option的match使用示例
    option_match_demo();
}

fn simple_match_demo() {
    // let value = Coin::value_in_cents(Coin::Dime);
    let value = Coin::value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("the value of cents is {}", value);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("the state of Quarter is {:?}", { state });
                25
            }
        }
    }
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn option_match_demo() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("the value of five is {:?}", five);
    println!("the value of six is {:?}", six);
    println!("the value of none is {:?}", none);
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
        // _ => None,
    }
}

/*
    tip:match 模式匹配 一般与枚举结合使用
        1.模式匹配通过match开始，语法可以理解为Java中的枚举switch
        2.Rust中模式匹配必须是穷尽的，必须处理所有可能的选项
        3.Rust提供的一种特殊的模式：_ ，用于匹配其他不复合条件的模式

    在Rust中，_ 的含义是不被需要的数据


*/
