fn main() {
    //整数
    // let int: u8 = 256;
    let int: u32 = 256;
    let long: u64 = 2 ^ 15 - 1;

    //浮点数
    let double = 2.010; //默认 f64
    let float: f32 = 1.2;

    //布尔值
    let boolean: bool = true;

    //字符类型
    let char_: char = 'A';
    let str_ = "ABC";

    //元组 及 元组解构
    let tuple: (u32, f64, &str) = (100, 0.8, "watermelon");
    let (x, y, z) = tuple;

    //数组
    let months: [&str; 12] = [
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

    println!("the value of int is {int}");
    println!("the value of long is {long}");
    println!("the value of double is {double}");
    println!("the value of float is {float}");
    println!("the value of boolean is {boolean}");
    println!("the value of char_ is {char_}");
    println!("the value of str_ is {str_}");
    println!("the value of tuple is {x}, {y}, {z}");
    println!(
        "the value of tuple by index is {}, {}, {}",
        { tuple.0 },
        { tuple.1 },
        { tuple.2 }
    );
    println!("the value of months is {}", months[0])
}

/*
    tip:rust数据类型有两大类：标量和复合
        1.标量类型：scalar，代表一个单独的值，rust有四种基本的标量类型：整数、浮点数、布尔、字符
            a.整数：
                - 整形： 根据长度以及有无符号进行区分：例如 8-bit 有符号的类型为 i8，无符号的类型为 u8; 32-bit 有符号的类型为 i32，无符号的类型为 u32
                - 整形字面值：Decimal (十进制) 、Hex (十六进制) 、 Octal (八进制) 、 Binary (二进制)
                - 溢出问题：integer overflow 在budeg模式下会被panic，在release模式下溢出会进行二进制补码的操作改变变量的值
            b.浮点数：f32是单精度浮点数、f64是双精度浮点数，默认类型是f64

            c.布尔值：true 和 false，rust中的布尔类型用bool表示

            d.字符类型：char类型是最原生的字母类型
                - char: 字符类型，大小为四个字节
                - 字符串：

        2.复合类型：compound，将多个值组合成一个类型，rust原生的复合类型：元组 tuple (和你想的一样，此处的元组与python中是一样的)和 数组 array
            a.原生复合：
                - 元组：元组是一个将多个其他类型的值组合仅一个复合类型的主要方式，元组长度固定，一旦声明，其长度不会增大或减小，元组可以通过解构来快速绑定值，通过tuple.index来访问元组中的元素
                - 数组：数组中的元素类型必须相同，Rust的长度是固定的，通过arr[index]来访问数组中的元素

*/
