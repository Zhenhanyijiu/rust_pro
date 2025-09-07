use rand::{self, Rng};
use rs_mod2::{array, base, slice};
// extern crate rand;

fn main() {
    // println!("Hello, world!");
    array::arr::print_array();
    slice::test_slice();
    base::array::test_print();
    base::slice::test_print();
    let num = 10 * 10000;
    base::hashmap::test_print(num);
    base::opt::test_opt();
    base::res::test_result();
    base::matc::test_match();
    base::matc::main2s();
    base::matc::f127();
    // extern crate rand;
    // rand::Rng();
    let rng = &mut rand::thread_rng();
    let ret = rng.next_u64();
    println!("random number is {}", ret);
}
