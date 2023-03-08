fn main() {
    println!("Hello, world!");
}

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    //迭代器需要可变，for循环通过获取所有权在后台获取可变引用
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);
}

#[test]
fn iterator_map() {
    let v = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", v2); // 输出：[2, 4, 6, 8, 10]
}
