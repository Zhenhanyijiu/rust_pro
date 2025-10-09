use network::tcp;
use network::tcp::Io;

fn main_test1() {
    let ip_port = String::from("127.0.0.1:8900");
    let ser = tcp::TcpSer::new(&ip_port).unwrap();
    // let data = String::from("hello world");
    let data = ser.recv().unwrap();
    println!("recv data:{}", data);
    let n = ser.send(&data).unwrap();
    println!("send data:{}", n);
}
fn main_test2() {
    let ip_port = String::from("127.0.0.1:8900");
    let ser = tcp::TcpSer::new(&ip_port).unwrap();
    let count = 100;
    for i in 1..=count {
        let data = ser.recv().unwrap();
        println!("recv {} data:{}", i, data);
        let n = ser.send(&data).unwrap();
        println!("send {} data:{}", i, n);
    }
}
fn main() {
    // main_test1();
    main_test2();
}
