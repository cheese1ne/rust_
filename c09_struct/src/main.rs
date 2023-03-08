fn main() {
    //创建我的第一个rust实例
    create_obj_demo();
    //元组结构体
    create_tuple_struct_demo();
    //没有任何字段的类单元结构体
    create_unit_like_struct();
    //一个使用结构体的demo
    use_struct_function_demo();
}

fn create_obj_demo() {
    // 手动创建一个对象
    let mut user = User {
        name: String::from("sobann"),
        age: 28,
        email: String::from("sobanndot@163.com"),
        active: true,
    };
    let user_info = to_string(&user);
    println!("the init user is {user_info}");

    //修改对象中的属性
    user.name = String::from("cheese1");
    user.age = 27;
    let user_info = to_string(&user);
    println!("the update user is {user_info}");

    //指定构造函数来创建
    let user_ = build_user("test".to_string(), 18, "123@qq.com".to_string(), true);
    println!("the build user is {}", to_string(&user_));
    //修改邮箱 对象展开表达式 参考js es5语法
    let update_user_ = update_email(user_, String::from("456@qq.com")); //此方法会移动user_所有权
    println!("the update email user is {}", to_string(&update_user_));
    // println!("the build user is {}", to_string(&user_)); //后续无法使用无所有权的user_
}

/**
 * my first rust struct
 */
struct User {
    name: String,
    age: u16,
    email: String,
    active: bool,
}

/**
 * 一个user的全参构造方法
 * 若参数名和参数字段相同，则类似es6中的简写语法，省略 name:name, 为 name,
 */
fn build_user(name: String, age: u16, email: String, active: bool) -> User {
    User {
        name,
        age,
        email,
        active,
    }
}

// 修改邮箱，此处的语法类似es5中的对象展开，对象展开必须放在最后代表其余字段从展开的对象中获取
fn update_email(user: User, email: String) -> User {
    User { email, ..user }
} //user 在此处会被drop掉

fn to_string(user: &User) -> String {
    let mut user_info = String::from("");
    user_info.push_str("name:");
    user_info.push_str(&user.name);
    user_info.push_str(";");
    user_info.push_str("age:");
    user_info.push_str(&user.age.to_string());
    user_info.push_str(";");
    user_info.push_str("email:");
    user_info.push_str(&user.email);
    user_info.push_str(";");
    user_info.push_str("active:");
    user_info.push_str(&user.active.to_string());
    user_info.push_str(";");
    user_info
}

fn create_tuple_struct_demo() {
    let black = Color(0, 0, 0);
    println!(
        "the value of color is {}, {}, {}",
        black.0, black.1, black.2
    );
}

//元组结构体
struct Color(i32, i32, i32);

fn create_unit_like_struct() -> AlwaysEqual {
    let always_equal = AlwaysEqual;
    always_equal
}

//类单元结构体
struct AlwaysEqual;

//结构体与所有权，成员使用引用属性来创建结构体, 必须要设置生命周期标识符
struct Game<'a> {
    username: &'a str,
    email: &'a str,
    active: bool,
}

fn use_struct_function_demo() {
    let rectangle = Rectangle {
        width: 4,
        height: 5,
    };
    let area = area(&rectangle);
    println!("the Rectangle info is {:?}", rectangle);
    println!("the area of the rectangle is {}", area);
}

//长方形的结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//通过impl来定义结构体的方法，内部的函数被称为关联函数
impl Rectangle {
    //创建一个正方形
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    //结构体Rectangle 具有计算面积和周长的方法
    fn area(&self) -> u32 {
        self.height * self.width
    }

    //周长
    fn perimeter(self: &Self) -> u32 {
        (self.height + self.width) << 1
    }

    //参数是否合法
    fn valid(&self) -> bool {
        self.height > 0 && self.width > 0
    }

    //是否包括
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/**
 * 计算长方形的面积
 *
 * @param rectangle 一个长方形实例的引用
 * @return 面积
 */
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*
   tip:结构体，结构体的定义和实例，创建对象和结构、对象展开与js es6的语法规范类似
    1.结构体类型：
        a.对象型：具有成员属性
        b.元组结构： 使用元组形式
        c.类单元结构体：没有任何字段的类单元结构体

    other:
        a.对象型结构体中设置引用类型的所有权处理，在后续的生命周期章节中学习
        b.调试结构体时
            - 需要添加#[derive(Debug)]
            - 宏中添加:?打印调试的变量，栗子：println!("the Rectangle info is {:?}", rectangle);


    tip:结构体的方法
        1.通过impl关键字定义结构体的方法，结构体的方法被称为关联函数
        2.关联方法如果类型是当前结构体的类型，可以用Self进行代替
            - Self 表示当前结构体的类型
            - &self 实际上是 self: &Self 的缩写


    other:位移运算与java类似
*/
