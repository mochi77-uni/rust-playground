fn print_sep(i : i32) {
    println!("------------- {i} -------------");
}

fn basic_vec() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    while let Some(value) = v.pop() {
        println!("{}", value);
    }

    // 存在しない、パニックする
    // println!("{}", v[10]);

    match v.get(10) {
        None => println!("Index out of bounds"),
        Some(num) => println!("{}", num)
    }
}

fn vec_with_macro() {
    let mut v = Some(vec![1, 2, 3]);

    if let Some(ref vec) = v {
        // for value in &vec {}と同義
        for value in vec.iter() {
            println!("{}", value);
        }
    }

    if let Some(ref mut vec) = v {
        // for value_ref in &mut vec {}と同義
        for value_ref in vec.iter_mut() {
            *value_ref += 10;
        }
    }

    // take()を使うことによってvをNoneにして中身を取り出す
    if let Some(vec) = v.take() {
        // for value in vec {}と同義、所有権は消費されて、ループ後にvecは使えなくなる
        for value in vec.into_iter() {
            println!("{}", value);
        }
        // vecは消費済みだから二度と呼ぶことは出来ない
        // println!("{}", vec[0]);
    }

    if v.is_none() {
        println!("v is None");
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn vec_with_enum () {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for cell in row.iter() {
        match cell {
            SpreadsheetCell::Int(value) => println!("Int({value})"),
            SpreadsheetCell::Text(value) => println!("Text(\"{value}\")"),
            SpreadsheetCell::Float(value) => println!("Float({value})"),
        }
    }
}

fn main() {
    // [basic_vec, vec_with_macro] の型は [fn(); 2]（固定長配列）
    // & をつけると &[fn(); 2]（配列への参照）になり、&[fn()] にスライス化される
    let demos: &[fn()] = &[
        basic_vec, vec_with_macro, vec_with_enum
    ];
    for (i, demo) in demos.iter().enumerate() {
        print_sep((i + 1) as i32);
        demo();
    }
}
