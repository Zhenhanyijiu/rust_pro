fn greet_use_string(name: String) {
    println!("Hello, {}!", name);
}

pub fn test_string() {
    println!("------ This is an test function test_string.");

    let s = String::from("Hello, Rust!");
    println!("{}", s);
    greet_use_string(s)
}
