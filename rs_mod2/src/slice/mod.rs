pub fn test_slice() {
    println!("------------ test slice -------------");
    let v1 = vec![1, 3, 5];
    for i in v1.iter().enumerate() {
        println!("ind:{},val:{}", i.0, i.1);
    }
}
