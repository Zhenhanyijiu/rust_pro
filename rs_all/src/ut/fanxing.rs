use core::fmt;
use std::f32::consts::E;
use std::io::Read;
use std::thread::Thread;
use std::time::{Duration, Instant};
use std::{
    borrow::{Borrow, BorrowMut},
    cell::Cell,
    collections::HashMap,
    env::{consts, var},
    fmt::Display,
    iter::Sum,
    ops::Deref,
    process::Output,
    u8,
};
use std::{future, io, thread};
use std::{string, time};
// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]
fn test_time() {
    let start = Instant::now();
    thread::sleep(Duration::from_millis(1000));
    let dur: Duration = start.elapsed();
    println!("{}", dur.as_millis());
}
struct TimePoint {
    start: Instant,
}
impl TimePoint {
    fn new() -> Self {
        println!("------------- 0.0 ms ------------");
        TimePoint {
            start: Instant::now(),
        }
    }
    fn time_here(&self) {
        let dur = self.start.elapsed();
        println!("------------- {} ms ------------", dur.as_millis());
    }
}
pub fn test_fang_xing() {
    println!("\n-------- test_fang_xing ---------");
    // fangxing1();
    // trait_test();
    // trait_test2();
    // trait_test3();
    // trait_test4();
    // trait_test6();
    // trait_test5();
    // trait_test8();
    // box_test1();
    // box_test2();
    // box_drop_test3();
    // box_thread_test4();
    // box_thread_test5();
    // box_test6();
    // // box_test7();
    // box_weak_test8();
    // box_unsafe_test9();
    // box_pin_test10();
    // trait_test11();
    // trait_test12();
    // trait_test13();
    // trait_test14();
    // trait_test15();
    // vector_test16();
    // vector_test17();
    // vector_test18();
    // vector_test19();
    let count = 1 * 5000 * 10000;

    // vector_test20(count);
    // vector_test21();
    // vector_test22(count);
    // vector_test23(count);
    // vector_test24(count);
    // test_lifetime25();
    // test_panic26();
    test_wenhao27();
    // test_time();
}
fn test_wenhao27() {
    println!("\n-------- test_wenhao27 ---------");
    fn first(arr: &[i32]) -> Option<&i32> {
        let r = arr.get(0)?;
        Some(r)
        //
        // arr.get(0)
    }
    //
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
        // let x = text.lines().next()?.chars().last()?;
        // Some(x)
    }
    use std::error::Error;
    use std::fs::File;

    fn main1() -> Result<(), Box<dyn Error>> {
        let f = File::open("hello.txt");
        Ok(())
    }
}

