// The dbg! macro and .inspect
// https://dhghomon.github.io/easy_rust/Chapter_38.html
// https://www.youtube.com/watch?v=qmtow7Hojtk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=M43XCULOAbA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// #![allow(unused)]

fn main() {
  // `dbg!` prints to standard error (stderr) and returns the value of a given expression for quick
  // and dirty debugging.

  // Invoking `dbg!(expr)` on an expression MOVES and TAKES OWNERSHIP of it
  // before returning the evaluated expression unchanged. If the type
  // of the expression does not implement `Copy` and you don't want
  // to give up ownership, you can instead borrow with `dbg!(&expr)`.

  {
    let my_number = 8;
    dbg!(my_number);
  }

  println!();

  {
    let mut my_number = dbg!(9);
    dbg!(my_number += 10);

    let new_vec = dbg!(vec![8, 9, 10]);

    let double_vec = dbg!(new_vec.iter().map(|x| x * 2).collect::<Vec<_>>());

    dbg!(double_vec);

    println!("{:?}", my_number);
  }

  println!();

  // inspect(…)

  {
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec.iter().map(|x| x * 2).collect::<Vec<_>>();

    println!("{:?}", double_vec);
  }

  println!();

  // While using chaining with iterators, you might want to check out what's happening at various
  // parts in the pipeline. To do that, insert a call to `inspect()`.

  {
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
      .iter()
      .inspect(|item| println!("The item is: {}", item))
      .map(|x| x * 2)
      .inspect(|item| println!("Then it is: {}", item))
      .collect::<Vec<_>>();

    println!("{:?}", double_vec);
  }

  println!();

  {
    let new_vec = vec![8, 9, 10];

    let double_vec = new_vec
      .iter()
      .inspect(|item| {
        println!("The item is: {}", item);
        dbg!(item); // goes to stderr
        match **item % 2 {
          0 => println!("It is even."),
          _ => println!("It is odd."),
        }
        println!("In binary it is {:b}.", item);
      })
      .map(|x| x * 2)
      .collect::<Vec<_>>();

    println!("{:?}", double_vec);
  }
}
