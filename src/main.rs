use std::io::Error;

// 结构体定义中的泛型
/*
 * Point 结构体存放了两个 T 类型的值 x 和 y
 */
struct Pointa<T> {
    x: T,
    y: T,
}

/*
 *  使用两个泛型的 Point，这样 x 和 y 可能是不同类型
 */
struct Pointb<T, U> {
    x: T,
    y: U,
}


// 枚举定义中的泛型
enum Option<T> {
    Some(T),
    None,
}

/**
 * Result 枚举有两个泛型类型，T 和 E。Result 有两个成员：Ok，
 * 它存放一个类型 T 的值，而 Err 则存放一个类型 E 的值。这个
 * 定义使得 Result 枚举能很方便的表达任何可能成功（返回 T 类
 * 型的值）也可能失败（返回 E 类型的值）的操作。实际上，这就是
 * 我们在示例 9-3 用来打开文件的方式：当成功打开文件的时候，T 
 * 对应的是 std::fs::File 类型；而当打开文件出现问题时，E 的
 * 值则是
 */
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 方法定义中的泛型
/*
 * 在 Point<T> 结构体上实现方法 x，它返回 T 类型的字段 x 的引用
 */
struct Pointc<T> {
    x: T,
    y: T,
}
// 实现
impl<T> Pointc<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
} 

// 构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
impl Pointc<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)) + self.y.powi(2).sqrt()
    }
}

fn main() {
    {
        let integer = Pointa {x: 5, y: 10 };    // 正确的
        let float = Pointa {x: 1.0, y: 4.0};    // 正确的
        // 错误的
        // let wont_work = Pointa {x: 5, y: 4.0}; // 这种是错误的，所以会报错
        println!("integer: x: {}, y: {}", integer.x, integer.y); // integer: x: 5, y: 10
        println!("float: x: {}, y: {}", float.x, float.y);       // float: x: 1, y: 4
    }

    println!("=================================");

    {
        let both_integer = Pointb {x: 5, y: 10};
        let both_float = Pointb {x: 1.0, y: 4.0};
        let integer_and_float = Pointb {x: 5, y: 4.0};
        println!("both_integer: x: {}, y: {}", both_integer.x, both_integer.y);
        println!("both_float: x: {}, y: {}", both_float.x, both_float.y);
        println!("integer_and_float: x: {}, y: {}", integer_and_float.x, integer_and_float.y);
    }

    println!("=================================");

    {
        let p = Pointc {x: 5, y: 10};
        println!("p.x = {},  p.y = {}", p.x(), p.y()); // p.x = 5,  p.y = 10
    }

    println!("=================================");

    {
        let p = Pointc {x: 39, y: 40};
        println!("p.x = {},  p.y = {}", p.x(), p.y()); // p.x = 39,  p.y = 40
    }

    

}