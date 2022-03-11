fn main() {
  
  {
    let guess: u32 = "43".parse().expect("Not a number!");
    println!("guess: {}", guess); // guess: 43
    // 这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：
    /*
    $ cargo build
        Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
      error[E0282]: type annotations needed
      --> src/main.rs:2:9
        |
      2 |     let guess = "42".parse().expect("Not a number!");
        |         ^^^^^ consider giving `guess` a type

      For more information about this error, try `rustc --explain E0282`.
      error: could not compile `no_type_annotations` due to previous error

      你会看到其它数据类型的各种类型注解。
    */
  }

  {
    // char str: u8 = b'A';
    // https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html#%E6%95%B0%E6%8D%AE%E7%B1%BB%E5%9E%8B
    
  }


}