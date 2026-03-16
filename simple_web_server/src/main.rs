use rust_embed::Embed;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use threadpool::ThreadPool;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

fn main() {
    let port = 7878;
    let address_string = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(address_string).unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Connection from {}", stream.peer_addr().unwrap());

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, file_path) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    println!("Trying to load html path: {}", file_path);
    let file = Assets::get(file_path).unwrap();
    let contents = str::from_utf8(&file.data).unwrap().to_string();

    let response = format!("{}\r\n\r\n{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}