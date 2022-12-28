## rust模块使用
### 同级目录下引入其他文件模块
现在`hello.rs`和`main.rs`在同一级目录下，我想要在`main.rs`中访问`hello.rs`模块的内容。
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── hello.rs
    └── main.rs
```
首先我们要定义好`hello.rs`模块的内容，将需要公开的函数添加前缀`pub`。比如我现在设置`hello.rs`的内容如下所示:
```rust
// hello.rs
pub mod hello_mod {
    pub fn hello_fn(){
        println!("我是hello_mod模块下的helloFn函数");
    }
}
```
现在我们想要在`main.rs`中访问`hello_fn`函数，只需添加一行`mod hello`即可:
```rust
// main.rs
mod hello;

fn main() {
    hello::hello_mod::hello_fn();
    println!("Hello, world!");
}
```
也可以设置下缩写或者用`as`给函数一个重命名
```rust
// main.rs
mod hello;
use hello::hello_mod::hello_fn as hello_fn2;

fn main() {
    hello::hello_mod::hello_fn();
    hello_fn2(); //等价于上面一行
    println!("Hello, world!");
}
```
### 不同级目录下引入其他文件模块
现在的目录结构变成了这样:
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── runtime
        └── config.rs
```
`config.rs`文件内容如图:
```rust
// config.rs
pub mod config_mod {
  pub fn config_fn(){
      println!("我是runtime下面的config_mod模块下的config_fn函数");
  }
}
```
现在我想要在main.rs中访问到`config_fn`函数，有两种方法。
#### 方法一：使用`mod.rs`声明模块
首先添加在`runtime`文件夹下新建一个`mod.rs`:
```
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    └── runtime
        ├── config.rs
        └── mod.rs
```
设置`mod.rs`内容：
```rust
// runtime/mod.rs
pub mod config;
```
接着就可以在`main.rs`中访问到`config_fn`函数了
```rust
//main.js 
mod runtime;

fn main() {
    runtime::config::config_mod::config_fn();
    println!("Hello, world!");
}
```
同样的，你也可以设置重命名导出
```rust
// runtime/mod.rs
mod config;
pub use  config::config_mod::config_fn;
```
在`main.rs`中直接引入`config_fn`函数
```rust
// main.rs
mod runtime;

fn main() {
    runtime::config_fn();
    println!("Hello, world!");
}
```
### 方法二：建立与目录同名的rs文件来导出模块,如新建`runtime.rs`
```rust
├── Cargo.lock
├── Cargo.toml
└── src
    ├── main.rs
    ├── runtime
    │   └── config.rs
    ├── runtime.rs
```
然后`rumtime.rs`中导出下模块
```rust
pub mod config;
```
接着在`main.rs`中使用`config_fn`函数
```rust
// main.rs
mod runtime;
fn main(){
    runtime::config::config_mod::config_fn();
    println!("Hello, world!");
}
```
同理，也可以重命名导出函数
```rust
//runtime.rs
mod config;
pub use config::config_mod::config_fn;
```
然后在`main.rs`中直接使用`config_fn`函数
```rust
//main.rs
mod runtime;
fn main(){
    runtime::config_fn();
    println!("Hello, world!");
}
```
### 在当前项目中引入其他本地项目
比如我在当前项目新建了一个`aalib`的库
```
├── Cargo.lock
├── Cargo.toml
├── aalib
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
└── src
    ├── main.rs
```
`lib.rs`文件的内容如下:
```rust
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```
我想要访问`aalib`下的`lib`文件中的`add`函数，how to?
首先配置下当前项目的`Cargo.toml`
```rust
// Cargo.toml
//左侧aalib与建立的aalib库项目名保持一致，右边的路径是aalib相对于当前项目的路径
aalib = {path = "./aalib"}
```
接着就可以在`main.rs`中使用了
```
// main.rs
use aalib::add;
fn main(){
    add(1, 2);
    println!("Hello, world!");
}
```

### todo cargo workspaces

### 参考链接
[Rust引入其他的rs文件](https://blog.csdn.net/weixin_44676081/article/details/121025703)

[了解下Rust模块使用方式](https://juejin.cn/post/7070481262749679653)

[给前端看的rust教程](https://juejin.cn/post/7037831488917733412)















