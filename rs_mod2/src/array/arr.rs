pub fn print_array() {
    println!("------------ print array -------------");
    let lst = [10, 20, 30, 40, 50];
    for i in lst.iter().enumerate() {
        println!("index:{},val:{}", i.0, i.1);
    }
}
