use std::cmp::PartialOrd;

fn main() {
    struct_generic_demo();
    largest_demo();
    //结构体的生命周期注解
    struct_lifetime_demo();
}

fn largest_demo() {
    // let list = vec![Some(3), None]; //Option::Some并没有实现PartialOrd，所以编译会报错
    let list = vec![1, 2, 3, 4];
    let largest = largest(&list);
    println!("the largest num of list is {}", largest);
}

// trait bound 在编译时期限制传入参数必须具有可比较的功能
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    //获取列表中的最大值
    for item in list {
        if largest < item {
            largest = item;
        }
    }
    largest
}

fn struct_generic_demo() {
    let point1 = Point {
        x: 10,
        y: 20,
        z: "luck",
    };
    println!(
        "point.x = {}, y = {}, z= {}",
        point1.x(),
        point1.y(),
        point1.z
    );
    let point2 = Point {
        x: "i",
        y: "u",
        z: 3.14,
    };
    // println!("point.x = {}, y = {}", point2.x(), point2.y()); //Point.y()指定了泛型为i32限定时才能使用
    println!(
        "point.x = {}, y = {}, z= {}",
        point2.x(),
        point2.y,
        point2.z
    );

    //point1和point2被移动了
    let mix_point = point1.mixup(point2);
    println!(
        "mix point.x = {}, y = {}, z = {}",
        mix_point.x(),
        mix_point.y,
        mix_point.z
    );
}

//结构体中的泛型
struct Point<T, U> {
    x: T,
    y: T,
    z: U,
}

impl<T, U> Point<T, U> {
    //方法定义中的泛型,返回x的引用
    fn x(&self) -> &T {
        &self.x
    }
    //方法使用除结构体指定之外的泛型
    fn mixup<C, H>(self, other: Point<C, H>) -> Point<T, H> {
        Point {
            x: self.x,
            y: self.y,
            z: other.z,
        }
    }
}

//泛型指定限制
impl<U> Point<i32, U> {
    fn y(&self) -> &i32 {
        &self.y
    }
}

//枚举中的泛型
enum Box<T> {
    Iron(T),
    Paper(T),
}

//定义生命周期 确保返回值引用在 x 和 y 中最短生命周期结束之前 返回的引用都是有效的!
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //在不知道具体代码块执行的情况下，检查器不能确定函数引用的具体生命周期，也就是返回值的引用是否有效!
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//结构体的生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

/*
    tip:泛型 generics
        1.结构体的泛型
        2.枚举中的泛型
        3.方法中的泛型
        4.泛型的单态化：Rust在编译时根据具体使用的类型，将通用代码转换为特定代码，可以理解为泛型在编译后产生了对应的实现，因为是在编译时转化的，所以没有运行时开销
        5.结合trait，完成largest方法
        6.函数的泛型生命周期：
            在不知道具体代码块执行的情况下，检查器不能确定函数引用的具体生命周期，也就是返回值的引用是否有效，需要添加泛型生命周期参数
            泛型生命周期语法：
                a.&i32        // 引用
                b.&'a i32     // 带有显式生命周期的引用
                c.&'a mut i32 // 带有显式生命周期的可变引用
            函数签名中的生命周期注解：栗子 fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
                    定义一个生命周期注解'a，方法参数x和y以及返回值的生命周期与a'一样长，此标注用于给明确Rust检查器的检查范围，只要引用满足函数的生命周期约束条件即可，不满足的都将被借用检查器拒绝
                    生命周期注解保证函数中标注了生命周期注解的引用在方法结束前都是有效的!
        7.结构体的生命周期注解，定义方式类似于泛型，栗子：struct ImportantExcerpt<'a> { part: &'a str, }
            结构体的生命周期注解的意义：表明结构体中具有生命周期注解的**成员引用**的生命周期 比**结构体**更长

        8.生命周期省略：
            省略规则：
                a.第一条规则：编译器为每一条引用参数都分配了一个生命周期参数
                b.第二条规则：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
                c.第三条规则：如果方法有多个输入生命周期参数且其中一个是&self或者 &mut self，说明是个对象的方法，所有输出生命周期参数被赋予self的生命周期
        9.方法定义中的生命周期注解(Rust中的方法特指实现结构体定义的函数)

*/
