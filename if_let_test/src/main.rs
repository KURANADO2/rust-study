#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let coin = Coin::Penny;
    if let Coin::Penny = coin {
        println!("Lucky!, is: {:?}", coin);
    } else {
        println!("Not found")
    }
}