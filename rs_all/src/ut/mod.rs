use core::str;
use std::str::{Chars, FromStr};
use std::{i32, option, vec};

use std::fs::File;
use std::io;
use std::io::{prelude::*, Write};

pub mod fanxing;
fn add_x_y(x: i32, y: i32) -> i32 {
    x + y
}
fn f1() -> ! {
    println!("no return");
    panic!("abort");
}
fn slice_test() {
    let s = String::from("0123456789");
    println!("s :{}", s);
    let s1 = &s[..4];
    println!("s1:{}", s1);
    let s2 = &s[3..8];
    println!("s2:{}", s2);
    let s3 = &s[3..];
    println!("s3:{}", s3);
    let s4 = &s[..];
    println!("s4:{}", s4);
    //
    let ch = String::from("中国人");
    println!("ch size:{}", ch.len());
    let ch1 = &ch[0..3];
    println!("ch1:{}", ch1);
}
fn str_add_test() {
    let s1 = String::from("nihao ");
    let s2 = String::from("rust");
    let mut result = s1 + &s2;
    result = result + "!";
    println!("=== str_add_test result:{}", result);
    let t2 = (1, 2, "ok");
    println!("=== truple :{},{},{}", t2.0, t2.1, t2.2);
}
struct Color(i32, i32, i32);
struct Week(i8, i8, i8, i8, i8, i8, i8);
fn test_struct_week() {
    let w = Week(1, 2, 3, 4, 5, 6, 7);
    println!("today is {}", w.0);
}
fn test_array() {
    let a1 = [3; 5];
    let a2 = [0; 100]; //初始化整型数组

    for (i, v) in a1.iter().enumerate() {
        println!("===>> {},{}", i, v);
    }
    // let s1 = [String::from(""); 10];//error
    let s1: [String; 10] = std::array::from_fn(|_i| String::from("rust")); //error
    for (i, v) in s1.iter().enumerate() {
        println!("==>> {},{}", i, v);
    }

    let s2 = ["hello"; 10];
    for (i, v) in s2.iter().enumerate() {
        println!("==>> {},{}", i, v);
    }
}
fn test_cycle() {
    let count = 30;
    for i in 0..count {
        println!("===>> i:{}", i);
    }
}
fn test_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
}
fn test_loop2() {
    let mut counter = 0;
    let mut ret = 0;
    loop {
        counter += 1;
        if counter == 10 {
            ret = counter * 2;
            break;
        }
    }
    println!("The result is {}", ret);
}
fn test_match() {
    println!("======> test match!!!");
    #[derive(Debug)]
    enum Day {
        One,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
    }
    let xx = Day::Two;
    match xx {
        Day::One => println!("I am One"),
        Day::Two => println!("I am Two"),
        Day::Three => println!("I am Three"),
        _ => println!("===> day:{:#?}", xx),
    };
    let n = 300;
    match n {
        100 => println!("n is {}", n),
        200 | 300 => println!("(200|300)n is {}", n),
        _ => println!("n is {}", n),
    };
}

fn tests_match_bind() {
    #[derive(Debug)]
    enum SiJi {
        Chun,
        Xia,
        Qiu,
        Dong,
    }
    enum Stu {
        XueQi(SiJi), //1000
        Yu(SiJi),    //1001
        Shu,         //1002
        Wai,         //1003
    }
    fn value_in_cents(stu: Stu) -> i32 {
        match stu {
            Stu::XueQi(siji) => {
                println!("---Xue Qi :{:?}", siji);
                1000
            }
            Stu::Yu(siji) => {
                println!("---Yu :{:?}", siji);
                1001
            }
            Stu::Shu => 1002,
            Stu::Wai => 1003,
        }
    }
    let st1 = Stu::Shu;
    println!("Stu::Shu {}", value_in_cents(st1));
    let st2 = Stu::Wai;
    println!("Stu::Wai {}", value_in_cents(st2));
    let st3 = Stu::XueQi(SiJi::Chun);
    println!("Stu::XueQi(SiJi::Chun) {}", value_in_cents(st3));
    let st3 = Stu::Yu(SiJi::Dong);
    println!("Stu::Yu(SiJi::Dong) {}", value_in_cents(st3));
}

fn tests_match_3() {
    println!("\n------------------------------");
    enum Action {
        Say(String),
        MoveTo(i32, i32),
        ChangeColorRGB(u16, u16, u16),
    }

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in &actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g,
                );
            }
        }
    }
    let fg = 9;
    if let 9 = fg {
        println!("=====>> here is iflet")
    }
}

