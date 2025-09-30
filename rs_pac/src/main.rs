use rs_pac::base;
use rs_pac::complex;
fn main() {
    base::bstring::test_string();
    base::bslice::test_slice();
    base::bslice::btuple::test_tuple();
    rs_pac::glb::printline();
    base::bslice::bstruct::test_struct();
    base::bslice::bfor::test_for();
    complex::moshipipei::test_moshipipei();
}
