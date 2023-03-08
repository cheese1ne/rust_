use std::collections::HashMap;

fn main() {
    //vector创建
    create_vector_demo();
    //添加元素
    push_vector_demo();
    //获取元素
    get_vetor_demo();
    //遍历元素
    iter_vetor_demo();
    //通过使用枚举包装来在vector中存放多中类型的数据，泛型为枚举类型
    multi_generic_demo();
    //一个所有权的demo
    ownership_vector_demo();

    //hash map创建
    create_hashmap_demo();
    //hash map和所有权
    ownership_hashmap_demo();
    //hash map经典操作，统计单词出现的次数
    word_quantity_from_str_demo();
}

fn create_vector_demo() {
    //通过Vec::new函数创建一个新的空Vector
    let v: Vec<i32> = Vec::new();
    //通过vec!宏创建一个带数据的Vector
    let v = vec![1, 2, 3];
}

fn push_vector_demo() {
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    v.push(6);
}

fn get_vetor_demo() {
    let v = vec![1, 2, 3, 4, 5];
    //索引语法
    let third = &v[2];
    println!("the third element is {}", third);
    println!("the vetor : {:?}", v);
    //get方法
    let third: Option<&i32> = v.get(2);

    match third {
        Some(value) => println!("the value of third match {}", value),
        _ => println!("the value is not match"),
    }

    if let Some(value) = third {
        println!("the value of third is {}", value);
    }
}

fn iter_vetor_demo() {
    let v = vec![100, 32, 57];
    //遍历v的不可变引用
    for i in &v {
        println!("{i}");
    }
    //遍历v的可变引用
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //给每一个元素+50，* 是解引用运算符，用于追踪指针的值
        *i += 50;
    }
    println!("the value of v {:?}", v)
}

fn multi_generic_demo() {
    let row = vec![
        SheetCell::Int(1i32),
        SheetCell::Float(3.23f64),
        SheetCell::Text(String::from("this is a cell")),
    ];

    println!("the multi vector value is {:?}", row);
}

#[derive(Debug)]
enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn ownership_vector_demo() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; //获取了vector内元素的不可变引用
    println!("The first element is: {first}");
    // v.push(6); //通过可变引用v增加vector中的元素，first在此处变得不太安全了，Rust编译会指出错误，后续若想继续使用v[0]的数据，需要重新从v中获取一个新的不可变引用
    println!("The first element is: {first}"); //检查器会确保vector内容的引用有效，当获取v的第一个元素的引用时，尝试通过

    //同样的是内存安全的问题：新增元素可能导致Vector重新分配新的空间，而数据的引用存在指向旧内存(已释放)
}

fn create_hashmap_demo() {
    //创建hashmap
    let mut scores = HashMap::new();
    //添加键值对，修改值的方法也是这个，和Java中的HashMap.put一致
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    //获取数据 get方法获取的是option
    let option = scores.get("Red");
    let value = match option {
        Some(value) => value,
        _ => &-1,
    };
    println!("the map red value is {}", value);

    //copied 将 Option<&T> 映射到 Option<T> ，unwrap_or类似Java中Optional.orEles
    let value = scores.get(&String::from("Blue")).copied().unwrap_or(-1i32);
    println!("the map blue value is {}", value);

    //在键没有对应值时插入键值对
    scores.entry(String::from("Yellow")).or_insert(30); //插入
    scores.entry(String::from("Blue")).or_insert(40); //已有键Blue所以不变

    let value = scores.get(&String::from("Blue")).copied().unwrap_or(-1i32);
    println!("the map blue value is {}", value);

    let value = scores
        .get(&String::from("Yellow"))
        .copied()
        .unwrap_or(-1i32);
    println!("the map Yellow value is {}", value);
}

fn ownership_hashmap_demo() {
    let mut field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    println!("{:?}", map);
    map.insert(field_name, field_value);
    // println!("{field_name}"); //键值被插入后就为hashmap所有，todo 可以尝试使用hashmap插入引用？
    println!("{:?}", map);
}

fn word_quantity_from_str_demo() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    //统计单词的个数
    for word in text.split_whitespace() {
        //or_insert返回一个可变的引用，默认为0， or_insert方法返回一个可变引用
        let count = map.entry(word).or_insert(0);
        //通过*解引用，并修改计数的个数
        *count += 1;
    } //每一次for循环count都会离开作用域
    println!("{:?}", map);
}

/*
    tip:常见集合
        Vector: 列表，基于数组
            - 创建：
                a.创建一个空的Vector：Vec::new；
                b.通过vec!宏创建一个带数据的Vector,泛型类型Rust会根据传入的元素进行推断
            - 泛型：存储值的类型
            - 存储：使用push方法向vector增加值，注意使用push时候必须声明vector实例的引用为可变类型变量
            - 读取：
                a.使用get方法获取vector中的元素，get方法返回的是一个Option，在处理超范围的情况使用这个
                b.使用索引获取vector中的元素，索引超范围时会panic
                c.使用pop来移除并返回vector中的最后一个元素
            - 遍历：for循环，遍历时修改需要vector的可变引用，使用 * 作为解引用运算符，用于追踪指针的值
            - 销毁：离开作用域时被释放，其中的内容也会被丢弃
            - 所有权/引用/借用：
                a.在vector中获取了一个不可变引用后，若vector发生了push操作，为保证指针安全之前的不可变引用不可使用(不安全的原因是扩容时数组会重新分配内存地址)
                b.操作vector内的元素使用 * 作为解引用运算符，用于追踪指针的值
                c.键值被插入后就为hashmap所有
        HashMap: 键值对
            - 创建：创建一个空的HashMap：HashMap::new;
            - 泛型：存储数据的类型
            - 保存：
                a.HashMap.insert:添加键值对，若hashmap中有添加的键会修改此键对应的值
                b.HashMap::Entry.or_insert:若Entry不存在则保存，存在则不保存
            - 读取：
                a.HashMap.get()=>Option 通过match控制分支来获取数据
                b.HashMap.get(key).copied().unwrap_or(default_value) 获取一个具有所有权的V实例
            - 销毁：离开作用域时被释放


*/
