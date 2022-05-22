fn main() {
    // 字符串字面值
    let data = "initial contents";
    // String 类型
    let s = data.to_string();
    // 从字符串字面值创建 String
    let s = String::from("initial contents");

    let mut s1 = String::from("Hello");
    let s2 = " World";
    s1.push_str(s2);
    println!("{}", s1);
    // s2 仍然可以使用，因为 push_str 接收的参数是 &str，并没有获取 s2 的所有权
    println!("{}", s2);
    
    // push 追加一个字符
    s1.push('!');
    println!("{}", s1);

    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    // + 运算符使用了 add 函数，add 函数的签名是 fn add(self, s: &str) -> String {
    // &s2 的类型是 &String，而 add 函数要求第二个参数为 &str，即字符串前面值，这里发生了 Deref 强制转换，所以可以正常传入
    let s3 = s1 + &s2;
    // s1 已失效，因为 self 参数没有 & 标识，所以 add 函数获取了 s1 的所有权
    // println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 略显啰嗦
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // 相比较 + 运算符，format 更简洁
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // 遍历字符串中的字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // 遍历字符串中的字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
