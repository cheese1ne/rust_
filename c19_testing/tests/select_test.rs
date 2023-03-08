use common::Connection; //使用Connection的Trait

mod common; //声明common模块

#[test]
fn select_one_test() {
    // tests/common/mod.rs中的函数不会被输出common中的方法
    let connection = common::get_connection();
    let list = connection.query(1);
    println!("the list is {:?}", list);
}