fn test_panic26() {
    println!("\n-------- test_panic26 ---------");
    use std::net::IpAddr;
    let home: IpAddr = "127.0.0.1".parse().unwrap();
    let home: IpAddr = "127.0.0.1".parse().expect("<<error exp>>");
    use std::fs::File;

    fn main1() {
        // let f = File::open("hello.txt").unwrap();
        // let f = File::open("hello.txt");
        let f = File::open("hello1.txt").is_err();
        println!("bool:{f},{}", f as i32)
        // match f {
        //     None => {
        //         println!("===========2")
        //     }
        //     Some(x) => {
        //         println!("======1,{}", x)
        //     }
        // }
    }
    main1();
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(x) => x,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        let ret = f.read_to_string(&mut s);
        match ret {
            // 返回想要的结果
            Ok(_) => Ok(s),
            // 将错误上传
            Err(e) => Err(e),
        }
    }
    match read_username_from_file() {
        Ok(x) => {
            println!("read ok :{}", x);
        }
        Err(e) => {
            println!("read failed :{}", e);
        }
    }
}
fn test_lifetime25() {
    println!("\n-------- test_lifetime25 ---------");
    fn main2() {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    // 函数中的生命周期
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    main2();
    //
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    fn main3() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    fn main4() {
        let x = String::new();
        let y = x.as_str();
    }
    main3();
}

fn vector_test24(count: i32) {
    println!("\n-------- vector_test24 ---------");
    use ahash::AHashMap;
    let mut hh: AHashMap<u64, u64> = AHashMap::default();
    let time_point = TimePoint::new();
    let first: u64 = 10000000;
    let txt = 10000000000000u64;
    println!("======= k-v 个数:{}", count);
    for i in 0..count {
        let k = first + (i as u64);
        let ret = hh.insert(k, txt);
        match ret {
            Some(x) => {
                panic!("error");
            }
            None => {}
        }
        // println!("{:?},{:?}", k, txt);
    }
    println!("下面是创建 hashmap 时间戳....");
    time_point.time_here();
    for i in 0..count {
        let k = first + (i as u64);
        let ret = hh.get(&k);
    }
    println!("下面是查询 hashmap 时间戳.....");
    time_point.time_here();
}
fn vector_test23(count: i32) {
    println!("\n-------- vector_test23 ---------");
    use ahash::AHashMap;
    let mut hh: AHashMap<u64, u128> = AHashMap::default();
    let time_point = TimePoint::new();
    let first: u64 = 10000000;
    let txt = 10000000000000u128;
    println!("======= k-v 个数:{}", count);
    for i in 0..count {
        let k = first + (i as u64);
        // let k = (first + (i.0 as u64)).to_string();
        let ret = hh.insert(k, txt);
        match ret {
            Some(x) => {
                panic!("error");
            }
            None => {}
        }
        // println!("{:?},{:?}", k, txt);
    }
    println!("下面是创建 hashmap 时间戳....");
    time_point.time_here();
    for i in 0..count {
        let k = first + (i as u64);
        let ret = hh.get(&k);
    }
    println!("下面是查询 hashmap 时间戳.....");
    time_point.time_here();
}
fn vector_test22(count: i32) {
    println!("\n-------- vector_test22 ---------");
    use ahash::{AHasher, RandomState};
    let mut hh: HashMap<u64, u128, RandomState> = HashMap::default();
    let time_point = TimePoint::new();
    let first: u64 = 10000000;
    let txt = 10000000000000u128;
    println!("======= k-v 个数:{}", count);
    for i in 0..count {
        let k = first + (i as u64);
        // let k = (first + (i.0 as u64)).to_string();
        let ret = hh.insert(k, txt);
        match ret {
            Some(x) => {
                panic!("error");
            }
            None => {}
        }
        // println!("{:?},{:?}", k, txt);
    }
    println!("下面是创建 hashmap 时间戳.....");
    time_point.time_here();
    for i in 0..count {
        let k = first + (i as u64);
        let ret = hh.get(&k);
    }
    println!("下面是查询 hashmap 时间戳.....");
    time_point.time_here();
}
fn vector_test21() {
    println!("\n-------- vector_test21 ---------");
    let mut h1 = HashMap::new();
    let text = "aa bb cc dd ee ff ff aa";
    // 统计单词出现的次数，用 hashmap 去做比较简单
    for v in text.split_whitespace() {
        let count = h1.entry(v).or_insert(0);
        *count += 1;
    }
    for v in &h1 {
        println!("{},{}", v.0, v.1);
    }
    let ret = h1.entry("aa");
    println!("{:?}", ret);
}
fn vector_test20(count: i32) {
    println!("\n-------- vector_test20 ---------");
    let mut h1 = HashMap::new();
    let ret1 = h1.insert("k1".to_string(), 12);
    println!("{:?}", ret1);
    let ret2 = h1.insert("k1".to_string(), 122);
    println!("{:?}", ret2);
    let ret3 = h1.get(&"k1".to_string());
    println!("{:?}", ret3);
    //
    let time_point = TimePoint::new();
    let mut hh: HashMap<u64, u128> = HashMap::new();
    let first: u64 = 10000000;
    let txt = 10000000000000u128;
    // let count = 1 * 1000 * 10000;
    println!("======= k-v 个数:{}", count);

    for i in 0..count {
        let k = first + (i as u64);
        // let k = (first + (i.0 as u64)).to_string();
        let ret = hh.insert(k, txt);
        match ret {
            Some(x) => {
                panic!("error");
            }
            None => {}
        }
        // println!("{:?},{:?}", k, txt);
    }
    println!("下面是创建 hashmap 时间戳.....");
    time_point.time_here();
    for i in 0..count {
        let k = first + (i as u64);
        let ret = hh.get(&k);
    }
    println!("下面是查询 hashmap 时间戳.....");
    time_point.time_here();
    //
    let mut hh2 = HashMap::new();
    let k = 12345u64;

    hh2.insert(k, txt);
    let ok = hh2.get(&(k + 1));
    match ok {
        Some(x) => {
            println!("find the key ok ...");
        }
        None => {
            println!("not find the key ...");
        }
    }
    let k2 = 1000u64;
    let v = hh2.entry(k2).or_insert(200000u128);
    println!("{:?}", v);
    println!("{}", v);
    println!("{}", v);
}
fn vector_test19() {
    println!("\n-------- vector_test19 ---------");

    let mut hm = HashMap::new();
    let ret1 = hm.insert(23, "xx".to_string());
    println!("ret:{:?}", ret1);
    let ret2 = hm.insert(23, "xx3".to_string());
    println!("------- ret:{:?}", ret2);
    let mut hm2 = HashMap::new();
    let mut ret3 = hm2.insert("k1".to_string(), 55);
    println!("ret:{:?}", ret3);
    let mut ret4 = hm2.insert("k2".to_string(), 55);
    let mut ret4 = hm2.insert("k3".to_string(), 55);
    let mut ret4 = hm2.insert("k4".to_string(), 55);
    let mut ret4 = hm2.insert("k5".to_string(), 55);
    let mut ret4 = hm2.insert("k6".to_string(), 55);
    let k7 = "k7".to_string();
    let mut hm3 = HashMap::new();
    let mut ret4 = hm3.insert(k7.clone(), 55);
    // std::mem::drop(&k7);
    println!("ret:{:?},{:?}", ret4, k7);
    let mut ret = hm3.get(&k7);
    for v in hm3 {
        println!(">> {},{}", v.0, v.1);
    }
    println!("------------------------");
}

pub fn test_fang_xing2() {
    println!("\n-------- test_fang_xing2 ---------");
}
fn vector_test18() {
    println!("\n-------- vector_test18 ---------");
    struct Stu {
        name: String,
        age: i32,
    }
    trait People {
        fn get_name(&self) -> &String;
        fn eat(&self) {
            println!("{} eat eat", self.get_name());
        }
    }
    impl People for Stu {
        fn get_name(&self) -> &String {
            &self.name
        }
    }
    impl Stu {
        fn study(&self) {
            println!("{} need studying", self.get_name())
        }
    }
    let s = Stu {
        name: "LiLei".to_string(),
        age: 18,
    };
    s.study();
    s.eat();
    let mut n = -101;
    println!("{}", n.to_string());
    let n_str = n.to_string();
    let ret = n_str.parse::<i8>().unwrap();
    println!(">> {}", ret);
}
fn vector_test17() {
    println!("\n-------- vector_test17 ---------");
    #[derive(Debug)]
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let arr = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string()),
    ];
    for v in &arr {
        println!("{:?}", v);
    }
    //
    trait Ip {
        fn display(&self);
    }
    // 第二种方法是特征对象，第一种方法是使用枚举类型
    struct Ipv4(String);
    struct Ipv6(String);
    impl Ip for Ipv4 {
        fn display(&self) {
            println!("v4:{}", self.0);
        }
    }
    impl Ip for Ipv6 {
        fn display(&self) {
            println!("v6:{}", self.0);
        }
    }
    let arr2: Vec<Box<dyn Ip>> = vec![
        Box::new(Ipv4("127.0.0.1".to_string())),
        Box::new(Ipv6("::1".to_string())),
    ];
    for v in &arr2 {
        v.display();
    }
    //
    println!("-------------------");
    let mut arr3 = Vec::with_capacity(100);
    arr3.push("value".to_string());
    fn pri_arr3(a: &Vec<String>) {
        println!("capacity:{}", a.capacity());
        println!("len:{}", a.len());
    }
    pri_arr3(&arr3);
    arr3.reserve(300);
    pri_arr3(&arr3);
    for v in arr3.iter().enumerate() {
        println!("i:{},v:{:?}", v.0, v.1);
    }
    let mut s3 = String::new();
    println!("=={}", s3.capacity());
    s3.push_str("yyyy");
    println!("=={},{}", s3.capacity(), s3.len());
    println!("{},{},{}", 1, 2, 3);
}
fn vector_test16() {
    println!("\n-------- vector_test16 ---------");
    let mut arr = vec![1, 2, 3, 4, 5];
    let first = &arr[0];
    // v.push(6);
    println!("The first element is: {:?}", first);
    for v in &arr {
        println!("{}", v);
    }
    println!("------------");
    for v in arr.iter() {
        println!("{}", v);
    }
    let mut arr2 = vec![
        "a".to_string(),
        "b".to_string(),
        "c".to_string(),
        "d".to_string(),
    ];
    for v in &arr2 {
        println!("{}", v);
    }
    println!("------------");
    for v in arr2.iter() {
        println!("{}", v);
    }
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
    struct V6(String);
    impl IpAddr for V6 {
        fn display(&self) {
            println!("ipv6: {:?}", self.0)
        }
    }
    fn main3() {
        // : Vec<Box<dyn IpAddr>>
        let v: Vec<Box<dyn IpAddr>> = vec![
            Box::new(V4("127.0.0.1".to_string())),
            Box::new(V6("::1".to_string())),
        ];
        for ip in v {
            ip.display();
        }
    }
    main3();
}
fn trait_test15() {
    println!("\n-------- trait_test15 ---------");
    pub trait A {
        fn get_str(&self) -> String;
    }
    pub trait B: A {
        fn pri(&self) {
            let ret = self.get_str();
            println!("{}", ret);
        }
    }
    struct T {}
    impl A for T {
        fn get_str(&self) -> String {
            "trait A for T".to_string()
        }
    }
    impl B for T {
        // fn pri(&self) {
        //     let ret = self.get_str();
        //     println!("{}", ret);
        // }
    }
    let t = T {};
    t.pri();
    //
    use std::fmt;
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }
    fn main1() {
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
    main1();
    let vs = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let ret = vs.join(",  ");
    println!("{}", ret);
}
fn trait_test14() {
    println!("\n-------- trait_test14 ---------");
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("Pilot  fly:This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Wizard fly:Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("Human  fly:*waving arms furiously*");
        }
    }
    let person = Human;
    // person.fly();
    <Human>::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);
    //
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            return String::from("Dog baby_name:Spot");
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("Animal baby_name:puppy")
        }
    }
    println!("dog is called a $ {}", Dog::baby_name());
    // 需要一个完全限定语法
    println!("需要一个完全限定语法");
    println!("dog is called a $ {}", <Dog as Animal>::baby_name());
    println!("dog is called a $ {}", <Dog>::baby_name());

    println!("{}", 6.8 as i32);
}
fn trait_test13() {
    use std::ops::Add;
    println!("\n-------- trait_test13 ---------");
    pub trait TT {}
    pub trait Iterater {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
        // fn next2(&mut self) -> Option<Self::Item>;
    }
    struct Counter;
    impl Iterater for Counter {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            Some(8)
        }
    }
    //
    let mut c = Counter;
    println!("{}", c.next().unwrap());
    // println!("{}", c.next().unwrap());
    // println!("{}", c.next().unwrap());
    // pub trait CacheableItem: Clone + Default + fmt::Debug + Decodable + Encodable {
    //     type Address: AsRef<[u8]> + Clone + fmt::Debug + Eq + Hash;
    //     fn is_null(&self) -> bool;
    // }
    struct Millimeters(f32);
    struct Meters(f32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000.0))
        }
    }
    impl Add<Millimeters> for Meters {
        type Output = Meters;
        fn add(self, rhs: Millimeters) -> Self::Output {
            Meters(self.0 + (rhs.0 / 1000.0))
        }
    }
    let mill = Millimeters(12.0);
    let met = Meters(1.0);
    let ret = mill + met;
    println!("{}", ret.0);
    let mill2 = Millimeters(12.0);
    let met2 = Meters(1.0);
    let ret2 = met2 + mill2;
    println!("{}", ret2.0);
    //
    use std::fmt::Display;
    // 特征定义中的特征约束
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    struct Point {
        x: i32,
        y: i32,
    }
    impl Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            // format!("{},{}", self.x, self.y)
            write!(f, "{},{}", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}
    let p1 = Point { x: 77, y: 99 };
    p1.outline_print();
}
fn trait_test12() {
    println!("\n-------- trait_test12 ---------");
    pub trait Draw {
        fn draw(&self);
    }
    struct Button {
        width: i32,
        height: i32,
        label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            println!("{},{},{}", self.label, self.width, self.height);
        }
    }
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            println!(
                "{},{},{}",
                self.options.get(0).unwrap(),
                self.width,
                self.height
            );
        }
    }
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
    impl Screen {
        fn run(&self) {
            for v in self.components.iter() {
                v.draw();
            }
        }
    }
    let s1 = Screen {
        components: vec![
            Box::new(Button {
                width: 20,
                height: 30,
                label: "绘制按钮".to_string(),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 300,
                options: vec!["绘制菜单".to_string()],
            }),
        ],
    };
    s1.run();
    // 使用泛型
    pub struct ScreenT<T: Draw> {
        compo: Vec<T>,
    }
    impl<T> ScreenT<T>
    where
        T: Draw,
    {
        fn run(&self) {
            for v in self.compo.iter() {
                v.draw();
            }
        }
    }

    let s2 = ScreenT {
        compo: vec![
            Button {
                width: 77,
                height: 88,
                label: "button".to_string(),
            },
            Button {
                width: 777,
                height: 888,
                label: "button".to_string(),
            },
            Button {
                width: 66,
                height: 22,
                label: "button".to_string(),
            },
            // SelectBox {
            //     width: 33,
            //     height: 44,
            //     options: vec!["selectbox".to_string()],
            // },
            // Box::new(SelectBox {
            //     width: 33,
            //     height: 44,
            //     options: vec!["selectbox".to_string()],
            // }),
        ],
    };
    s2.run();
}
fn trait_test11() {
    println!("\n-------- trait_test11 ---------");
    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T> 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法
        println!("{}", x.draw());
    }
    fn draw2(x: &dyn Draw) {
        println!("{}", x.draw());
    }
    fn main1() {
        let x = 1.1f64;
        // do_something(&x);
        let y = 8u8;
        // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T> 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
        // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
        draw1(Box::new(x));
        // 基于 y 的值创建一个 Box<u8> 类型的智能指针
        draw1(Box::new(y));
        draw2(&x);
        draw2(&y);
    }
    main1();
}

