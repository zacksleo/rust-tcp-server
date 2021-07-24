use std::fs;
use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use tcp_server::ThreadPool;

// 缓存大小
const BUFFER_SIZE: usize = 1024;

fn handle_connections(mut stream: TcpStream) {
    // 声明缓存
    let mut buffer = [0; BUFFER_SIZE];
    // 将 stream 读取到 buffer 中
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 Ok\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 Ok\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents,);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    // 绑定端口
    let listener = TcpListener::bind("0.0.0.0:80").expect("cannot listen to :80");
    // 打印日志
    println!("Ready for connection from 80:");

    let pool = ThreadPool::new(4);

    // 循环迭代器来处理接收到的数据
    for stream in listener.incoming() {
        // 模式匹配遍历结果
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connections(stream);
        });
        // match stream {
        //     // 连接成功
        //     Ok(s) => thread::spawn(|| {
        //         handle_connections(s);
        //     }),
        //     // 连接失败
        //     Err(e) => panic!("encountered error: {}", e),
        // }
    }
}
