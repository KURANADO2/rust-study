#[derive(Debug)]
struct Rectangel {
    width: u32,
    height: u32,
}

impl Rectangel {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // 判断 width 字段是否大于零的方法
    fn width(&self) -> bool {
        self.width > 0
    }

    fn square(size: u32) -> Rectangel {
        Rectangel {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangel) -> bool {
        self.width() > other.width() && self.height > other.height
    }
}

fn main() {
    let width = 30;
    let height = 50;

    println!("The area of rectangle is {} square pixels.", area(width, height));
    
    let rect = (30, 50);
    println!("The area of rectangle is {} square pixels.", area2(rect));
    
    let rect = Rectangel {
        width: 30,
        height: 50,
    };
    // 获得所有权
    // println!("The area of rectangle is {} square pixels.", area3(rect));
    // 报错，因为 rect 已不具备所有权
    // println!("The widht of rectangle is {}.", rect.width);

    // 借用
    println!("The area of rectangle is {} square pixels.", area4(&rect));
    // 可以继续使用 rect
    println!("The widht of rectangle is {}.", rect.width);
    // 需要在结构体上增加外部属性 #[derive(Debug)]
    println!("Rect is {:?}.", rect);
    println!("Rect is {:#?}.", rect);
    
    // 使用方法
    println!("The area of rectangle is {} square pixels.", rect.area());
    println!("The rectangle has or not: {}, a nonzero width; it is {}", rect.width(), rect.width);

    let square = Rectangel::square(50);
    println!("Can rect hold square: {}", rect.can_hold(&square));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rect: Rectangel) -> u32 {
    rect.width * rect.height
}

fn area4(rect: &Rectangel) -> u32 {
    rect.width * rect.height
}