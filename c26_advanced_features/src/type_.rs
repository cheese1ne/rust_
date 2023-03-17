// 类型别名，用于减少很长很长很长的类型在多个地方的签名
type Kilometers = i32;
type Thunk = Box<dyn Fn() + Send + 'static>;

fn takes_long_type(f: Thunk) {}

// fn returns_long_type() -> Thunk {}

// newtype结构体
struct Meters(u32);

// never type
fn bar() -> ! {
    loop {
        print!("and ever ");
        // if true {
        // break; // 用loop来表达never type 时候循环不能终止
        // }
    }
} // bar方法从不返回

fn generic<T>(t: T) {}
// 此方法等同于
fn generic_default<T: Sized>(t: T) {}

// 手动放宽泛型是否在编译时已知类型大小
fn generic_manual<T: ?Sized>(t: &T) {
    // t的类型由 T 改为 &T: 因为 t 类型可能不是Sized，所以需要将其置于某种指针之后 (引用也是一种指针哦)
}

/*
    tip: 高级类型
      newtype: 元组结构体，可以保证类型安全和抽象设计
      类型别名: 通过type关键字来命名
      nevertype: ! 它被称为empty type 在函数从不返回值的时候充当返回值
        match 控制流中的 continue的值是 !，他不真正返回一个值，而是把控制权交回给上层循环
        Option<T>的unwrap函数产生一个值或者panic，不返回值而是中断程序
        loop 循环不终止的情况 表达式为 !
      unsized types: 动态大小类型，运行时才知道大小的类型，Rust中使用动态大小类型的方式通过将动态类型置于某种指针之后 (引用也是一种指针)
        str: str是一个未知长度的类型，&str保存的是str的地址和长度(slice保存的是开始的位置和slice的长度)
        trait: trait作为一个动态类型的参数或返回值的时候，将他们放于指针之后，如 Box<dyn Trait> Rc<dyn Trait>
      Sized trait: Rust提供的一个决定类型大小是否在编译时可知的trait，Rust隐式的为每一个泛型函数增加了Size bound
        特殊语法 ?Sized 覆盖泛型类型必须在编译时拥有固定大小的默认规则
*/