fn fangxing1() {
    println!("\n-------- fangxing1 ---------");

    fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
        a + b
    }
    fn largest<T: std::cmp::PartialOrd + Clone>(list: &[T]) -> T {
        let mut tmp = list[0].clone();
        for i in list.iter() {
            if *i > tmp {
                tmp = i.clone();
            }
        }
        tmp
    }
    println!("{}", add(4, 7));

    fn create_and_point<T>()
    where
        T: From<i32> + Display,
    {
        // let a: T = 100.into();
        let a: T = 100.into();
        println!("a is :{}", a);
    }
    create_and_point::<i64>();
    create_and_point::<i64>();
    struct Point<T> {
        x: T,
        y: T,
    }
    let p1 = Point { x: 10, y: 77 };
    enum Opt<T> {
        Some(T),
        None,
    }

    enum Res<T, E> {
        OK(T),
        Err(E),
    }
    impl<T> Point<T> {
        fn f1(&self) -> &T {
            return &self.x;
        }
    }
    let p2 = Point { x: 100, y: 200 };
    println!("p.x={}", p2.f1());
    //
    struct Point2<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> Point2<T, U> {
        fn mixup<V, W>(self, oth: Point2<V, W>) -> Point2<V, U> {
            return Point2 {
                x: oth.x,
                y: self.y,
            };
        }
    }
    //
    let p3 = Point2 { x: 10, y: 10.5 };
    let p4 = Point2 { x: "ooo", y: 'r' };
    let p5 = p3.mixup(p4);
    println!("===> {:?},{:?}", p5.x, p5.y);
    //
    impl Point<f32> {
        fn distance(self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let p6: Point<f32> = Point { x: 10.0, y: 10.0 };
    println!("dis:{}", p6.distance());
    //
    fn display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
        println!("display_arr:{:?}", arr);
    }
    fn display_arr2<T: std::fmt::Debug>(arr: &[T]) {
        println!("display_arr:{:?}", arr);
    }
    let arr1 = [23, 45, 67];
    let arr2 = [23, 45, 67, 89];
    display_arr(arr1);
    display_arr(arr2);
    display_arr2(&arr1);
    display_arr2(&arr2);
    //
}

