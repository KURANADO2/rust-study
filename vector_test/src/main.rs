enum SpreadSheetCell {
    A(i32),
    B(f64),
    C(String),
}

fn main() {
    // 手动指定类型
    let v: Vec<u8> = vec![1, 2, 3];

    // 自动推导出 v 的类型是 Vec<i32>
    let v = vec![1, 2, 3];
    let mut v = Vec::new();
    // 从放入第一个 i32 值开始，推导出 v 的类型是 Vec<i32>
    v.push(5);
    v.push(6);
    v.push(7);
    // Vector 只能存储相同类型，以便知道储存每个元素到底需要多少内存
    // v.push(false);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // 遍历 Vector
    // 获取每一个元素的不可变引用
    for i in &v {
        println!("{}", i);
    }

    // 获取每一个元素的可变引用
    for i in &mut v {
        // 解引用
        *i += 50;
        println!("{}", i);
    }

    let v = vec![SpreadSheetCell::A(3), SpreadSheetCell::B(1.0), SpreadSheetCell::C(String::from("Hello"))];
}
