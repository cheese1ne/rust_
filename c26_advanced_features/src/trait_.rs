use std::fmt::{self, Debug};
use std::ops::Add;

pub trait Iterator {
    type Item; //定义一个关联类型
    fn next(&mut self) -> Option<Self::Item>;
}

// // 设置一个默认类型参数，如果实现trait时不指定Rhs的具体类型，Rhs将是默认的Self类型
// pub trait Add<Rhs = Self> {
//     type Output;

//     fn add(self, rhs: Rhs) -> Self::Output;
// }

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// std::ops::Add 是 + 运算符的 trait
impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Meters(u32);
#[derive(Debug, Copy, Clone, PartialEq)]
struct Millimeters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimeters(self.0 + rhs.0 * 1000)
    }
}

pub fn operator_override_demo() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    assert_eq!(Millimeters(1000) + Meters(1), Millimeters(2000));
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Man;

impl Pilot for Man {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Man {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Man {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

pub fn man_self_fly_demo() {
    let man = Man {};
    man.fly();
    Pilot::fly(&man);
    Wizard::fly(&man);
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

pub fn dog_noself_name_demo() {
    // 如果结构体实现的trait中有一个与自身关联函数重名的方法，默认调用关联函数
    println!("A baby dog is called a {}", Dog::baby_name()); //Spot

    // 手动指定trait来消除歧义 通过as关键字
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); //puppy
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //write宏
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

pub fn outprint_point_demo() {
    let point = Point { x: 10, y: 20 };

    // point.fmt(&mut format);
}

// 通过newtype来绕开孤儿规则，让Vec实现fmt::Display的方法
struct Wrapper(Vec<String>);

// 我们无法直接让Vec实现fmt::Display这个trait，因为结构体和trait都不是本地的（违反孤儿规则）
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

/*
  tip:高级trait
    1.关联类型:
      Rust中不使用泛型而使用关联类型来确定类型的原因：一个类型仅能实现一次这个trait，trait实现必须提供一个类型来替代关联类型占位符
    2.泛型类型参数和运算符重载:
      如果实现trait时不指定Rhs的具体类型，Rhs将是默认的Self类型。
      std::ops::Add 是 + 运算符的 trait
    3.完全限定语法与消歧义
      完全限定语法: <Type as Trait>::function(receiver_if_method, next_arg, ...);
      结构体和实现的trait与自身关联函数有重名的方法时，默认调用自身的关联函数，想要具体调用trait的函数时
        a.若方法中包含self参数时，通过具体的trait::method(&self) 来完成调用
        b.若方法中不包含self参数时，通过<struct as trait>::method() 来完成调用 ,此为完全限定语法
    4.超trait的一个子trait使用另一个子trait的功能

    5.newtype模式用在外部类型上实现外部trait
      孤儿规则：如果trait或类型对于当前crate是本地的话就可以实现该trait，结构体或trait必须有一个是本地的才能实现trait，通过newtype简单包装绕开这一规则







*/
