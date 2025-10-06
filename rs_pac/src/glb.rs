use crate::base;
use crate::complex;

pub fn printline() {
    println!("\n---------------------------");
}

pub fn main_test() {
    base::bstring::test_string();
    base::bslice::test_slice();
    base::bslice::btuple::test_tuple();
    printline();
    base::bslice::bstruct::test_struct();
    base::bslice::bfor::test_for();
    complex::moshipipei::test_moshipipei();
    complex::moshipipei::test_at_binding();
    complex::fangfa::test_fangfa();
    complex::fangfa::test_summary();
    complex::fangfa::test_psi();
    complex::tezheng::test_tezheng();
    base::bslice::test_vector();
}
