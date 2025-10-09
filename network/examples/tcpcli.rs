use network::tcp;
use network::tcp::Io;

fn main_test1() {
    let ip_port = String::from("127.0.0.1:8900");
    let cli = tcp::TcpCli::new(&ip_port).unwrap();
    let data = String::from("rust language");
    let n = cli.send(&data).unwrap();
    println!("send {} bytes", n);
    let r = cli.recv().unwrap();
    println!("recv:{}", r);
}
fn main_test2() {
    let ip_port = String::from("127.0.0.1:8900");
    let cli = tcp::TcpCli::new(&ip_port).unwrap();
    let count = 100;
    for i in 1..=count {
        let data = format!("rust language {}", i);
        let n = cli.send(&data).unwrap();
        println!("send {} bytes", n);
        let r = cli.recv().unwrap();
        println!("recv:{}", r);
    }
}
fn main() {
    // main_test1();
    main_test2();
}