fn trait_test() {
    println!("\n-------- trait_test ---------");
    pub trait Summary {
        fn pri(&self) {
            println!("----------- pri in trait -----");
        }
        fn summarize(&self) -> String;
    }
    pub struct Post {
        pub title: String,
        pub author: String,
        pub content: String,
    }
    impl Summary for Post {
        // fn pri(&self) {
        //     println!("---- Sumary for Post pri ----");
        // }
        fn summarize(&self) -> String {
            format!("文章{}, 作者是{}", self.title, self.author)
        }
    }
    //
    pub struct Weibo {
        pub username: String,
        pub content: String,
    }
    impl Summary for Weibo {
        // fn pri(&self) {
        //     println!("---- Sumary for Weibo pri ----");
        // }
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
    let weibo = Weibo {
        username: "sunface".to_string(),
        content: "好像微博没Tweet好用".to_string(),
    };
    //
    // post.summarize();
    println!("{}", post.summarize());
    println!("{}", weibo.summarize());
    post.pri();
    weibo.pri();
}
fn trait_test2() {
    println!("\n-------- trait_test2 ---------");
    pub trait Summary {
        fn process1(&self);
        fn process2(&self);
        fn process3(&self);
        fn run_all(&self) {
            self.process1();
            self.process2();
            self.process3();
            println!("---- run_all end ---")
        }
    }
    pub struct Weibo {
        name: String,
    }
    impl fmt::Display for Weibo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Weibo:{}", self.name)
        }
    }
    impl Summary for Weibo {
        fn process1(&self) {
            println!("{} .... process1", self.name);
        }
        fn process2(&self) {
            println!("{} .... process2", self.name);
        }
        fn process3(&self) {
            println!("{} .... process3", self.name);
        }
    }
    //
    let weibo = Weibo {
        name: String::from("weibooo"),
    };
    // weibo.run_all();
    //
    struct Post {
        name: String,
    }
    impl fmt::Display for Post {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Post:{}", self.name)
        }
    }
    impl Summary for Post {
        fn process1(&self) {
            println!("{} .... process1", self.name);
        }
        fn process2(&self) {
            println!("{} .... process2", self.name);
        }
        fn process3(&self) {
            println!("{} .... process3", self.name);
        }
    }
    let post = Post {
        name: String::from("posttttt"),
    };
    // post.run_all();
    //
    // fn run(v: &impl Summary) {
    //     println!("??????????????");
    //     v.run_all();
    // }
    // fn run2(v: &impl Summary) {}
    //
    pub fn notify<T: Summary, U: Summary>(item1: &T, item2: &U) {
        item1.run_all();
        item2.run_all();
    }
    pub fn notify2<T, U>(item1: &T, item2: &U)
    where
        T: Summary + Display,
        U: Summary + Display,
    {
        item1.run_all();
        item2.run_all();
        println!("{}", item1);
        println!("{}", item2);
    }
    // run(&weibo);
    // run(&post);
    notify(&weibo, &post);
    notify2(&weibo, &post);
    //
    fn add<T>(a: T, b: T) -> T
    where
        T: std::ops::Add<Output = T>,
    {
        a + b
    }
    fn add2<T>(a: T, b: T)
    where
        T: std::ops::Add,
    {
        a + b;
    }
    fn add3<T>(a: T, b: T) -> f32
    where
        T: std::ops::Add<Output = f32>,
    {
        a + b
    }
    impl Summary for String {
        fn process1(&self) {}
        fn process2(&self) {}
        fn process3(&self) {}
    }
}
fn trait_test3() {
    println!("\n-------- trait_test3 ---------");
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Pair { x, y }
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }
    let p1 = Pair::new(22, 55);
    p1.cmp_display();
    let p2 = Pair::new("123", "456");
    p2.cmp_display();
    let p3 = Pair::new(String::from("123"), String::from("456"));
    p3.cmp_display();
    // assert_eq!(String::from("123"), String::from("456"));
    use std::cmp::{Eq, Ordering, PartialEq, PartialOrd};

    // #[derive(PartialEq)]
    struct Pr1 {
        x: i32,
    }
    impl PartialEq for Pr1 {
        fn eq(&self, other: &Self) -> bool {
            self.x == other.x
        }
    }
    impl Display for Pr1 {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.x)
        }
    }
    impl PartialOrd for Pr1 {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.x.cmp(&other.x))
        }
    }
    let x1 = Pr1 { x: 2 };
    let y1 = Pr1 { x: 300 };
    let p4 = Pair::new(x1, y1);
    p4.cmp_display();
    // std::string::ToString;
    let x1 = Pr1 { x: 2 };
    println!("{}", x1.to_string());
}
fn trait_test4() {
    println!("\n-------- trait_test4 ---------");
    pub trait Print {
        fn pri(&self) -> Option<String>;
    }
    pub trait ToStr {
        fn to_str(&self) -> String;
    }
    impl<T: Print> ToStr for T {
        fn to_str(&self) -> String {
            println!("------>> to_str ----");
            self.pri().unwrap()
        }
    }
    struct ST1 {
        x: i32,
    }
    impl Print for ST1 {
        fn pri(&self) -> Option<String> {
            Some(format!("[x:{}]", self.x))
        }
    }
    let st1 = ST1 { x: 56 };
    println!("{}", st1.to_str());
    println!("{}", "33".parse::<i32>().unwrap());
}
fn trait_test5() {
    println!("\n-------- trait_test5 ---------");
    pub trait Summary {
        fn summarize(&self);
    }
    struct Weibo {
        username: String,
        content: String,
    }
    impl Summary for Weibo {
        fn summarize(&self) {}
    }
    struct Post {
        title: String,
        author: String,
        content: String,
    }
    impl Summary for Post {
        fn summarize(&self) {}
    }
    fn returns_summarizable() -> impl Summary {
        Weibo {
            username: String::from("sunface"),
            content: String::from("m1 max太厉害了，电脑再也不会卡"),
        }
    }
    fn returns_summarizable2(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(Post {
                title: String::from("Penguins win the Stanley Cup Championship!"),
                author: String::from("Iceburgh"),
                content: String::from("The Pi."),
            })
        } else {
            Box::new(Weibo {
                username: String::from("horse_ebooks"),
                content: String::from("of course,people"),
            })
        }
    }
}

