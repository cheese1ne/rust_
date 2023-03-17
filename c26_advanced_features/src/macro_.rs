use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};
#[macro_export]
macro_rules! my_vec {
    // $变量是一个宏变量， $代表宏变量， 逗号*代表匹配 0+的模式，每次匹配成功，都会将
    ($($x: expr), *) => {
        {
            let mut tem_vc = Vec::new();
            // $()* 会匹配任意次数，匹配成功会执行()内的方法
            $(
                // 每一次匹配成功都会将宏变量push进vec中
                tem_vc.push($x);
            )*
            tem_vc
        }
    };
}

#[macro_export]
macro_rules! my_println {
    ($expr:expr) => {{
        println!("the value of expr is {:?}", $expr);
    }};
}

pub fn my_vec_macro_demo() {
    let temp_vec = my_vec![1, 2, 3];
    println!("the macro of diy vec is {:?}", temp_vec);
}

pub fn my_print_macro_demo() {
    let str = String::from("C4");
    my_println!(str)
}

// 过程宏 自定义宏
#[proc_macro_derive(HasProperty, attributes(has))]
pub fn derive_has_property(input: TokenStream) -> TokenStream {
    // 将输入解析为 AST
    let input = parse_macro_input!(input as DeriveInput);

    // 提取结构体的名称和字段
    let name = input.ident;
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Can only derive HasProperty on structs."),
    };

    // 生成一个用于检查结构体字段的闭包
    let check_fields = fields.iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let has_attr = field.attrs.iter().any(|attr| attr.path.is_ident("has"));
        quote! {
            if !#has_attr {
                return false;
            }
        }
    });

    // 生成代码以实现 HasProperty trait
    let output = quote! {
        impl HasProperty for #name {
            fn has_property(&self) -> bool {
                #(#check_fields)*
                true
            }
        }
    };

    // 返回生成的代码
    TokenStream::from(output)
}

// 定义 HasProperty trait
pub trait HasProperty {
    fn has_property(&self) -> bool;
}

// 定义一个结构体和相应的属性
#[derive(HasProperty)]
struct Person {
    #[has]
    name: String,
    // #[has]
    age: u32,
}

pub fn proc_macro_derive_demo() {
    let person = Person {
        name: "Alice".to_string(),
        age: 25,
    };
    assert!(person.has_property());
}

/*

    tip: 宏 是一种元编程工具，它可以在编译时生成代码，接受任意数量和类型的参数，并使用Rust语法来生成代码
        声明宏: 使用macro_rules!关键字定义，匹配对应模式然后以另外一部分代码替换当前代码，栗子 my_vec、my_println 用于编译时 Rust会将宏调用 转换为 Rust代码
        过程宏: 过程宏接收Rust代码作为输入，在这些代码上进行操作，然后产生并输出另外一些代码，分为以下三种类型
            自定义宏: 自定义派生宏可以通过#[derive]属性来使用，它们可以用来自动为结构体或枚举类型实现trait，例如HasProperty
            类属性宏: 创建新的属性，例如web应用框架 #[route(GET, "/")]
            类函数宏: 接受未知数量参数的宏，如 sql!(SELECT * FROM foo WHERE id = 1);



*/
