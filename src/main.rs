fn main() {
  // Slice 类型
  {
    let s1 = String::from("Hello");

    let len = first_word(&s1);
    println!("len: {}",len);
    // len: 5
  }

  // TODO: 字符串 slice
  {
    let s = String::from("Hello, World");
    let s1 = &s[1..3];
    let s2 = &s[3..7];
    println!("s: {}", s);   // s: Hello, World
    println!("s1: {}", s1); // s1: el
    println!("s2: {}", s2); // s2: lo,
  }

  // TODO: 字符串 slice
  {
    let s = String::from("Hello, World");
    let len = s.len();
    let s1 = &s[3..len];
    let s2 = &s[3..];
    let s3 = &s[..];
    println!("s: {}", s);   // s: Hello, World
    println!("s1: {}", s1); // s2: lo, World
    println!("s2: {}", s2); // s2: lo, World
    println!("s3: {}", s3); // s3: Hello, World
  }

  {
    let s1 = String::from("Hello");

    let len = first_worda(&s1);
    println!("len: {}",len); // len: Hello
  }
  
}

fn first_word(s: &String) -> usize {
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return i;
    }
  }

  s.len()
}

// 在记住所有这些知识后，让我们重写 first_word 来返回一个 slice。“字符串 slice” 的类型声明写作 &str：
fn first_worda(s: &String) -> &str {
  let bytes = s.as_bytes();
  
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }

  &s[..]
}