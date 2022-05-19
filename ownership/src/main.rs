fn main() {
    // 两个 5 都被放入栈中
    // i32 类型已经实现了 Copy trait
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    let s1 = String::from("Hello");
    let s2 = s1;
    // s1 已经被移动给了 s2，此时只有 s2 有效，s1 不再有效，下面这行代码使用了 s1，所以将会报错
    // println!("{}, {}", s1, s2);

    let s3 = String::from("Hello");
    // 复制堆上的数据
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    let s5 = String::from("World");
    // 将 s5 的所有权交给 takes_ownership 函数
    takes_ownership(s5);
    // 此处 s5 不再有效，故下面报错
    // println!("{}", s5);

    let s6 = 5;
    makes_copy(s6);
    // i32 实现了 Copy trait，故下面 s6 仍然有效
    println!("s6: {}", s6);

    let s7 = gives_ownership();
    let s8 = String::from("hello");
    // 将 s8 的所有权交给 takes_and_gives_back
    let s9 = takes_and_gives_back(s8);

    let s10 = String::from("nihao");
    let (s11, len) = calculate_length(s10);
    println!("{}, {}", s11, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    // 返回 some_string，将所有权移动给调用者
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