fn trait_test6() {
    println!("\n-------- trait_test6 ---------");
    use std::ops::Add;
    // 为Point结构体派生Debug特征，用于格式化输出
    #[derive(Debug)]
    struct Point<T: Add<T, Output = T>> {
        //限制类型T必须实现了Add特征，否则无法进行+操作。
        x: T,
        y: T,
    }
    impl<T: Add<T, Output = T>> Add for Point<T> {
        type Output = Point<T>;
        fn add(self, p: Point<T>) -> Point<T> {
            Point {
                x: self.x + p.x,
                y: self.y + p.y,
            }
        }
    }
    fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
        a + b
    }
    fn main2() {
        let p1 = Point {
            x: 1.1f32,
            y: 1.1f32,
        };
        let p2 = Point {
            x: 2.1f32,
            y: 2.1f32,
        };
        println!("{:?}", add(p1, p2));

        let p3 = Point { x: 1i32, y: 1i32 };
        let p4 = Point { x: 2i32, y: 2i32 };
        println!("{:?}", add(p3, p4));
    }

    main2();
}
fn trait_test7() {
    println!("\n-------- trait_test7 ---------");
    pub trait Draw {
        fn draw(&self);
    }
    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            // 绘制按钮的代码
        }
    }
    struct SelectBox {
        width: u32,
        height: u32,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            // 绘制SelectBox的代码
        }
    }
    //
    pub struct Screen {
        pub components: Vec<Box<dyn Draw>>,
    }
}
fn trait_test8() {
    println!("\n-------- trait_test8 ---------");

    trait Draw {
        fn draw(&self) -> String;
    }
    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", *self)
        }
    }
    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", *self)
        }
    }
    // 若 T 实现了 Draw 特征， 则调用该函数时传入的 Box<T>
    // 可以被隐式转换成函数参数签名中的 Box<dyn Draw>
    fn draw1(x: Box<dyn Draw>) {
        // 由于实现了 Deref 特征，Box 智能指针会自动解引用为它所包裹的值，
        // 然后调用该值对应的类型上定义的 `draw` 方法
        println!("draw1:{}", x.draw());
    }

    fn draw2(x: &dyn Draw) {
        println!("draw2:{}", x.draw());
    }
    fn main2() {
        let x = 1.1f64;
        // do_something(&x);
        let y = 8u8;
        // x 和 y 的类型 T 都实现了 `Draw` 特征，因为 Box<T>
        // 可以在函数调用时隐式地被转换为特征对象 Box<dyn Draw>
        // 基于 x 的值创建一个 Box<f64> 类型的智能指针，指针指向的数据被放置在了堆上
        // Box::new(x)
        draw1(Box::new(x));
        // 基于 y 的值创建一个 Box<u8> 类型的智能指针
        draw1(Box::new(y));
        draw2(&x);
        draw2(&y);
    }
    main2();
}
fn box_test1() {
    println!("\n-------- box_test1 ---------");
    let a = Box::new(4);
    let b = *a + 1;
    println!("a:{},b:{}", a, b);
    let a1 = [0; 10];
    let a2 = a1;
    println!("{},{}", a1.len(), a2.len());
    let a3 = Box::new([0; 10]);
    let a4 = &a3;
    println!("{},{}", a4.len(), a3.len());
    pub trait Draw {
        fn draw(&self);
    }
    struct Draw1();
    impl Draw for Draw1 {
        fn draw(&self) {
            println!("draw draw1 ...");
        }
    }
    struct Draw2();
    impl Draw for Draw2 {
        fn draw(&self) {
            println!("draw draw2 ...");
        }
    }
    // 实现运行时多态
    let arr: Vec<Box<dyn Draw>> = vec![
        Box::new(Draw1 {}),
        Box::new(Draw2 {}),
        Box::new(Draw1 {}),
        Box::new(Draw2 {}),
    ];
    for v in &arr {
        v.draw();
    }
    for v in &arr {
        v.draw();
    }
    fn gen_static_str() -> &'static str {
        let mut s = String::new();
        s.push_str("hello rust!");
        Box::leak(s.into_boxed_str())
    }
    let s = gen_static_str();
    println!("after gen_static_str:{}", s);
}
fn box_test2() {
    println!("\n-------- box_test2 ---------");
    struct Person {
        name: String,
        age: u8,
    }
    impl Person {
        fn new(name: String, age: u8) -> Self {
            Person { name, age }
        }
        fn display(self: &mut Person, age: u8) {
            let Person { name, age } = &self;
            println!("name:{},age:{}", name, age);
        }
    }
    //
    let mut p1 = Person::new(String::from("nihao"), 33);
    p1.display(99);
    //
    let x = 5;
    let y = &x;
    let _z = *y;
    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, _z);
    //
    let x1 = Box::new(3);
    assert_eq!(*x1, 3);
    // 一元组结构体
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }
    let x2 = 5;
    let y2 = MyBox(5);
    assert_eq!(5, *y2);
    use std::ops::Deref;
    impl<T> Deref for MyBox<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    //
    let y3 = MyBox::new(Person {
        name: String::from("ooo"),
        age: 99,
    });
    let y4 = y3.deref();
    println!("{},{}", y3.name, y4.name);
    //
    struct Stu {
        name: String,
        age: u8,
    }
    impl Stu {
        // 借用，得保证在作用域内
        fn ret_ref(&self) -> &Self {
            &self
        }
        fn reff() -> Self {
            // 所有权转移
            Stu {
                name: String::from("value22"),
                age: 44,
            }
        }
    }
    let s1 = Stu {
        name: String::from("value"),
        age: 77,
    };
    let s2 = s1.ret_ref();
    // let s3 = s1;
    println!("{}", s2.age);
    fn display(s: &str) {
        println!(">>> display:{}", s);
    }
    let s = String::from("hello world");
    display(&&&&&s);
    // 可以连续的解引用
    let s = MyBox::new(String::from("=== hello world"));
    display(&s);
    display(&(*s)[..]);
    let s1: &str = &s;
    let s2: String = s.to_string();
    // let s3: String = s.to_string();
    println!("{},{}", s1, s2);
}
fn box_drop_test3() {
    println!("\n-------- box_drop_test3 ---------");
    struct HasDrop1;
    struct HasDrop2;
    impl Drop for HasDrop1 {
        fn drop(&mut self) {
            println!("Dropping HasDrop1!");
        }
    }
    impl Drop for HasDrop2 {
        fn drop(&mut self) {
            println!("Dropping HasDrop2!");
        }
    }
    struct HasTwoDrops {
        one: HasDrop1,
        two: HasDrop2,
    }
    impl Drop for HasTwoDrops {
        fn drop(&mut self) {
            println!("Dropping HasTwoDrops!");
        }
    }
    struct Foo;
    impl Drop for Foo {
        fn drop(&mut self) {
            println!("Dropping Foo!")
        }
    }
    fn main1() {
        println!("-------- main1 in ---------");
        let _x = HasTwoDrops {
            two: HasDrop2,
            one: HasDrop1,
        };
        let _foo = Foo;
        drop(_foo);
        println!("Running!");
    }
    println!("-------- main1 out1 ---------");
    main1();
    println!("-------- main1 out2 ---------");

    //
    use std::rc::Rc;
    let a = Rc::new(String::from("value"));
    let b = Rc::clone(&a);

    assert_eq!(2, Rc::strong_count(&a));
    assert_eq!(2, Rc::strong_count(&b));
    //
    fn main2() {
        let a = Rc::new(String::from("test ref counting"));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Rc::clone(&a);
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Rc::clone(&a);
            println!("count after creating c = {}", Rc::strong_count(&c));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    }
    main2();
    //
    struct Owner {
        name: String,
    }
    struct Gadget {
        id: i32,
        owner: Rc<Owner>,
    }
    let gad_owner = Rc::new(Owner {
        name: "Gadget Man".to_string(),
    });
    let gad1 = Gadget {
        id: 1001,
        owner: Rc::clone(&gad_owner),
    };
    let gad2 = Gadget {
        id: 1002,
        owner: Rc::clone(&gad_owner),
    };
    println!("ref count:{}", Rc::strong_count(&gad_owner));
    println!("ref count:{}", Rc::strong_count(&gad2.owner));
    drop(gad_owner);
    println!("ref count:{}", Rc::strong_count(&gad2.owner));
    println!("Gadget {} owned by {}", gad1.id, gad1.owner.name);
    println!("Gadget {} owned by {}", gad2.id, gad2.owner.name);
    println!("-------- box_drop_test3 end ---------");
}
fn box_thread_test4() {
    println!("\n-------- box_thread_test4 ---------");
    use std::rc::Rc;
    use std::thread;
    let s = Rc::new(String::from("多线程漫游者"));
    // for _ in 0..10 {
    //     let s = Rc::clone(&s);
    //     let handle = thread::spawn(move || println!("{}", s));
    // }
    use std::cell::Cell;
    let c = Cell::new("66666");
    println!("cell:{}", c.get());
    println!("cell:{}", c.get());
    c.set("88888");
    println!("cell:{}\n", c.get());
    //
    // use std::cell::RefCell;
    // let s = RefCell::new(String::from("hello, world"));
    // let s1 = s.borrow();
    // let s2 = s.borrow_mut();
    // println!("{},{}", s1, s2);
}
fn box_thread_test5() {
    println!("\n-------- box_thread_test5 ---------");
    use std::cell::Cell;
    // code snipet 1
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());
    // println!("{}", x);
    let n = 9;
    let n1 = &n;
    let n2 = &n;
    println!("{},{}", n1, n2);
    let mut m = 99;
    let m1 = &m;
    let m2 = &mut m;
    // *m2 = 88;
    println!("{}", m2);
    println!("{}", m2);
}
fn box_test6() {
    use std::cell::RefCell;
    use std::rc::Rc;
    println!("\n-------- box_test6 ---------");
    pub trait Messager {
        fn send(&self, da: String);
    }
    struct MsgQueue {
        msg_cache: RefCell<Vec<String>>,
    }
    impl Messager for MsgQueue {
        fn send(&self, da: String) {
            self.msg_cache.borrow_mut().push(da);
        }
    }
    //
    let s = Rc::new(RefCell::new("我很善变，还拥有多个主人".to_string()));
    // let s1 = s.clone();
    let s1 = Rc::clone(&s);
    // let s2 = s.clone();
    let s2 = Rc::clone(&s);

    // s2.borrow_mut().push_str(", oh yeah!");
    (*s2).borrow_mut().push_str(" string");

    // let mut ss = "oo".to_string();
    // ss.push_str("string");
    println!("{:?}\n{:?}\n{:?}", s, s1, s2);
}
fn box_test7() {
    println!("\n-------- box_test7 ---------");
    use std::cell::*;
    use std::rc::*;
    // let c = Cell::new("value");
    // let c = RefCell::new(String::from("value"));

    // println!("{}", c.get());
}
fn box_weak_test8() {
    use std::rc::Rc;
    use std::rc::Weak;
    println!("\n-------- box_weak_test8 ---------");
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    assert_eq!(*strong_five.unwrap(), 5);
    drop(five);
    let strong_five: Option<Rc<_>> = weak_five.upgrade();
    assert_eq!(strong_five, None);
    //
    let s = "3333".to_string();
    struct SelfRef<'a> {
        value: String,
        pointer_to_val: &'a str,
    }
    // let ss = SelfRef {
    //     value: s,
    //     pointer_to_val: &s,
    // };
    #[derive(Debug)]
    struct WhatAboutThis<'a> {
        name: String,
        nickname: Option<&'a str>,
    }
    // 此方法限制很多，无法从一个函数创建并返回
    fn main1() {
        let s = "Annabelle".to_string();
        let mut tricky = WhatAboutThis {
            name: s,
            nickname: None,
        };
        tricky.nickname = Some(&tricky.name[..4]);
        println!("{:?}", tricky);
    }
    main1();
    // unsafe 实现
}
fn box_unsafe_test9() {
    println!("\n-------- box_unsafe_test9 ---------");
    struct SelfRef {
        value: String,
        pointer_to_value: *const String,
    }
    impl SelfRef {
        fn new(txt: &str) -> Self {
            SelfRef {
                value: txt.to_string(),
                pointer_to_value: std::ptr::null(),
            }
        }
        fn init(&mut self) {
            let self_ref: *const String = &self.value;
            self.pointer_to_value = self_ref;
            // self.pointer_to_value = &self.value;
        }
        fn value(&self) -> &str {
            &self.value
        }
        fn pointer_to_value(&self) -> &String {
            // unsafe { &*(self.pointer_to_value) };
            assert!(
                !self.pointer_to_value.is_null(),
                "Test::b called without Test::init being called first"
            );
            // unsafe { &*(self.pointer_to_value) }
            unsafe { &*(self.pointer_to_value) }
        }
    }

    //
    let mut t = SelfRef::new("hello");
    t.init();
    // 打印值和指针地址
    println!("{}, {:p}", t.value(), t.pointer_to_value());
}
fn box_pin_test10() {
    println!("\n-------- box_pin_test10 ---------");
    println!("\n-------- box_pin_test10 ---------");
}
