// HashMap 不在 prelude 中
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    // 推断出 Key 为 String，Value 为 i32
    map.insert(String::from("Blue"), 10);
    // 覆盖掉原来的 value
    map.insert(String::from("Blue"), 30);
    map.insert(String::from("Yellow"), 20);
    let key = String::from("Hello");
    // 获取 HashMap 中的值，返回类型为 Option
    map.get(&key);

    // 遍历 HashMap
    for (key, value) in &map {
        println!("{}, {}", key, value);
    }
    // 因为 for 循环中使用的是 &map，所以此处 map 仍然有效
    map.insert(String::from("C"), 15);
    println!("{:?}", map);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 20];
    let mut map: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    map.insert(String::from("Green"), 15);

    let field_name = 3;
    let field_value = true;
    let mut map: HashMap<u8, bool> = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 仍然可以使用，因为这些类型实现了 Copy trait，其值被拷贝进了 HashMap
    println!("{}, {}", field_name, field_value);
    
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(field_name, field_value);
    // field_name 和 field_value 已失效，因为 String 未实现，HashMap 会获取其所有权
    // println!("{}, {}", field_name, field_value);

    let text = "Hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 如果 word 这个 key 存在，则返回 value，否则以 word 作为 key，0 作为 value 插入 map
        let count = map.entry(word).or_insert(0);
        // count 的类型是 &mut，即可变引用
        // 解引用
        *count += 1;
    }
    println!("{:?}", map);
}
