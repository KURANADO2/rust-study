enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// 使用 enum 代替 struct
enum IpAddrEnum {
    V4(String),
    V6(String),
}

// 使用 enum 代替 struct
enum IpAddrEnum2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 包含多种类型（也可以分成多个 struct 来定义，但定义成枚举，就可以轻易的定义一个能够处理这些不同类型的结构体的函数，因为枚举是单独一个类型）
enum Message {
    // 没有关联任务数据
    Quit,
    // 类似结构体包含命名字段
    Move {x: i32, y: i32},
    // 包含单独一个 String
    Write(String),
    // 包含 3 个 i32
    ChangeColor(i32, i32, i32),
}

// 为枚举定义方法
impl Message {
    fn call(&self) {
        // ...
    }
}

// // 类单元结构体
// struct Quit;
// // 普通结构体
// struct Move {
//     x: i32,
//     y: i32,
// }
// // 元组结构体
// struct Write(String);
// // 元组结构体
// struct ChangeColor(i32, i32, i32);

fn main() {
    // 创建两个 IpAddrKind 的实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // enum + struct
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // 使用 enum 代替 struct
    let home = IpAddrEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrEnum::V6(String::from("::1"));

    let home = IpAddrEnum2::V4(127, 0, 0, 1);
    let loopback = IpAddrEnum2::V6(String::from("::1"));

    let messageColor = Message::ChangeColor(255, 0, 0);
    messageColor.call()
}
