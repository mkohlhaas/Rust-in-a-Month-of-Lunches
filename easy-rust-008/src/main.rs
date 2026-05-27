// Mutability (changing) & Shadowing
// https://dhghomon.github.io/easy_rust/Chapter_11.html

fn times_two(number: i32) -> i32 {
  number * 2
}

fn main() {
  let _my_number = 8; // rustc: consider making this binding mutable: `mut ` [E0384]
  // _my_number = 10; // ⚠️ rust-analyzer: cannot mutate immutable variable `my_number` [E0384]

  let mut my_number = 8;
  println!("{}", my_number);
  my_number = 10;
  println!("{}", my_number);

  // let mut my_variable = 8;
  // my_variable = "Hello, world!"; // ⚠️ rust-analyzer: expected i32, found &'static str [E0308]

  let my_number = 8;
  println!("{}", my_number);
  let my_number = 9.2;
  println!("{}", my_number);

  let my_number = 8;
  println!("{}", my_number);

  {
    let my_number = 9.2;
    println!("{}", my_number)
  }
  println!("{}", my_number);

  let final_number = {
    let y = 10;
    let x = 9;
    let x = times_two(x);
    let x = x + y;
    x // 28
  };
  println!("The number is now: {}", final_number);

  // without shadowing we had to invent a lot of useless variable names
  let final_number = {
    let y = 10;
    let x = 9;
    let x_twice = times_two(x);
    let x_twice_and_y = x_twice + y;
    x_twice_and_y // 28
  };
  println!("The number is now: {}", final_number)
}
