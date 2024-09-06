//  导入必要组件
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    //  从客户端读取数据的 buffer
    let mut buffer = [0; 1024];

    //  从云端读取数据并将数据存储在 buffer 中
    stream.read(&mut buffer).expect("读取错误！");

    //  将 buffer 中的数据转换为可编码的 UTF-8 string 字符串
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("收到请求：{}", request);
    let response = "你好，客户端！".as_bytes();
    stream.write(response).expect("写入回复失败！");

}

// Entry Pointer
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("绑定地址失败！");
    println!("服务器监听到 127.0.0.1:8080");

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // stderr - standard error stream
                eprintln!("建立连接失败: {}!", e);
            }
        }
    }
}
