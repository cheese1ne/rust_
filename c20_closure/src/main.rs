use c20_closure::{self, Inventory, ShirtColor};

fn main() {
    //一个简单的闭包demo
    // closures_::simple_closure_demo();

    //一个类似缺省的使用方式
    // inventory_demo();

    //所有权
    // closures_::immutabl_reference_borrow_closure_demo();
    // closures_::mutable_reference_borrow_closure_demo();

    //闭包与线程
    // closures_::concurrent_closure_demo();

    //闭包的类型
    // closures_::option_unwrap_or_else();
    c20_closure::slice_sort_by_key();
}

pub fn inventory_demo() {
    //初始化一个库存实例
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    //设置一个缺省
    let expect_color = None;
    let get_color = store.giveaway(expect_color);
    println!(
        "The user1 with preference {:?} gets {:?}",
        expect_color, get_color
    );

    let expect_color = Some(ShirtColor::Red);
    let get_color = store.giveaway(expect_color);
    println!(
        "The user2 with preference {:?} gets {:?}",
        expect_color, get_color
    );
}
