extern crate bn;
extern crate crypto;
extern crate num_bigint;
use std::ops::Mul;

use bn::{pairing, Fr, Group, G1, G2};
use num_bigint::{BigInt, Sign};
use rand::ThreadRng;

use crypto::digest::Digest;
use crypto::sha2::Sha224;

#[derive(Debug)]
pub struct Bls {
    curve_name: String,
    hs: Sha224,
}
pub fn newBls() -> Bls {
    let hs = Sha224::new();
    let bs = Bls {
        curve_name: String::from("bn254"),
        hs: hs,
    };
    return bs;
}

impl Bls {
    pub fn hash_to_g2(&mut self, msg: &String) -> Option<G2> {
        self.hs.reset();
        let msg_bytes = msg.as_bytes();
        println!("> hash_to_g2,msg_bytes:{:?}", msg_bytes);
        self.hs.input(msg_bytes);
        // let ret = self.hs.result_str();
        let out = &mut [0; 32];
        self.hs.result(out);
        println!("sha224:{:?}", out);
        let hash_2_int = BigInt::from_bytes_le(Sign::Plus, out);
        let int_2_str = &hash_2_int.to_string();
        println!("int_2_str:{}", int_2_str);
        let opt: Option<Fr> = Fr::from_str(int_2_str);
        if (opt != None) {
            let sk2 = opt.unwrap();
            println!("> hash_to_g2 ok");
            return Some(G2::one() * sk2);
        } else {
            println!("> Fr::from_str error");
            return None;
        }
    }
    pub fn gen_key(&self, rng: &mut ThreadRng) -> (Fr, G1) {
        let sk = Fr::random(rng);
        let pk = G1::one() * sk;
        return (sk, pk);
    }
    pub fn sign(&mut self, msg: &String, sk: &Fr) -> Option<G2> {
        let hm_g2 = self.hash_to_g2(msg);
        if hm_g2 != None {
            let ret = Some(hm_g2.unwrap() * (*sk));
            println!("> signature ok");
            ret
        } else {
            println!("==== signature not ok");
            None
        }
    }
    pub fn verify(&mut self, msg: &String, pk: &G1, delta: &G2) -> bool {
        let hm_g2 = self.hash_to_g2(msg);
        if hm_g2 != None {
            println!("> verify hash_to_g2 ok");
            let left = pairing(*pk, hm_g2.unwrap());
            let right = pairing(G1::one() * Fr::one(), *delta);
            if left == right {
                return true;
            }
        }
        return false;
    }
}

pub fn test_lib() {
    let x_opt = Fr::from_str("11");
    let x = x_opt.unwrap();
    let x2_opt = Fr::from_str("2");
    let x2 = x2_opt.unwrap();

    let ret1 = x.pow(x2);
    let ret2 = x2.pow(x);
    println!("{:#?}", ret1 == ret2);
}
