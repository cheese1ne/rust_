fn main() {
    simple_trait_demo();
    trait_as_param_demo();
}

fn simple_trait_demo() {
    let article = NewArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    //article trait
    let article_summarize = article.summarize();
    println!("the summarize of article is {}", article_summarize);
    println!(
        "the default summarize of article is {}",
        article.default_summarize()
    );

    let tweet = Tweet {
        username: String::from("username"),
        content: String::from("content"),
        reply: true,
        retweet: true,
    };

    let tweet_summarize = tweet.summarize();
    println!("the tweet of article is {}", tweet_summarize);
    println!(
        "the default summarize of article is {}",
        tweet.default_summarize()
    );
}

fn trait_as_param_demo() {
    let tweet = returns_summarizable();
    notify(&tweet);
    notify2(&tweet);
    // notify_mul(&tweet); //因为Tweet没有同时实现 Summary 和 Display，所以编译会报错

    let article = NewArticle {
        headline: String::from("headline"),
        location: String::from("location"),
        author: String::from("author"),
        content: String::from("content"),
    };

    notify_mul(&article);
}

//类似java中的接口
pub trait Summary {
    fn summarize(&self) -> String;

    //默认实现，wow 设计模式之模板方法
    fn default_summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize())
    }
}

pub trait Display {}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

//impl实现trait for 具体的结构体类型
impl Summary for NewArticle {
    //NewArticle 中 summarize 的内容
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//NewArticle实现了两个trait 分别是 Summary 和 Display
impl Display for NewArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    //重载Summary的默认default_summarize
    fn default_summarize(&self) -> String {
        String::from("Tweet default_summarize :D")
    }
}

//trait作为方法参数
pub fn notify(item: &impl Summary) {
    //号外号外!
    println!("Breaking news! {}", item.summarize());
}

//trait bound语法 ，泛型 + trait
pub fn notify2<T: Summary>(item: &T) {
    //号外号外!
    println!("Breaking news2! {}", item.summarize());
}

//通过 + 指定多个trait bound
pub fn notify_mul<T: Summary + Display>(item: &T) {
    //号外号外!
    println!("notify_mul! {}", item.summarize());
}

//trait作为返回值
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }

    //在当前方法中，像Java一样返回接口的不同实现是不被允许的，是因为trait在编译时期就被确定了吗?
    // if true {
    //     Tweet {
    //         username: String::from("horse_ebooks"),
    //         content: String::from("of course, as you probably already know, people"),
    //         reply: false,
    //         retweet: false,
    //     }
    // } else {
    //     NewArticle {
    //         headline: String::from("Penguins win the Stanley Cup Championship!"),
    //         location: String::from("Pittsburgh, PA, USA"),
    //         author: String::from("Iceburgh"),
    //         content: String::from(
    //             "The Pittsburgh Penguins once again are the best \
    //              hockey team in the NHL.",
    //         )
    //     }
    // }
}

/*
    tip:trait 定义共同行为，可以理解为Java中的接口
        1.定义一个trait：通过关键字trait进行定义，trait完成一些类型相同的行为的抽象设计
        2.类型实现Trait：语法 impl Trait for Type，实现作用域内要完成所有Trait中方法的具体实现(trait中默认方法除外，但可以重载(Rust中叫重载，对应Java的重写))
        3.trait作为参数：
            1.参考栗子pub fn notify(item: &impl Summary) {...}
            2.trait + 泛型结合使用 pub fn notify2<T: Summary>(item: &T) {...}
            3.通过 + 指定多个trait bound：参数需要实现多个不同的trait
            4.使用where 简化trait bound，处理函数签名过长的问题：其实就是trait bound 后置
                简化前：fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
                简化后：fn some_function<T, U>(t: &T, u: &U) -> i32 where T: Display + Clone, U: Clone + Debug, {}
        4.trait作为返回值：pub fn returns_summarizable() -> impl Summary
        5.blanket implementation:根据trait条件有条件的实现trait(优点拗口，就是给所用已经实现A的类实现B)，栗子(标准库) impl<T: Display> ToString for T { ... }

*/
