fn main() {
    let s1 = String::from("hello");
    // &s1 只借用 s1，并不具备 s1 的所有权
    let len = calculate_length(&s1);
    // s1 仍然有效
    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("hello");
    // 可变引用
    change(&mut s2);
    println!("{}", &s2);
    println!("{}", s2);

    let mut s3 = String::from("hello");
    // 针对同一数据同时创建多个可变引用将会报错
    // let r1 = &mut s3;
    // let r2 = &mut s3;
    // println!("{}, {}", r1, r2);
    {
        let _r1 = &mut s3;
    }
    // 此处 _r1 已经离开作用域，故针对同一数据，没有同时存在多个可变引用
    let r2 = &mut s3;
    println!("{}", r2);
    
    let mut s4 = String::from("hello");
    // 不可变引用
    let r3 = &s4;
    // 可变引用，针对同一数据同时创建可变引用与不可变引用将会报错
    let r4 = &mut s4;
    // 针对同一数据同时创建可变引用与不可变引用将会报错
    // println!("{}, {}, {}", s4, r3, r4);

    // let reference_to_nothing = dangle();
    let s5 = no_dangle();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" world");
}

// fn dangle() -> &String {
    // let s = String::from("Hello");
    // &s
    // 此函数结束后，&s 这个引用将会失效，造成悬垂引用
// }

fn no_dangle() -> String {
    let s = String::from("Hello");
    s
}