fn tests_match_4() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
    let bar = Some(3);
    assert!(matches!(bar, Some(x) if x > 2));
    //
    let age = Some(30);
    println!("在匹配前，age是{:?}", age);
    if let Some(age) = age {
        println!("匹配出来的age是{}", age);
    }

    println!("在匹配后，age是{:?}", age);
}
fn tests_option() {
    fn f1(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 100),
            None => None,
        }
    }
    let x = Some(88);
    let ret = f1(x);
    println!("tests_option ret:{:?}", ret);
    //
    let mut stack_s = Vec::new();
    stack_s.push(1);
    stack_s.push(2);
    stack_s.push(3);
    stack_s.push(4);
    stack_s.push(5);
    let p1 = stack_s.pop();
    println!("tests_option p1:{:?},len:{}", p1, stack_s.len());
    while let Some(v) = stack_s.pop() {
        println!("tests_option while let,v:{}", v);
    }
    let s = Some(991);
    if let Some(v) = s {
        println!("模式匹配 v:{}", v);
    }
}

fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };
    println!("count_str:{},item:{}", count_str, item);
    let Ok(count) = u64::from_str(count_str) else {
        panic!("Can't parse integer: '{count_str}'");
    };
    // error: `else` clause of `let...else` does not diverge
    // let Ok(count) = u64::from_str(count_str) else { 0 };
    (count, item)
}

fn test_let_else() {
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));
    println!("{:?}", get_count_item("3 chairs"),);
    //
    let my_vec: [Option<i32>; 6] = [Some(1), Some(2), Some(3), Some(4), None, Some(6)];
    let mut case5 = Vec::from(my_vec);
    while let Some(tp) = case5.pop() {
        println!("case success {:?}", tp);
    }
    println!("---------------------------");
    let mut case5 = Vec::from(my_vec);
    while let Some(tp) = case5.pop() {
        if let Some(v) = tp {
            println!("case success {:?}", v);
        }
    }
    let (Some(s1), Some(s2)) = (Some(3), Some("yanyan")) else {
        panic!("ppppsssss");
        // println!("=============");
    };
    //
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 4, y: 2 };
    println!("{:?}", p);
    let Point { x: a, y: b } = p;
    println!("Point,a:{},b:{}\n----------", a, b);
    match p {
        Point { x, y: 0 } => println!("x:{}", x),
        Point { x: 0, y } => println!("y:{}", y),
        Point { x, y } => println!("x:{},y:{}", x, y),
        _ => println!("else"),
    }
    // #[derive(Debug)]
    #[derive(Debug)]
    struct Yuan {
        r: i32,
        c: i32,
    }
    let yuan = Yuan { r: 2, c: 20 };
    println!("yuan:{:?}\n----------", yuan);
    let xx = Some(5);
    let yy = 10;
    match xx {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", xx),
    }
    println!("at the end: x = {:?}, y = {:?}", xx, yy);
}
fn test_let_else2() {
    println!("------------ test_let_else2 ------------");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 4 };
    let Point { x, y } = p;
    println!("x:{},y:{}", x, y);
    //
    #[derive(Debug)]
    enum Color {
        RGB(i32, i32, i32),
        HSV(i32, i32, i32),
    }
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::HSV(2, 2, 2));
    let msg = Message::ChangeColor(Color::RGB(2, 2, 2));
    let msg = Message::Write(String::from("wwwwwwwwwwwwwww"));
    let msg = Message::Move { x: 300, y: 400 };
    let msg = Message::Quit;
    println!("------------ test_let_else2 ------------");
    match msg {
        // Message::Quit => println!("quit here"),
        Message::Move { x, y } => {
            println!("Message::Move:{},{}", x, y);
        }
        Message::Write(x) => {
            println!("Message::Write:{}", x);
        }
        Message::ChangeColor(Color::HSV(x, y, z)) => {
            println!("Message::ChangeColor HSV:{},{},{}", x, y, z);
        }
        Message::ChangeColor(Color::RGB(x, y, z)) => {
            println!("Message::ChangeColor RGB:{},{},{}", x, y, z);
        }
        _ => {
            println!("something else");
        }
    }
    ///////
    println!("------------------------");
    let arr: &[u16] = &[114, 514];
    if let [x, ..] = arr {
        assert_eq!(x, &114);
        println!("x:{}", x);
    }
    if let &[.., y] = arr {
        assert_eq!(y, 514);
        println!("y:{}", y);
    }
    let arr: &[u16] = &[];
    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));
    //
    println!("------------------------");
    let val1 = (1, "ee");
    match val1 {
        (x, y) => {
            println!("x:{},y:{}", x, y);
        }
        _ => (),
    }
    //
    let _xxx = 5;
    println!("_x:{}", _xxx);
}
fn test_match2() {
    println!("------------ test_match2 ------------");
    let x1 = (2, 4, 6, 8, 10);
    match x1 {
        (two, .., eight, ten) => {
            println!("two:{},eight:{},ten:{}", two, eight, ten);
        }
        (two, .., six, _, ten) => {
            println!("two:{},six:{},ten:{}", two, six, ten);
        }
        _ => (),
    }
    //
    println!("---------------------");
    let x2 = 24;
    match x2 {
        y => println!("y:{}", y),
        _ => (),
    }
    match x2 {
        y @ 22..=32 => {
            println!("@ 22..=32  y:{}", y);
        }
        y => println!("y:{}", y),
    }
    //
    println!("---------------------");
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 0 };
    match p {
        // Point { x, y: 0 } => println!("(x,0) On the x axis at {}", x),
        Point { x: 0, y } => println!("(0,y) On the y axis at {}", y),
        // Point { x, y } => println!("(x,y) On neither axis: ({}, {})", x, y),
        _ => {
            println!("something else");
        }
    }
    //
    println!("-----------------");
    let x2 = Some(String::from("aaaaaaaaaa"));
    let y2 = false;
    match x2 {
        Some(s) if y2 => println!("[if y2 ] s:{}", s),
        Some(s) => println!("s:{}", s),
        _ => println!("jjj"),
    }
    // println!("{:?}", x2);
}

