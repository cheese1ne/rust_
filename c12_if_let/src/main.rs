fn main() {
    //一个简单的if let简洁流示例
    simple_if_let_demo();
    //if let else示例
    if_let_else_demo();
}

fn simple_if_let_demo() {
    //match语法
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    //使用if简洁流简化的语法
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn if_let_else_demo() {
    // let coin = Coin::Dime;
    let coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("the state of Quarter is {:?}", { state });
    } else {
        count += 1;
    }
    println!("the count is {}", count);
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
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

/*
    tip:if let简洁控制流
        1.用来处理match只匹配一个模式而忽略其他模式的情况
*/
