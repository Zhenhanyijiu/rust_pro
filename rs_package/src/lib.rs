pub mod f1;
pub mod f2;
pub mod ut;
pub mod f3 {
    pub fn dev(a: i32, b: i32) -> i32 {
        a / b
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fn_super() {}
pub mod school {
    pub mod stu {
        pub fn study() {
            println!("stu study ...");
        }
    }
    pub mod teacher {
        pub fn teach() {
            println!("teacher teach ...");
        }
    }
}

mod fff {
    pub fn f() {
        println!("=============>>>reexporting");
    }
}

pub use crate::fff::*;
pub fn f33() {
    fff::f();
}
