// 同级目录访问
// main.rs

// mod hello;
// mod runtime;
// use hello::hello_mod::hello_fn as hello_fn2;

// fn main() {
//     hello::hello_mod::hello_fn();
//     hello_fn2();
//     println!("Hello, world!");
// }


//不同级目录访问 方法1 设置mod.rs
// mod runtime1;
// pub mod runtime;

// fn main() {
//     runtime1::config_fn();
//     println!("Hello, world!");
// }

//不同级目录访问 方法2 runtime.rs

// mod runtime;
// fn main(){
//     runtime::config_fn();
//     println!("Hello, world!");
// }

use aalib::add;
fn main(){
    add(1, 2);
    println!("Hello, world!");
}
