fn main() {
  // 3.5. 控制流


  // TODO: if 表达式
  {
    let number = 3;
    
    if number < 5 {
      println!("condition was trun");
    } else {
      println!("condition was false");
    }

    // condition was trun
  }

  {
    /* let number = 3;
    if number {
      println!("number was three");
    } */
    // 这里 if 条件的值是 3，Rust 抛出了一个错误：

    // 正确写法
    let number = 3;
    if number != 0 {
      println!("Number was something other than zero");
    }
    // 运行代码会打印出 Number was something other than zero
  }


  // TODO: 使用 else if 处理多重条件
  /*
   * 可以将 if 和 else 组成的 else if 表达式来实现多重条件。例如：
   */
  {
    let number = 6;

    if number % 4 == 0 {
      println!("number is divisible by 4");
    } else if number % 3 == 0 {
      println!("number is divisible by 3");
    } else if number % 2 == 0 {
      println!("number is divisible by 2");
    } else {
      println!("number is not divisible by 4, 3, or 2");
    }

    // number is divisible by 3

    
  }

  // TODO: 在 let 语句中使用 if
  /*
   * 因为 if 是一个表达式，我们可以在 let 语句的右侧使用它来将结果赋值给
   * 一个变量，例如在示例 3-2 中： 
   */
  {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    // The value of number is: 5
    // 将 if 表达式的返回值赋给一个变量

  }
   // 错误的写法
   /*
    * 记住，代码块的值是其最后一个表达式的值，而数字本身就是一个表达
    * 式。在这个例子中，整个 if 表达式的值取决于哪个代码块被执行。
    * 这意味着 if 的每个分支的可能的返回值都必须是相同类型；
    * 在示例 3-2 中，if 分支和 else 分支的结果都是 i32 整型。如果它们的类型不匹配，如下面这个例子，则会产生一个错误：
    */

   /* {
     let condition = true;
     let number = if condition { 5 } else { "six " };
     println!("The value of number is: {}", number);
   } */

   /*
    *  当编译这段代码时，会得到一个错误。if 和 else 分支的值类型
    *  是不相容的，同时 Rust 也准确地指出在程序中的何处发现的这个问题：
    */

    // TODO: 使用循环重复执行
    /*
     * 多次执行同一段代码是很常用的，Rust 为此提供了多种循环（loop），它们遍历执行循环体中的代码直到结尾并紧接着回到开头继续执行。为了试验循环，让我们新建一个名为 loops 
     * 
     * Rust 有三种循环：loop、while 和 for。我们每一个都试试。
     * 
     */

    // 使用 loop 重复执行代码
    /*
     * loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
     */
    /* loop {
      println!("这是一个 loop 循环体 again!");
    } */
    /*
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    * 这是一个 loop 循环体 again!
    */

    /*
     * 如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。你可以选择在一个循环上指定一
     * 个循环标签（loop label），然后将标签与 break 或 continue 一起使用，使这些关键字应
     * 用于已标记的循环而不是最内层的循环。下面是一个包含两个嵌套循环的示例：
     * 
     * 
     * 外层循环有一个标签 counting_up，它将从 0 数到 2。没有标签的内部循环
     * 从 10 向下数到 9。第一个没有指定标签的 break 将只退出内层循
     * 环。break 'counting_up; 语句将退出外层循环。这个代码打印:
     */

     {
       let mut count = 0;
       'counting_up: loop {
         println!("count: {}", count);
         let mut remaining = 10;
         loop {
           println!("remaining = {}", remaining); // remaining = 10
           if remaining == 9 {
             break;
           }
           if count == 2 {
             break 'counting_up;
           }
           remaining -= 1;
         }

         count += 1;
       }

       println!("End count = {}", count); // End count = 2

     }

     // TODO: 从循环返回
     {
       let mut counter = 0;
       let result = loop {
         counter += 1;
         
         if counter == 10 {
           break counter * 2;
         }
       };
       println!("The result is {}", result); // The result is 20
     }


     // TODO: while 条件循环
     /*
      在程序中计算循环的条件也很常见。当条件为真，执行循环。当条件不再为真，调用 break 
      停止循环。这个循环类型可以通过组合 loop、if、else 和 break 来实现；如果你喜欢的话，现在就可
      以在程序中试试。然而，这个模式太常用了，Rust 为此内置了一个语言结构，它被称为 while 循环。
      示例 3-3 使用了 while 来程序循环 3 次，每次数字都减 1。接着在循环结束后，打印出另一个信息并退出。
      */
      {
        let mut number = 3;
        while number != 0 {
          println!("{} !", number);
          number -= 1;
        }

        println!("LIFTOFF!!!: {}", &number); // LIFTOFF!!!: 0
      }


    // TODO: 使用 for 遍历集合
    /*
     * 可以使用 while 结构来遍历集合中的元素，比如数组。例
     * 如，示例 3-4 中的循环打印数组 a 中的每个元素。
     */
    {
      let a = [10, 20, 30, 40, 50];
      let mut index = 0;
      while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
      }
      /*
      the value is: 10
      the value is: 20
      the value is: 30
      the value is: 40
      the value is: 50
      */
      // 使用 while 循环遍历集合中的元素

    }
    // 作为更简洁的替代方案，可以使用 for 循环来对一个集合的每个元素执行一些代码。for 循环看起来如
    {
      let a = [10, 20, 30, 40, 50];
      for element in a {
        println!("for in the value is: {}", element);
      }
      /*
      for in the value is: 10
      for in the value is: 20
      for in the value is: 30
      for in the value is: 40
      for in the value is: 50
      */
    }

    // 下面是一个使用 for 循环来倒计时的例子，它还使用了一个我们还未讲到的方法，rev，用来反转区间（range）:
    {
      for number in (1..4).rev() {
        println!("rev: {}!", number);
      }
      println!("LIFTOFF!!!");
      /*
      rev: 3!
      rev: 2!
      rev: 1!
      LIFTOFF!!!
      */
      // 这段代码更好一些，不是吗？
    }

  

}