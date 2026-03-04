use std::thread;
use std::thread::spawn;
use std::time::Duration;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn thread_test() {
    let handle = spawn(|| {
        for i in 0..10 {
            println!("Thread: {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Main: {i}");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn thread_with_move() {
    let vec = vec![1, 2, 3];
    let handle = spawn(move || {
        println!("Here's a vector: {:?}", vec);
    });
    // vecはすでにmoveされたためdrop出来ない
    // drop(vec);
    handle.join().unwrap();
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 20 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width * r.height
    });
    println!("{:#?}", list);

    thread_test();
    thread_with_move();
}
