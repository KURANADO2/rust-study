use std::fs::File;
use std::io::ErrorKind;
use std::io::Error;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");
    let file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            },
        }
    };

    // 如果 Result 值是 Ok，unwrap 会返回 Ok 中的值，否则会调用 panic!
    // let f = File::open("hello2.txt").unwrap();

    // 如果 Result 值是 expect 会返回 Ok 中的值，否则会调用 panic!，与 unwrap 的区别是，expect 可以自定义错误信息
    // let f = File::open("hello2.txt").expect("Failed to open hello2.txt");

}

fn read_username_from_file() -> Result<String, Error> {
    let f = File::open("hello2.txt");
    let mut file = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, Error> {
    let mut f = File::open("hello2.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file3() -> Result<String, Error> {
    let mut s = String::new();
    let mut f = File::open("hello2.txt")?.read_to_string(&mut s)?;
    Ok(s)
}