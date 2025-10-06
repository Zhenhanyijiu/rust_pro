enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}
pub fn test_moshipipei() {
    println!("------ This is a test function in moshipipei module.");
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, world!"));
    let msg4 = Message::ChangeColor(255, 11, 123);
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}
fn test_at_binding2() {
    match 1 {
        num @ (1 | 2) => {
            println!("=== num {}", num);
        }
        _ => {}
    }
}

pub fn test_at_binding() {
    println!("\n------ This is a test function in at_binding module.");
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

    let point = Point { x: 10, y: 5 };
    if let p @ Point { x: 10, y } = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
    test_at_binding2()
}
