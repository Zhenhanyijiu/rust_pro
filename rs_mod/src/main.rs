mod utils;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}
fn tst1() {
    let ret = utils::cal::add(2, 3);
    println!("ret = {}", ret);
    let ret2 = utils::cal::sub(10, 5);
    println!("ret2 = {}", ret2);
    let ret3 = utils::cal::mul(1, 3);
    println!("ret3 = {}", ret3);
    let ret4 = utils::cal::div(10, 5);
    println!("ret4 = {}", ret4);
    let ret5 = utils::char::genstr();
    println!("ret5 = {}", ret5);
}
fn main1() {
    let point = Point { x: 1, y: 2 };
    let p2 = point.clone();
    let serialized = serde_json::to_string(&point).unwrap();
    // 输出结果: serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    // 输出结果: deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    println!("p2 = {:?}", p2);
}
fn main() {
    main1();
    tst1();
}
