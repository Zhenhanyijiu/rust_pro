fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
pub fn test_fangfa() {
    println!("\n------ This is a test function test_fangfa in fangfa module.");
    let int_sum = add(5, 10);
    let float_sum = add(5.5, 10.2);
    println!("int sum: {}, float sum: {}", int_sum, float_sum);
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Post {
    title: String,
    author: String,
    content: String,
}
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

struct Webo {
    username: String,
    content: String,
}

impl Summary for Webo {
    fn summarize(&self) -> String {
        format!("{}发表了微博: {}", self.username, self.content)
    }
}
pub fn test_summary() {
    println!("\n------ This is a test function test_summary in fangfa module.");

    let post = Post {
        title: String::from("Rust语言"),
        author: String::from("sunface"),
        content: String::from("Rust是一种系统编程语言..."),
    };
    let webo = Webo {
        username: String::from("sunface"),
        content: String::from("今天天气不错，适合学习Rust!"),
    };
    println!("1 文章摘要: {}", post.summarize());
    println!("2 微博摘要: {}", webo.summarize());
}

trait Psi {
    fn init(&self) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
    fn process(&self) -> Result<(), String> {
        self.init()?;
        self.run()?;
        self.stop()?;
        Ok(())
    }
}
struct Oprf {}
impl Psi for Oprf {
    fn init(&self) -> Result<(), String> {
        println!("... Oprf init");
        Ok(())
    }
    fn run(&self) -> Result<(), String> {
        println!("... Oprf run");
        Ok(())
    }
    fn stop(&self) -> Result<(), String> {
        println!("... Oprf stop");
        Ok(())
    }
}

fn test_oprf() {
    println!("\n------ This is a test function test_oprf in fangfa module.");
    let oprf = Oprf {};
    match oprf.process() {
        Ok(_) => println!("Oprf process completed successfully."),
        Err(e) => println!("Oprf process failed: {}", e),
    }
}

struct Kkrt {}
impl Psi for Kkrt {
    fn init(&self) -> Result<(), String> {
        println!("... Kkrt init");
        Ok(())
    }
    fn run(&self) -> Result<(), String> {
        println!("... Kkrt run");
        Ok(())
    }
    fn stop(&self) -> Result<(), String> {
        println!("... Kkrt stop");
        Ok(())
    }
}
fn test_kkrt() {
    println!("\n------ This is a test function test_kkrt in fangfa module.");
    let kkrt = Kkrt {};
    match kkrt.process() {
        Ok(_) => println!("Kkrt process completed successfully."),
        Err(e) => println!("Kkrt process failed: {}", e),
    }
}
fn notify(item: &impl Psi, item2: &impl Psi) {
    _ = item.process();
    _ = item2.process();
}
fn notify2<T: Psi>(item: &T, item2: &T) {
    _ = item.process();
    _ = item2.process();
}

trait Processer {
    fn process_run(&self) -> Result<(), String>;
}

impl<T: Psi> Processer for T {
    fn process_run(&self) -> Result<(), String> {
        self.init()?;
        self.run()?;
        self.stop()?;
        Ok(())
    }
}
trait Ot {
    fn init(&self) -> Result<(), String>;
    fn run(&self) -> Result<(), String>;
    fn stop(&self) -> Result<(), String>;
}
// impl<T: Ot> Processer for T {
//     fn process_run(&self) -> Result<(), String> {
//         self.init()?;
//         self.run()?;
//         self.stop()?;
//         Ok(())
//     }
// }
fn get_largest<T: PartialOrd + Copy>(lt: &[T]) -> T {
    let mut largest = lt[0];
    for item in lt.iter() {
        if *item > largest {
            largest = *item;
        }
    }
    largest
}
fn test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = get_largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(&char_list);
    println!("The largest char is {}", result);
}
pub fn test_psi() {
    test_oprf();
    test_kkrt();
    let oprf = Oprf {};
    let oprf2 = Oprf {};
    let kkrt = Kkrt {};
    let kkrt2 = Kkrt {};
    println!("\n------ This is a test function notify in test_psi.");
    notify(&oprf, &kkrt);
    println!("\n------ This is a test function notify2 in test_psi.");
    notify2(&oprf, &oprf2);
    notify2(&kkrt, &kkrt2);

    println!("\n------ This is a test function processer in test_psi.");
    let oprf3 = Oprf {};
    let kkrt3 = Kkrt {};
    let _ = oprf3.process_run();
    let _ = kkrt3.process_run();

    println!("\n------ This is a test function largest in test_largest.");
    test_largest();
}
