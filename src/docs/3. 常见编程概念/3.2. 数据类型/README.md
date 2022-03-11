# 数据类型
- 在 Rust 中，每一个值都属于某一个 `数据类型（data type）`，这告诉 Rust 它被指定为何种数据，以便明确数据处理方式。我们将看到两类数据类型子集：标量（scalar）和复合（compound）。


- 记住，Rust 是 `静态类型（statically typed）`语言，也就是说在编译时就必须知道所有变量的类型。根据值及其使用方式，编译器通常可以推断出我们想要用的类型。当多种类型均有可能时，比如第二章的 “比较猜测的数字和秘密数字” 使用 parse 将 String 转换为数字时，必须增加类型注解，像这样：
```rust
let guess: u32 = "32".parse().expect("Not a number!");
```
```rust
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


}
```
- 这里如果不添加类型注解，Rust 会显示如下错误，这说明编译器需要我们提供更多信息，来了解我们想要的类型：
```shell
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error

# 你会看到其它数据类型的各种类型注解。
```

### 标量类型
- `标量（scalar）`类型代表一个单独的值。Rust 有四种基本的标量类型：整型、浮点型、布尔类型和字符类型。你可能在其他语言中见过它们。让我们深入了解它们在 Rust 中是如何工作的。
  
***`整型`***
- 整数 是一个没有小数部分的数字。我们在第二章使用过 u32 整数类型。该类型声明表明，它关联的值应该是一个占据 32 比特位的无符号整数（有符号整数类型以 i 开头而不是 u）。表格 3-1 展示了 Rust 内建的整数类型。我们可以使用其中的任一个来声明一个整数值的类型。
**表格 3-1: Rust 中的整型**
```rust
长度	    有符号	      无符号
8-bit	    i8	        u8
16-bit	    i16	        u16
32-bit	    i32	        u32
64-bit	    i64	        u64
128-bit	    i128	    u128
arch	    isize	    usize
```
- 每一个变体都可以是有符号或无符号的，并有一个明确的大小。有符号 和 无符号 代表数字能否为负值，换句话说，这个数字是否有可能是负数（有符号数），或者永远为正而不需要符号（无符号数）。这有点像在纸上书写数字：当需要考虑符号的时候，数字以加号或减号作为前缀；然而，可以安全地假设为正数时，加号前缀通常省略。有符号数以补码形式（two’s complement representation） 存储。

- 每一个有符号的变体可以储存包含从 -(2n - 1) 到 2n - 1 - 1 在内的数字，这里 n 是变体使用的位数。所以 i8 可以储存从 -(27) 到 27 - 1 在内的数字，也就是从 -128 到 127。无符号的变体可以储存从 0 到 2n - 1 的数字，所以 u8 可以储存从 0 到 28 - 1 的数字，也就是从 0 到 255。

- 另外，isize 和 usize 类型依赖运行程序的计算机架构：64 位架构上它们是 64 位的， 32 位架构上它们是 32 位的。

- 可以使用表格 3-2 中的任何一种形式编写数字字面值。请注意可以是多种数字类型的数字字面值允许使用类型后缀，例如 57u8 来指定类型，同时也允许使用 _ 做为分隔符以方便读数，例如1_000，它的值与你指定的 1000 相同。

***表格 3-2: Rust 中的整型字面值***
```rust
数字字面值	                        例子
Decimal (十进制)	                98_222
Hex (十六进制)	                    0xff
Octal (八进制)	                    0o77
Binary (二进制)	                    0b1111_0000
Byte (单字节字符)(仅限于u8)	        b'A'
```
