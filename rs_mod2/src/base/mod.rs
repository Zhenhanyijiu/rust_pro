pub mod array {
    pub fn test_print() {
        println!("------------ array test_print ------------");
        let x = ["s001", "s022", "s003", "s004", "s005"];
        for i in x.iter().enumerate() {
            println!("ind:{},val:{}", i.0, i.1);
        }
    }
}
pub mod slice {
    pub fn test_print() {
        println!("------------ slice test_print ------------");
        let x = vec!["s001", "s022", "s003", "s004", "s005"];
        for i in x.iter().enumerate() {
            println!("ind:{},val:{}", i.0, i.1);
        }
        let mut xx = Vec::new();
        xx.push("qq");
        xx.push("qq2");
        xx.push("qq3");
        for i in xx.iter().enumerate() {
            println!("ind:{},val:{}", i.0, i.1);
        }
    }
}

pub mod hashmap {
    use std::{collections::HashMap, time};

    pub fn test_print(num: i32) {
        println!("------------ hashmap test_print ------------");

        let mut mp = HashMap::new();

        // let num = 10000;
        let start = time::Instant::now();
        for i in 0..num {
            mp.insert(i + 1, "xxx-".to_string() + &(i.to_string()));
        }
        let end = time::Instant::now();
        // let ret = (end - start).as_secs();
        println!(
            "hashmap insert {} , use time {:?}",
            num,
            (end - start).as_secs_f32()
        );
        // panic!("error panic");
    }
}
pub mod opt {
    fn get_value(fg: bool) -> Option<i32> {
        if fg {
            return Some(100);
        } else {
            return None;
        }
    }
    pub fn test_opt() {
        println!("------------ opt test_print ------------");
        let some_number: Option<i32> = Some(33);
        if some_number != None {
            let unwrapped = some_number.unwrap();
            println!("Unwrapped value: {}", unwrapped);
        } else {
            println!(">>>>>>> Option == None >>>>>");
            return;
        }
        let ret = get_value(true).unwrap();
        println!("ret:{}", ret);
    }
}
pub mod res {
    use std::fs::File;
    pub fn test_result() {
        let f = File::open("hello.txt");
        if f.is_err() {
            println!("Problem opening the file: {:?}", f);
        } else {
            println!("it is ok");
        }
        match f {
            Ok(_) => {
                println!("it is ok");
            }
            Err(_) => {
                println!("it is not ok");
            }
        }
    }
}

pub mod matc {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        let x = match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        };
        return x;
    }
    pub fn test_match() {
        let ret = value_in_cents(Coin::Penny);
        println!("ret {}", ret);
        let ret = value_in_cents(Coin::Nickel);
        println!("ret {}", ret);
        let ret = value_in_cents(Coin::Dime);
        println!("ret {}", ret);
        let ret = value_in_cents(Coin::Quarter);
        println!("ret {}", ret);
    }

    pub fn main2s() {
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(tti) => Some(tti + 1),
            }
        }

        let five = Some(50);
        let six = plus_one(five);
        println!("=== {:?}", six);
        let none = plus_one(None);
    }

    pub fn f127() {
        let some_u8_value = Some(3u8);
        println!(">>>>> {:?}", some_u8_value);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
    }
}
