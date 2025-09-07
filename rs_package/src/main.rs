use rs_package::{add, f1::sub, f2, f2::mul, f3::dev, school, ut};

fn main() {
    let ret = add(4, 7);

    println!("ret {}", ret);
    let ret2 = sub(10, 3);
    println!("ret2 {}", ret2);
    let ret3 = mul(8, 4);
    println!("ret3 {}", ret3);
    println!("ret3 {}", dev(10, 5));
    school::stu::study();
    school::teacher::teach();
    f2::xxx::pri();
    ut::max();
    ut::mn::min();
}