fn test_class_fangfa() {
    println!("------------ test_class_fangfa ---------------");
    #[derive(Debug)]
    struct Rect {
        a: i32,
        b: i32,
    }
    impl Rect {
        fn init(a: i32, b: i32) -> Self {
            return Rect { a: a, b: b };
        }
        fn init2(a: i32, b: i32) -> Rect {
            return Rect { a: a, b: b };
        }
        fn new(a: i32, b: i32) -> Self {
            return Rect { a: a, b: b };
        }
        fn new2(a: i32, b: i32) -> Rect {
            return Rect { a: a, b: b };
        }
        fn area(&self) -> i32 {
            return self.a * self.b;
        }
        fn get_width(&self) -> i32 {
            self.a
        }
        fn get_height(&self) -> i32 {
            self.b
        }
        fn set_rect(&mut self, other: &Rect) {
            self.a = other.a;
            self.b = other.b;
        }
        fn cmp_rect(&self, other: &Rect) -> bool {
            self.a > other.a
        }
    }
    //
    let mut r1 = Rect::new2(100, 2);
    println!("===>> {:?},area:{}", r1, r1.area());
    //
    enum Color {
        Quit,
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Color {
        fn call(&self) {
            match self {
                Color::Write(s) => {
                    println!("====>> match here s:{}", s);
                }
                _ => {
                    println!("someting else");
                }
            }
        }
    }
    let c1 = Color::Write(String::from("ooooooooooo"));
    c1.call();
}

fn test_fanxing() {
    println!("---------------- test_fanxing ----------");
    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        return a + b;
    }
    println!("int:{}", add(11, 10));
    println!("float:{}", add(11.5, 10.3));
}
fn tests_trait() {
    println!("------------- test_trait ------------");
    trait Writer2 {
        fn write(&self, v: i32);
    }
    struct TT {}
    impl Writer2 for TT {
        fn write(&self, v: i32) {
            println!("write v:{}", v);
        }
    }
    let t1 = TT {};
    t1.write(55);
    fn f1(w: &impl Writer2) {
        w.write(77);
    }
    fn f2<T: Writer2>(w: &T) {
        w.write(77);
    }
    f1(&t1);
    f2(&t1);
}

