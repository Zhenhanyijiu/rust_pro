use std::io::{self, BufRead, Read, Write};
use std::net::TcpStream;

fn main_1() -> io::Result<()> {
    // 连接到服务端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("已连接到服务端");

    // 获取标准输入，用于读取用户输入
    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        // 提示用户输入消息
        print!("请输入要发送的消息 (输入 'exit' 退出): ");
        io::stdout().flush()?;

        // 读取用户输入
        input.clear();
        stdin.lock().read_line(&mut input)?;
        let message = input.trim();

        // 如果用户输入 'exit'，则退出程序
        if message.eq_ignore_ascii_case("exit") {
            println!("断开连接");
            break;
        }

        // 发送消息到服务端
        stream.write_all(message.as_bytes())?;

        // 接收服务端的响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("服务端响应: {}", response);
    }

    Ok(())
}

fn main_2() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("已连接到服务端");
    let mut i = 1;
    loop {
        let message = format!("rust language {}", i);
        // 发送消息到服务端
        stream.write_all(message.as_bytes())?;

        // 接收服务端的响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("服务端响应: {}", response);
        i = i + 1;
        if i == 100 {
            break;
        }
    }

    Ok(())
}

fn main() {
    // _ = main_1();
    _ = main_2();
}
