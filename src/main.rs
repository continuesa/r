// TODO: 6.2. match 控制流运算符

#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

// 枚举
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

// 枚举
enum Coina {
  Penny,
  Nickel,
  Dime,
  Quarter,
}





fn main() {
}

// 以枚举成员作为模式的 match 表达式
fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => 1, // 第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 1
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}

fn value_in_centsa(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    }, // 第一个分支的模式是值 Coin::Penny 而之后的 => 运算符将模式和将要运行的代码分开。这里的代码就仅仅是值 1
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}