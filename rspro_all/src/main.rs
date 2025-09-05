pub mod ut;

use ut::fanxing::{test_fang_xing, test_fang_xing2};
use ut::{client_tcp, server_tcp, test_file, test_file2, test_vec_str_string, yufa};

fn main() {
    println!("----------- ");
    // yufa();
    // test_vec_str_string();
    test_fang_xing();
    // let argv: Vec<String> = std::env::args().collect();
    // let role = &argv[1];
    // let ip_port = "127.0.0.1:8989".to_string();

    // if role.eq("0") {
    //     server_tcp(&ip_port);
    // } else if role.eq("1") {
    //     client_tcp(&ip_port);
    // } else {
    //     println!("not operation");
    // }
    // test_file();
    // test_file2();
}
