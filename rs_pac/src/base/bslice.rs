pub fn test_slice() {
    println!("------ This is an test function test_slice.");

    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    println!("Slice: {:?}", slice);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);
}

pub mod btuple {
    pub fn test_tuple() {
        println!("------ This is an test function test_tuplee.");
        let tup = (500, 6.4, 1);
        let (_x, y, _z) = tup;
        println!("The value of y is: {}", y);
    }
}
pub mod bstruct {

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    fn build_user(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    pub fn test_struct() {
        println!("------ This is an test function test_struct.");
        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("emailname@google.com"),
            sign_in_count: 1,
        };
        let mut user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("wwww"),
            sign_in_count: user1.sign_in_count,
        };
        user2.email = String::from("1111111111");
        let user3 = build_user(
            String::from("build_user2222222"),
            String::from("build_user33333333"),
        );
        println!("build_user user3: {:#?} ", user3);
    }
}

pub mod bfor {
    pub fn test_for() {
        for i in 1..=5 {
            print!("{} ", i);
        }
        println!();
        // let arr = [10, 20, 30, 40, 50];
        let mut arr2: [String; 6] = std::array::from_fn(|_i| String::from("hello"));
        for i in &arr2 {
            print!("{} ", i);
        }
        println!();
        for i in arr2.iter() {
            print!("{} ", i);
        }
        println!();
        for i in arr2.iter_mut() {}
    }
}
