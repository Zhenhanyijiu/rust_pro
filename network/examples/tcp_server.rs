use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // 缓冲区用于存储接收的数据
    let mut buffer = [0; 1024];

    loop {
        // 读取客户端发送的数据
        match stream.read(&mut buffer) {
            Ok(n) if n == 0 => {
                // 读取到 0 字节表示连接关闭
                println!("客户端断开连接");
                break;
            }
            Ok(n) => {
                // 打印接收到的数据
                let message = String::from_utf8_lossy(&buffer[..n]);
                println!("收到消息: {}", message);

                // 向客户端发送响应
                let response = format!("已收到: {}", message);
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("发送响应失败: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("读取数据失败: {}", e);
                break;
            }
        }
    }
}

fn main() -> io::Result<()> {
    // 绑定到本地地址和端口 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("服务端已启动，监听端口 8080...");
    let v = Vec::from([1, 2, 3]);
    // 循环接受客户端连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新客户端连接: {}", stream.peer_addr()?);

                // 为每个客户端创建一个新线程处理
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
            }
        }
    }

    Ok(())
}
