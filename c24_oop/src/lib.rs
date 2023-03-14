// 定义一个带有绘画方法的trait
pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub lable: String,
}

// 一个实现Draw trait的 Button结构体
impl Draw for Button {
    fn draw(&self) {}
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

// 定义一块屏幕
pub struct Screen {
    // 对外公开成员, 一个Draw trait的包装列表
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 面相对象的行为设计模式-状态模式
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    fn approve(self: Box<Self>) -> Box<dyn State>;

    //生命周期省略规则不符合，这里需要手动指定，返回值的生命周期满足与状态实例、博客实例的生命周期
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        ""
    }
}

// 状态为推送
struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// 状态为审核中
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 状态为草稿
pub struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 返回一个
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// 定义一个博文
pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    // 需要改变post实例
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        // if let 赋值 Option为Some时
        if let Some(state) = self.state.take() {
            // 通过state抽象来完成赋值，消费state
            self.state = Some(state.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }

    pub fn content(&self) -> &str {
        // 获取一个state的引用: Option<&Box<dyn State>>
        // let state_ref = self.state.as_ref();
        self.state.as_ref().unwrap().content(self)
    }
}

/*
  储存多种不同类型使用Trait而不是泛型
  原理：Rust中泛型在编译的单态化处理，单态化的代码执行静态分发，trait对象使用动态分发，
    泛型参数一次只能替代一个具体类型
    trait对象允许在运行时替代多种具体类型


    trait对象类型安全的规则：
      1.返回值不是Self
      2.没有泛型类型的参数

  总结：使用trait而不是泛型完成多态


*/
