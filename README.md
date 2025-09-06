# RUST_PRO

## 基础环境以及安装

1. 打开终端并输入下面命令安装 rust 环境：$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh

2. vscode 环境搭建

3. 使用 cargo 创建、编译、运行项目
   - cargo new project_name
   - cd project_name
   - cargo build 
   - cargo run

## cargo

1. cargo new (pro-name)
   - 例如：cargo new mypro

2. [编译]cargo build

3. [编译+运行]cargo run

4. vscode debug 配置如下：
```json
{    
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Debug",
            "type": "gdb",
            "request": "launch",
            "target": "./target/debug/test2",
            "cwd": "${workspaceRoot}",
            "valuesFormatting": "parseText"
        }
    ]
}
```

5. 所有权

```rust
// 对于基本类型，只是值传递，其实就是复制
let x=5;
let y=x;
// 对于堆中的数据就不是了
let s1=String::from("hh");
let s2=s1;
// s1绑定一个堆中的数据，s1,s2都是存储在栈中的，s1栈中的数据复制到s2中，s2也绑定了同一个堆中的数据，离开作用域释放堆内存。所以此时s1已经被移动了，已经失效了，不能再被使用了。
// 如果想保持两份数据那就使用clone方法
```

6. 引用

```rust
let s1=String::from("rr");
let s2=&s1;
// 此时s2是s1的引用，也叫s2租借s1，并没有指向实际的变量的堆中的值。s1仍然有效，可以继续使用；
```

7. 使用第三方包的步骤
   - 配置项目管理工具cargo
   - 在cargo.toml中添加依赖
   - 引入使用第三方包

8. serde 库的使用
   - https://blog.wangjunfeng.com/post/2024/rust-serde/
   - cargo add serde --features derive
   - cargo add serde_json
   - https://course.rs/cargo/guide/package-layout.html
   - https://rustwiki.org/zh-CN/book/title-page.html
  
## cargo 使用指南

1. 上手
   - cargo build --release 编译成release版本
   - cargo run --release release版本
   - 如果你的程序在跑性能测试 benchmark，一定要使用 Release 模式，因为该模式下，程序会做大量性能优化

2. 基础
   - 为啥需要cargo
     - Rust 有两种类型的包: 库包和二进制包，前者是我们经常使用的依赖包，用于被其它包所引入，而后者是一个应用服务，可以编译成二进制可执行文件进行运行
     - 引入两个元数据文件，包含项目的方方面面信息: Cargo.toml 和 Cargo.lock
     - 获取和构建项目的依赖，例如 Cargo.toml 中的依赖包版本描述，以及从 crates.io 下载包
     - 调用 rustc (或其它编译器) 并使用的正确的参数来构建项目，例如 cargo build
     - 引入一些惯例，让项目的使用更加简单
   - 下载并构建 Package
     - $ git clone https://github.com/rust-lang/regex.git
     - $ cd regex
     -  cargo build
   - 添加依赖
     -  [dependencies]\
        time = "0.1.12"
   - package 目录结构
     - 如下：
        ![package](./package.png)    

3. 进阶
   - 指定依赖项
     - 从其它注册服务引入依赖包\
        [registries]\
        ustc = { index = "https://mirrors.ustc.edu.cn/crates.io-index/" }\
        [dependencies]\
        time = {  registry = "ustc" }
     - 通过路径引入本地依赖包\
        geometry = { path = "crates/geometry" }


## rust 语法
### 所有权规则
首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

