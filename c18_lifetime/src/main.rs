fn main() {
    dangling_reference_demo();
    //生命周期引用测试demo1
    lifetime_demo1();
    //生命周期引用测试demo2
    lifetime_demo2();
    //生命周期引用测试demo3
    lifetime_demo3();
}

fn dangling_reference_demo() {
    let r;

    {
        let x = 5;
        r = &x; //r获取x的引用
    } //作用域关闭后，x被drop，r指向一个空内存

    //由于外部作用域仍使用到了r，报错信息：`x` does not live long enough
    // println!("r: {}", r); //r引用了一个生命周期较短的x，此为无效的引用
}

//定义生命周期'a 保证x和y的生命周期 > 'a 返回的引用都是有效的! 也就是需要保证 返回值的引用的生命周期 = 参数中最短的生命周期 成立
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //在不知道具体代码块执行的情况下，检查器不能确定函数引用的具体生命周期，也就是返回值的引用是否有效!
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_demo1() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    } //此处 string2 和 result 失效
} // 此处 string1 失效

fn lifetime_demo2() {
    //函数的生命周期是返回较短的那个
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        // result = longest(string1.as_str(), string2.as_str());
        result = longest(string1.as_str(), string1.as_str()); //为保证编译通过，使用生命周期更长的string1，测试生命周期使用上一行
    } //string2的生命周期为内部作用域，string2的生命周期 < result的生命周期 Rust检查器不通过
    println!("The longest string is {}", result); //检查器保证返回值的引用到此处必须是有效的，否则编译不通过
} // 此处 string1 result 失效

//结构体的生命周期函数
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//结构体方法的生命周期
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn lifetime_demo3() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence, //也就是保证part的引用的生命周期大于ImportantExcerpt实例的生命周期
    };
}

/*
    tip:生命周期，用于确保引用有效
        定义：引用保持有效的作用域就是引用的生命周期
        作用：避免dangling references 也就是悬垂引用
        借用检查器：
            Rust编译器有一个借用检查器borrow checker 它比较作用域来确保所有的借用都是有效的
            有效的引用：数据比引用有更长的生命周期
        函数签名中的生命周期注解：栗子 fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { ... }
            生命周期注解添加在返回值和参数上
        结构体定义中的生命周期注解：栗子：struct ImportantExcerpt<'a> { part: &'a str, }
            结构体的生命周期注解的意义：表明结构体中具有生命周期注解的**成员引用**的生命周期 比**结构体**更长
            一个存放引用的结构体，其定义需要生命周期注解!
        生命周期省略：
            省略规则：
                a.第一条规则：编译器为每一条引用参数都分配了一个生命周期参数
                b.第二条规则：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
                c.第三条规则：如果方法有多个输入生命周期参数且其中一个是&self或者 &mut self，说明是个对象的方法，所有输出生命周期参数被赋予self的生命周期
        方法定义中的生命周期注解(Rust中的方法特指实现结构体定义的函数)

*/
