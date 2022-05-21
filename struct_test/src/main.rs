struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 类单元结构体
struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("kuranado"),
        email: String::from("kuranado@163.com"),
        sign_in_count: 1,
    };
    println!("username: {}", user1.username);
    // 整个实例是不可变的，所以下面这行代码会报错
    // user1.email = String::from("anotheremail@163.com");

    let mut user2 = build_user(String::from("kuranado"), String::from("kuranado@163.com"));
    user2.email = String::from("anotheremail@163.com");
    println!("another email: {}", user2.email);
    
    let user2 = build_user2(String::from("kuranado"), String::from("kuranado@163.com"));
    println!("email: {}", user2.email);

    let user2 = User {
        email: String::from("123@163.com"),
        // 其他字段值全部取自 user1
        ..user1
    };
    println!("username: {}, email: {}", user2.username, user2.email);

    // 实例化元组结构体
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    println!("{}", white.2)
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }
}

fn build_user2(username: String, email: String) -> User {
    User {
        active: true,
        // 简略写法
        username,
        email,
        sign_in_count: 1
    }
}