- Rust 中的每一个值都有一个被称为其所有者（owner）的变量。
- 值在任一时刻有且只有一个所有者。
- 当所有者（变量）离开作用域，这个值将被丢弃。
### 所有权转移和借用的区别
### 集合访问
- 第一种方式是循环索引，然后通过索引下标去访问集合，这样会有一定的性能损耗，因为会检查数组是否越界。性能低且不安全。
- 第二种方式是直接循环集合中的元素，这种方式性能高又安全。
### 模式匹配
- 模式是 Rust 中的特殊语法，它用来匹配类型中的结构和数据，它往往和 match 表达式联用
- if let 往往用于匹配一个模式，而忽略剩下的所有模式的场景
- 一个与 if let 类似的结构是 while let 条件循环，它允许只要模式匹配就一直进行 while 循环
- for循环，使用 enumerate 方法产生一个迭代器，该迭代器每次迭代会返回一个 (索引，值) 形式的元组，然后用 (index,value) 来匹配。
- 使用 let-else 匹配，即可使 let 变为可驳模式。它可以使用 else 分支来处理模式不匹配的情况，但是 else 分支中必须用发散的代码块处理（例如：break、return、panic）。
- 通过序列 ..= 匹配值的范围（..= 语法允许你匹配一个闭区间序列内的值）
- 在 match 表达式中，可以使用 | 语法匹配多个模式，它代表 或的意思（1 | 2 => println!("one or two"),）
- 也可以使用模式来解构结构体、枚举、元组、数组和引用。
- 在 match 中，我们有讲过变量遮蔽的问题，这个在匹配命名变量时会遇到
- matches!（返回的是true false）
  - 其实就是我们常用到的match表达式，只是在使用matches!宏表达起来更简洁
- 虽然 _ 模式作为 match 表达式最后的分支特别有用，但是它的作用还不限于此。例如可以将其用于函数参数中,代码会完全忽略作为第一个参数传递的值;
- 使用下划线开头忽略未使用的变量,let _x = 5;
- 匹配守卫提供的额外条件
  - 匹配守卫（match guard）是一个位于 match 分支模式之后的额外 if 条件，它能为分支模式提供更进一步的匹配条件
- @（读作 at）运算符允许为一个字段绑定另外一个变量。
  - Message::Hello { id: id_variable @ 3..=7 }根据条件绑定到变量 id_variable 上。
  - @前绑定后解构(Rust 1.56 新增)
  - 当你既想要限定分支范围，又想要使用分支的变量时，就可以用 @ 来绑定到一个新的变量上，实现想要的功能。
  - 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
    - // 绑定新变量 `p`，同时对 `Point` 进行解构 [let p @ Point {x: px, y: py } = Point {x: 10, y: 23};]

### 结构体方法
- self、&self 和 &mut self
  - self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
  - &self 表示该方法对 Rectangle 的不可变借用
  - &mut self 表示可变借用
### 泛型
- 显式地指定泛型的类型参数
- 为具体的泛型类型实现方法
- 特征 Trait
  - 如果我们想定义一个文件系统，那么把该系统跟底层存储解耦是很重要的。文件操作主要包含四个：open 、write、read、close，这些操作可以发生在硬盘，可以发生在内存，还可以发生在网络 IO。
  - 可能你是第一次听说这个名词，但是不要怕，如果学过其他语言，那么大概率你听说过接口，没错，特征跟接口很类似
  - 特征定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为
  - 定义特征
    - 如果不同的类型具有相同的行为，那么我们就可以定义一个特征，然后为这些类型实现该特征。定义特征是把一些方法组合在一起，目的是定义一个实现某些目标所必需的行为的集合
    - 使用特征作为函数参数
    - impl Summary，只能说想出这个类型的人真的是起名鬼才，简直太贴切了，顾名思义，它的意思是 实现了 Summary 特征 的 item 参数。(实现了 Summary 抽象接口的item参数)
  - 特征约束(trait bound)
    - 虽然 impl Trait 这种语法非常好理解，但是实际上它只是一个语法糖
    - 多重约束
      - pub fn notify(item: &(impl Summary + Display)) {}
      - pub fn notify<T: Summary + Display>(item: &T) {}
    - where 约束
      - fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,U: Clone + Debug{}
    - 使用特征约束有条件地实现方法或特征
    - 函数返回中的 impl Trait
    - 关于特征实现与定义的位置，有一条非常重要的原则：如果你想要为类型 A 实现特征 T，那么 A 或者 T 至少有一个是在当前作用域中定义的！
### 结构体 有三种结构体
- 单元结构体、元组结构体、带字段的结构体；       