use rs_pac::base;
use rs_pac::complex;
use rs_pac::glb;
fn main() {
    base::bstring::test_string();
    // base::bslice::test_slice();
    // base::bslice::btuple::test_tuple();
    glb::printline();
    // base::bslice::bstruct::test_struct();
    // base::bslice::bfor::test_for();
    complex::moshipipei::test_moshipipei();
    // complex::moshipipei::test_at_binding();
    // complex::fangfa::test_fangfa();
    // complex::fangfa::test_summary();
    // complex::fangfa::test_psi();
    // complex::tezheng::test_tezheng();
    // base::bslice::test_vector();

    println!("\n------ The End ------");
    glb::main_test();
}
