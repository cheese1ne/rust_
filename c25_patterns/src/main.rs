fn main() {
    println!("Hello, world!");
}

/*
    tip:
    模式和模式匹配
        1.match分支: 表达式必须是穷尽的，_ 是一个特殊的模式，他可以匹配所有的情况，在希望忽略值的场景使用
            match VALUE {
                PATTERN => EXPRESSION,
                PATTERN => EXPRESSION,
                PATTERN => EXPRESSION,
            }
        2.if let表达式: 只关心一种情况的match的简写，也可以组合else if、else if let、else表达式
            if let PATTERN = VALUE1 {

            } else if VALUE2 {

            } else if let PATTERN = VALUE3 {

            } else {

            }
        3.while let条件循环: 只要模式匹配就一直进行while循环
            while let PATTERN = VALUE {
                //循环体
            }
        4.for循环
            for (index, value) in ITER {
                //循环体
            }
        5.let语句: let语句就是一个最简单的模式
            let PATTERN = EXPRESSION;
        6.fn参数: 函数中的每一个参数就是一个模式，体现在调用函数时的赋值行为
            fn METHOD(PATTERN1,...,PATTERNX){}
            METHOD(EXPRESSION1,...,EXPRESSIONX);

    可反驳性: 能匹配任何传递的可能值的模式被称为不可反驳的，对某些可能的值进行匹配失败的模式称为可反驳的
        1.函数参数、let语句、for循环只能接受不可反驳的模式
        2.if let 和 while let表达式被限制为只能接受可反驳的模式

    模式语法:
        1.匹配字面值
        2.匹配命名变量
        3.通过 | 语法匹配多个模式(代表或)
        4.通过..=匹配值的范围
        5.解构: 可以结构结构体、解构枚举、元组
        6.忽略模式中的值:
            _: 使用_忽略整个值 _、使用嵌套的_忽略部分值Some(_)，通过在名字前以一个_开头来忽略未使用的变量_x
            ..: 使用..忽略结构体中剩余的值X {a, .. } X { a, .., z }、忽略元组中的剩余的值 (a,..) (a,..,z)
        7.匹配守卫: 指定一个match分支后额外的if条件
            match num {
                Some(x) if x % 2 == 0 => EXPRESSION,
                Some(x) => EXPRESSION,
                None => EXPRESSION,
            }
        8.@绑定: 创建一个存放值的变量的同时测试其值是否匹配模式
            match msg {
                Message::Hello { id: id_variable @ 3..=7} => EXPRESSION, //分支的表达式中就可以使用id_variable这个变量
                Message::Hello { id } => EXPRESSION,
            }
*/
