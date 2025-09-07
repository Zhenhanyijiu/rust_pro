use serde::{Deserialize, Serialize};

use std::env;

fn main2() {
    let args: Vec<String> = env::args().collect();
    println!("命令行参数有：");
    for (index, arg) in args.iter().enumerate() {
        println!("参数 {}: {}", index, arg);
    }
}
fn test_array() {
    let a = [11, 12, 13, 14, 15];
    for i in a.iter().enumerate() {
        println!("index :{},val:{}", i.0, i.1)
    }
    println!("==============");
    for i in a.iter() {
        println!("val {}", i);
    }
    println!("==============");
    for i in a {
        println!("{}", i)
    }
    let mut a1 = [1, 3];
    a1.fill(5);
    for i in a1.iter().enumerate() {
        println!("a1 index :{},val:{}", i.0, i.1)
    }
}
fn test_slice() {
    let s = String::from("qwerty");
    println!("s :{},len:{}", s, s.len());
    println!("s :{},len:{}", s, s.len());
    for i in s.bytes() {
        println!("bytes:{}", i);
    }
    let s2 = "9876543210";
    println!(
        "s2:{},len:{},{},{}",
        s2,
        s2.len(),
        std::mem::size_of_val("0000"),
        std::mem::size_of_val(&s2),
    );
    ////////////////////
    let p1 = &s[0..2];
    println!("p1:{}", p1);
    let p1 = &s2[2..5];
    println!(">> p1:{}", p1);

    let p2 = &s[..];
    println!(">> p2:{}", p2);
}

// #[derive(Serialize, Deserialize)]
struct serv {
    ip: String,
    port: i32,
    // num_array: [i32; 10],
    // str_array: [String; 100],
}
fn create_server() -> serv {
    // let nums = [0; 10];
    // nums.fill(77);
    let s = serv {
        ip: String::from("127.0.0.1"),
        port: 9888,
        // num_array: &nums,
    };
    let c = 'l';
    // s.num_array.fill(777);
    // s.str_array.fill(String::from("333"));
    return s;
}
// use u;
fn test_struct() {
    let s = create_server();
    // println!("{},{}", s.ip, s.port);
    prt(&s);
    prt(&s);
    println!("{}", u2::add(3, 5));
    // let x = serde_json::to_string(&s).unwrap();
}
fn prt(s: &serv) {
    println!("!!!{},{}", s.ip, s.port);
}

mod u2 {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("命令行参数有：");
    for (index, arg) in args.iter().enumerate() {
        println!("参数>>> {}: {}", index, arg);
    }
    print!("");
    test_array();
    println!("===============");
    test_slice();
    println!("===============");

    test_struct()
}
