use std::slice;

pub fn simple_raw_pointer_demo() {
    let mut num = 5;
    // 可以在安全代码中
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 解引用需要一个unsafe块
    // println!("r1 is: {}", *r1); // need unsafe function or block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn unsafe_mothod_demo() {
    //调用不安全函数或方法需要一个unsafe块
    // dangerous();
    unsafe {
        dangerous();
    }
}

unsafe fn dangerous() {
    //在一个不安全的函数中进行其他的不安全操作不需要增加额外的unsafe块
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("dangerous r1 is: {}", *r1);
    println!("dangerous r2 is: {}", *r2);
}

pub fn unsafe_split_mut_demo() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);
    assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 7]);
    assert_eq!(b, &mut [4, 5, 6]);
}

pub fn unsafe_extern_c_demo() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// "C"部分定义了外部函数所使用的二进制接口，
extern "C" {
    fn abs(input: i32) -> i32;
}

// 禁止mangle函数名
#[no_mangle]
// 定义一个可以从C语言访问Rust的接口
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

pub fn rw_static_data_demo() {
    let inc = 2;

    // COUNTER += inc;
    // println!("COUNTER : {}", COUNTER); //访问和修改静态常量都是不安全的
    unsafe {
        COUNTER += inc;
        println!("COUNTER : {}", COUNTER);
    }
}

// 实现一个不安全的trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}

/**
 * 这个函数的功能是传入一个slice以及一个切割位，返回一个元组，包含被分割的两部分
 */
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // 返回一个裸指针
    assert!(len >= mid);
    // 无法在这里借用两次
    // (&mut values[..mid], &mut values[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // 此函数需要一个裸指针
            slice::from_raw_parts_mut(ptr.add(mid), len - mid), //通过add方法获取一个从mid开始的裸指针
        )
    }
}
/*
  tip: unsafe 不安全Rust
    1.裸指针:
      概念: raw pointer，是一种可以直接访问内存地址的指针类型，裸指针没有借用规则的限制，使用时要小心内存泄漏问题
      类型: 不可变裸指针 *const T , 可变裸指针 *mut T
      创建: 参考simple_raw_pointer_demo()
      解引用: 解引用裸指针需要在unsafe块中完成
    2.unsafe块:
      1.块: 通过unsafe修饰的作用域
      2.函数: 通过unsafe修饰的函数或方法
    3.extern函数: 用于创建使用外部函数接口
      使用外部函数接口:
          extern "C" {
              fn abs(input: i32) -> i32;
          }
      创建外部接口:
    4.访问和修改可变静态变量需要在unsafe块中完成
      定义一个静态常量: static HELLO_WORLD: &str = "Hello, world!";
    5.不安全的trait:
      定义一个不安全的trait，通过unsafe修饰 unsafe trait Foo {} unsafe impl Foo for i32 {}
      当trait中至少有一个方法包含编译器无法验证不变式时，例如线程中传递的消息未使用Send trait标记时
    6.union联合体
      联合体主要用于和C中的联合体进行交互，访问联合体中的字段是不安全的，因为Rust无法保证当前储存在联合体实例中的数据类型

*/
