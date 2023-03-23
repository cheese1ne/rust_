pub mod function_;
pub mod macro_;
pub mod trait_;
pub mod type_;
pub mod unsafe_;

fn main() {
    // 裸指针
    unsafe_::simple_raw_pointer_demo();
    // 不安全的方法
    unsafe_::unsafe_mothod_demo();
    // 一个需要避开借用规则检查的demo split_at_mut
    unsafe_::unsafe_split_mut_demo();
    // 调用外部函数接口，与其他语言接入的demo
    unsafe_::unsafe_extern_c_demo();
    // 访问和修改静态常量
    unsafe_::rw_static_data_demo();
    // 运算符重载
    trait_::operator_override_demo();
    // 具有歧义的语句
    trait_::man_self_fly_demo();
    trait_::dog_noself_name_demo();
    // 通过超trait使用其他子trait的功能
    trait_::outprint_point_demo();

    // 函数指针
    function_::function_pointer_demo();
    function_::iter_function_or_closure_demo();
    function_::return_closure_demo();

    // 宏
    macro_::my_vec_macro_demo();
    macro_::my_print_macro_demo();
    // macro_::proc_macro_derive_demo();
}
