extern crate bls;
use bls::Bls;

fn test_bls() {
    let mut bs = bls::newBls();
    let msg = String::from("hello");
    bs.hash_to_g2(&msg);
    //
    let rng = &mut rand::thread_rng();
    // let mut hsh = Sha256::new();
    let (sk, pk) = bs.gen_key(rng);
    let signature = bs.sign(&msg, &sk);
    let sig_g2 = signature.unwrap();
    // println!("{:?}", sig_g2);
    let is_ok = bs.verify(&msg, &pk, &sig_g2);
    println!("bls test is_ok:{}", is_ok);
}

fn main() {
    println!("==========");
    test_bls();
    bls::test_lib();
}
