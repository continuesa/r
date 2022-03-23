use std::option::Option
// TODO: 枚举和模式匹配

// 定义枚举
#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

// 使用枚举甚至还有更多优势。进一步考虑一下我们的 IP 地址类型，目前没有一个存储实际 IP 地址 数据 的方法；只知道它是什么 类型 的。考虑到已经在第五章学习过结构体了，你可能会像示例 6-1 那样处理这个问题
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}
// 将 IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中

enum IpAddra {
  V4(String),
  V6(String),
}

enum IpAddrb {
  V4(u8, u8, u8, u8),
  V6(String),
}



// 一个 Message 枚举，其每个成员都存储了不同数量和类型的值
/**
 * 这个枚举有四个含有不同类型的成员：
 * Quit 没有关联任何数据。
 * Move 类似结构体包含命名字段。
 * Write 包含单独一个 String。
 * ChangeColor 包含三个 i32。
 */
enum Message {
  Quit,
  Move {x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

// 类型
struct QuitMessage; // 类单元结构体
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

impl Message {
  fn call(&self) {
    // 在这里定义方法体
  }
}

/**
 * 问题不在于概念而在于具体的实现。为此，Rust 并没有空值，
 * 不过它确实拥有一个可以编码存在或不存在概念的枚举。这个
 * 枚举是 Option<T>，而且它定义于标准库中，如下:
 */
enum Option<T> {
  None,
  Some(T),
}



fn main() {

  // TODO: 枚举值
  {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:#?}", four);
    println!("four: {:#?}", six);
    /*
    four: V4
    four: V6
    */

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
      kind: IpAddrKind::V4,
      address: String::from("127.0.0.1"),
    };


    let loopback = IpAddr {
      kind: IpAddrKind::V6,
      address: String::from("::1"),
    };

  }

  {
    let home = IpAddra::V4(String::from("127.0.0.1"));
    let loopback = IpAddra::V6(String::from("::1"));

  }

  {
    let home = IpAddrb::V4(127, 0, 0, 0);
    let loopback = IpAddrb::V6(String::from("::1"));
  }


  {
    let m = Message::Write(String::from("hello"));
    m.call();
  }

  {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
  }

  {
    let x: i8 = 8;
    let y: Option<i8> = Some(5);
    
    let sum = x + y;
    println!("sum: {}", sum);
  }




  
}

fn route(ip_kind: IpAddrKind) {

}

