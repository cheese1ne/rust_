use crate::garden::vegetables::Asparagus; //crate代表根crate, 此处为绝对路径

pub mod garden; //mod garden声明模块

fn main() {
    //创建一个garden模块下vegetables子模块中Asparagus的结构体实例
    let plant = Asparagus {};
    println!("the Asparagus is {:?}", plant);
}

/*

    root crate: 在库crate，根crate是src/lib.rs；在二进制crate，根crate是src/main.rs
    package:
        1.一个package包含一个Cargo.toml
        2.一个package最多包含一个lib crate 库crate
        3.一个package可以包含多个bin crate 二进制crate
        4.一个package最少包含一个crate
    crate:
        1.src/main.rs包代表他有一个名为"crate_"的二进制crate
        2.如果一个包同时包含src/main.rs和src/lib.rs，则这个包拥有两个二进制crate
        3.src/bin目录下每个文件都会被编译成为一个独立的二进制crate

    modules：
        1.从crate根节点开始：编译时候从根crate寻找需要被编译的代码，层层递进，
        2.声明模块：mod garden
            a. 在根crate(栗子是在src/main.rs)中，编译器会在以下路径寻找模块代码
                - 内联：定义在声明模块最后，以代码块的形式引入
                - 在文件 src/garden.rs
                - 在文件 src/garden/mod.rs
        3.声明子模块：在src/garden.rs中定义mod vegetables
            a.内联：定义在声明模块最后，以代码块的形式引入
            b.在以父模块命名的目录中寻找子模块：src/garden/vegetables.rs
            c.在以父模块命名的目录中寻找子模块：src/garden/vegetables/mod.rs
        4.模块中的代码路径：如果模块是crate的一部分，在访问权限允许的情况下，可以从crate通过代码路径引用该模块的代码，栗子: crate::garden::vegetables::Asparagus 引用vegetables子模块的Asparagus结构体
        5.私有vs公用：一个模块的代码默认对父模块私有，使用pub关键字声明模块公用以及模块内部的成员公用
        6.use关键字：在一个作用域内，使用use关键字减少长路径的重复，类似Java中的import
            - use的作用域不能传递到作用域内部模块中
            - 通过use进行引用模块时，引用到模块(除同名模块外)，引用结构体、枚举或其他项时，引入完整路径
            - 引用相同名称但不同父模块的项到作用域时，作用域内部使用时通过父模块::项 的相对路径，也可以通过 as 关键字来给同名项取别名
            - pub use: 通过pub修饰的use，导入的名称不仅可以在当前作用域内使用，在引用此作用域的其他作用域也可以使用
            - 使用外部包：
                a.在Cargo.toml中配置外部包依赖及版本，栗子：rand = "0.8.5"，标准库则无需配置
                b.要使用rand的作用域内引入rand中的具体内容，栗子： use rand::Rng; use std::collections::HashMap;
            - 嵌套路径来消除重复的use行：
                - 带有相同前缀：如 use std::{cmp::Ordering, io}， 代替分别引用stdx下cmp::Ordering和io的内容;
                - 一个是另一个的子路径可以用self代替：use std::io::{self, Write} 替代use std::io; use std::io::Write;
                - 通过*引入所有公有项：如 use std::collections::*;




    tip: src/main.rs 是一个与包同名的crate的crate根，在当前项目"crate_"中

*/
