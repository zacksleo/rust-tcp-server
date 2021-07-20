use std::io::prelude::*;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

const BUFFER_SIZE: usize = 1024;

fn handle_connections(mut stream: TcpStream) {
    // 声明缓存
    let mut buffer = [0; BUFFER_SIZE];
    // 将 stream 读取到 buffer 中
    stream.read(&mut buffer).unwrap();
    // 打印收到的消息
    print!("Got msg: {}", String::from_utf8_lossy(&buffer[..]));
    // 响应消息
    stream.write(&buffer[..]).unwrap();
}

fn main() {
    // 绑定端口
    let listener = TcpListener::bind("0.0.0.0:80").expect("cannot listen to :80");
    // 打印日志
    println!("Ready for connection from 80:");

    // 循环迭代器来处理接收到的数据
    for stream in listener.incoming() {
        // 模式匹配遍历结果
        match stream {
            // 连接成功
            Ok(s) => handle_connections(s),
            // 连接失败
            Err(e) => panic!("encountered error: {}", e),
        }
    }
}
