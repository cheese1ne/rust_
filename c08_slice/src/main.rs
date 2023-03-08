fn main() {
    //获取第一个单词的的位置，
    find_first_word();
    //slice str
    slice_str_demo();
    //slice arr
    slice_arr_demo();
}

/**
 * 该函数接收一个用空格分隔单词的字符串，返回在该字符串中找到的第一个单词
 * 如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串
 *
 * 存在问题：第一个单词的索引与str本身数据不同步，若字符本身可变，可能会产生严重的错误
 */
fn find_first_word() {
    // let mut str = String::from("I am a king!");
    let str = String::from("I am a king!");
    let index = first_word(&str);
    println!("the index of first word is {}", index)
    // str.clear(); //这清空了字符串，使其等于 ""，clean方法需要清空String 此时会尝试获取一个可变引用
    // index 在作用域结束之前仍存在，但这是一个无意义的数字了
}

fn first_word(str: &String) -> usize {
    let bytes = str.as_bytes();

    //迭代器遍历 元组中为index和元素
    for (i, &item) in bytes.iter().enumerate() {
        //此处的b为byte类型 当元素 = ' '时 返回当前的索引
        if item == b' ' {
            //相等时返回b 指的是找到了空格
            return i;
        }
    }
    //没有空格 返回整个长度
    str.len()
}

fn slice_str_demo() {
    let s = String::from("hello world");

    // hello 是 一个部分String的引用，由[0..5]部分指定，slice规则 [starting_index..ending_index]，字符串长度为ending_index - starting_index
    let hello = &s[0..5]; // 0是slice的初始位置 5是slice的终止位置 左开右闭
    let world = &s[6..11]; // 6是slice的初始位置 11是slice的终止位置

    println!("{hello}");
    println!("{world}");

    // 根据以上规则，slice获取字符串本身全部的引用可以写为
    let hello_world_full1 = &s[0..s.len()];

    //与python相同，若左侧或右侧为边界，可以省略，最终可以省略为
    let hello_world_full2 = &s[..];

    println!("obtain the full slice of str is {}", hello_world_full1);
    println!("obtain the full slice of str is {}", hello_world_full2);

    // let mut str = String::from("I am a king!");
    let str = String::from("I am a king!");
    let first_word = find_first_word_by_slice(&str);
    // str.clear(); clean方法需要清空String 此时会尝试获取一个可变引用，
    println!("the first word of str is {}", first_word); //后续的println!宏需要一个引用

    //通过引用和借用一节的结论：rust不允许在统一作用域内有一个变量的可变引用和不可变引用，如果出现编译失败
}

fn find_first_word_by_slice(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (index, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..index];
        }
    }
    &str[..]
}

fn slice_arr_demo() {
    let arr = ["1", "2", "3", "4", "5"];
    //取数组的前三个字符
    let sub_arr = &arr[0..3];
    let mut sub_str = String::from("");
    for (_index, &item) in sub_arr.iter().enumerate() {
        sub_str.push_str(&item);
    }
    println!("the value of subarr is {}", sub_str);
}

/*
   tip:slice用于安全的截取字符串和数组
    1.用法：语法上与python较为类似，[starting_index..ending_index]，左右边界与元数据一致时可省略
    2.特殊点：slice字符串时候，这个参数必须是一个引用，rust的安全机制，详细参考引用和借用
*/
