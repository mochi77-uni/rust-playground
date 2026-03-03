use std::fmt;

fn print_sep(i : i32) {
    println!("------------- {i} -------------");
}

fn get_type_name<T>(_: T) -> String {
    std::any::type_name::<T>().to_string()
}

fn literal() {
    let literal = "Literal";
    println!("literal (type: {}): {}", get_type_name(literal), literal);
}

fn concat_with_move() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = s1 + &s2;
    // s1はmoveされたためもう使えない
    // println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");
}

fn concat_with_format() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let s3 = format!("{}{}", s1, s2);

    println!("s1: {s1}");
    println!("s2: {s2}");
    println!("s3: {s3}");
}

fn utf8_string() {
    let s = String::from("こんにちは、世界！");
    println!("UTF-8 string: {s}");
    let slice = &s[0..3];
    println!("{slice}");
    // 最初の文字の「こ」は3バイト使うから、不完全なsliceを使おうとするとエラーが起きる
    // let slice = &s[0..2];
    // println!("{slice}");
    for (index, ch) in s.chars().enumerate() {
        println!("{index}: {ch}");
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0}, {1})", self.x, self.y)
    }
}

fn format_string() {
    let point = Point { x: 1, y: 2 };
    let num = 1234;
    println!("{}", point);
    println!("{:#?}", point);
    println!("{:x}", num);
    println!("{:#x}", num);
    println!("{:o}", num);
    println!("{:#o}", num);
    println!("{:b}", num);
    println!("{:#b}", num);
}

fn main() {
    let demos: &[fn()] = &[
        literal, concat_with_move, concat_with_format, utf8_string,
        format_string,
    ];
    for (i, demo) in demos.iter().enumerate() {
        print_sep((i + 1) as i32);
        demo();
    }
}
