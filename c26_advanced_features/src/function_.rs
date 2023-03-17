pub fn function_pointer_demo() {
    let x = do_twice(add_one, 5);
    println!("the add twice value is {}", x)
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, args: i32) -> i32 {
    f(args) + f(args)
}

// 函数 ｜ 闭包
// Java中的方法复用和stream中的表达式(闭包)
pub fn iter_function_or_closure_demo() {
    let list_of_numbers = vec![1, 2, 3];
    let closure_list = closure_demo(&list_of_numbers);
    let function_reuse_list = function_reuse_demo(&list_of_numbers);
    println!("the value of closure list is : {:?}", closure_list);
    println!(
        "the value of function reuse list is : {:?}",
        function_reuse_list
    );
    assert_eq!(closure_list, function_reuse_list);
}

fn closure_demo(num_list: &[i32]) -> Vec<String> {
    let list_of_strings: Vec<String> = num_list.iter().map(|i| i.to_string()).collect();
    list_of_strings
}

fn function_reuse_demo(num_list: &[i32]) -> Vec<String> {
    let list_of_strings: Vec<String> = num_list.iter().map(ToString::to_string).collect();
    list_of_strings
}

pub fn return_closure_demo() {
    let closure = returns_closure();
    let result = closure(5);
    println!("the return closure calculate result is {}", result);
}

// 返回一个闭包
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
/*
    tip: 高级函数
      函数指针: 指向函数的指针，函数指针可以作为参数传递给其他函数，也可以作为返回值
      返回闭包: 闭包所需的空间未知，返回一个指向闭包的指针即可

*/
