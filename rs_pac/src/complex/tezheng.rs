// use std::convert::TryInto;

fn test_trait() {
    println!("\n------ This is an test function test_trait.");
    let a: i32 = 10;
    let b: u16 = 100;
    let b_ = b.try_into().unwrap();
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}

use std::ops::Add;

struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn test_add() {
    println!("\n------ This is an test function test_add.");
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
struct Point2<T> {
    x: T,
    y: T,
}
impl<T: Add> Add for Point2<T>
where
    T: Add<T, Output = T>,
{
    type Output = Point2<T>;
    fn add(self, other: Point2<T>) -> Point2<T> {
        Point2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
fn test_add2() {
    println!("\n------ This is an test function test_add2.");
    let p1 = Point2 { x: 1, y: 0 };
    let p2 = Point2 { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

pub fn test_tezheng() {
    test_trait();
    test_add();
    test_add2();
}