fn tests_trait2() {
    println!("------------ tests_trait2 -----------");
    pub trait Summary {
        fn summarize(&self) -> String;
    }
    pub struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) -> String {
            return format!("文章:{},作者是:{}", self.title, self.author);
        }
    }
    pub struct WeiBo {
        username: String,
        content: String,
    }
    impl Summary for WeiBo {
        fn summarize(&self) -> String {
            format!("{}发表了微博{}", self.username, self.content)
        }
    }
    //
    let post = Post {
        title: "Rust语言简介".to_string(),
        author: "Sunface".to_string(),
        content: "Rust棒极了!".to_string(),
    };
    let weibo: WeiBo = WeiBo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };

    println!("{}", post.summarize());
    println!("{}", weibo.summarize());

    // let x = Some(9);
    // let x: Option<i32> = None;
    enum TT {
        X,
        Y,
    }
    let x = TT::Y;
    if let TT::X = x {
        println!(">>>>>>>>>>s:");
    } else {
        println!(">>>>>>>>>> None");
    }
    //
    struct QQ {
        arr: Vec<i32>,
    }

    fn f3(s: &mut Vec<i32>) {
        s.push(44);
    }
    let mut q = QQ { arr: Vec::new() };
    f3(&mut q.arr);
    println!("{}", q.arr[0]);
}

fn test_return() -> Result<(), String> {
    let cc = Chars::from("sss".chars());
    Ok(())
}
fn test_vector() -> Option<()> {
    println!("---------- test_vector ----------");
    let v = vec![0, 1, 2, 3, 4, 5];
    let x = v.get(3);
    let x = v.get(3)?;
    let c = &v[5];
    //
    trait IpAddr {
        fn display(&self);
    }

    struct V4(String);
    impl IpAddr for V4 {
        fn display(&self) {
            println!("ipv4: {:?}", self.0)
        }
    }
    struct V6(String); //元组结构体
                       // 单元结构
                       // 带字段的结构体

    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }
    //
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
    return None;
}
pub fn yufa() {
    let mut x = 9;
    x = 8;
    println!("x:{}", x);
    let x_char = '中';
    println!("字符占用内存:{}", std::mem::size_of_val(&x_char));
    let c_char = 'c';
    println!("字符占用内存:{}", std::mem::size_of_val(&c_char));
    println!("表达式 :{}", add_x_y(23, 32));
    println!("===============");
    slice_test();
    str_add_test();
    test_struct_week();
    test_array();
    test_cycle();
    test_loop();
    test_loop2();
    test_match();
    tests_match_bind();
    tests_match_3();
    tests_match_4();
    tests_option();
    test_let_else();
    test_let_else2();
    test_match2();
    test_class_fangfa();
    test_fanxing();
    tests_trait();
    tests_trait2();
    test_vector();
}
use std::net::TcpListener;
pub fn server_tcp(ip_port: &String) {
    println!("---------- server_tcp ---------");
    // 监听地址: 127.0.0.1:8989
    let listener: TcpListener = TcpListener::bind(ip_port).unwrap();

    for stream in listener.incoming() {
        let stream: std::net::TcpStream = stream.unwrap();
        println!("Connection established!");
    }
}
pub fn client_tcp(ip_port: &String) {
    println!("---------- client_tcp ---------");
    let n: i32 = 65;
    let tmp = n.to_le_bytes();
    println!("tmp:{:?}", tmp);
    let mut data: String = String::new();
    let tmp2 = tmp.to_vec();
    let tmp3 = String::from_utf8_lossy(&tmp2).to_string();
    println!("{}", tmp3);

    // data.push_str(tmp.to_vec());
}
pub fn test_vec_str_string() {
    println!("------------ test_vec_str_string -------------");
    let s1 = String::from("uuuuu");
    let s2 = String::from("dddd");
    let s3 = s1.clone() + &s2;
    println!("s3:{}", s3);
    let mut v1: Vec<u8> = vec![1, 2, 3];
    let mut v2: Vec<u8> = vec![1, 2, 3];
    // v1.append(&v2);
    v1.append(&mut v2);
    println!("v1:{:?}", v1);
    let v3 = s1.into_bytes();
    // let v3 = s1.into_bytes();
    let s1 = v3.to_vec();
}

pub fn test_file() {
    let mut f = File::create("data.txt").expect("create error");
    let mut buffer = [0; 10];
    let data = "rust is a language";
    // f.write_all(data.as_bytes());
    let byt = data.as_bytes();
    f.write_all(byt).expect("write_all error");
    f.write_all("\nit is well\n".as_bytes())
        .expect("write_all error");
}

pub fn test_file2() -> io::Result<()> {
    let mut f = File::create("data.txt")?;
    let mut buffer = [0; 10];
    let data = "rust is a language";
    // f.write_all(data.as_bytes());
    let byt = data.as_bytes();
    f.write_all(byt)?;
    f.write_all("\nit is well\n".as_bytes())?;
    Ok(())
}
