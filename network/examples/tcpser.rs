use network::tcp;
use network::tcp::Io;
fn main() {
    let ip_port = String::from("127.0.0.1:8900");
    let cli = tcp::TcpSer::new(&ip_port).unwrap();
    // let data = String::from("hello world");
    let data = cli.recv().unwrap();
    println!("recv data:{}", data);
    let n = cli.send(&data).unwrap();
    println!("send data:{}", n);
}
