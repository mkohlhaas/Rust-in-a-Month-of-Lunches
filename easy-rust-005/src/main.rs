// Printing 'hello, world!'
// https://dhghomon.github.io/easy_rust/Chapter_9.html

fn number1() -> i64 {
  8
}

// fn number2() -> i32 {
//   8; // ⚠️ rustc: remove this semicolon to return this value [E0308]
// }

fn number3() -> () {
  8;
}

// no return value (`multiply1` is just for side-effects)
fn multiply1(number_one: i32, number_two: i32) {
  let result = number_one * number_two;
  println!("{} times {} is {}", number_one, number_two, result);
}

fn multiply2(number_one: i32, number_two: i32) -> i32 {
  let result = number_one * number_two;
  println!("{} times {} is {}", number_one, number_two, result);
  result
}
fn main() {
  println!("Hello, world!");
  println!("Hello, world number {}!", 8);
  println!("Hello, worlds number {} and {}!", 8, 9);
  println!("Hello, world number {}!", number1());
  println!("Hello, world number {}", number1());
  println!("Hello, world number {:?}", number3());

  multiply1(8, 9);

  let some_number = 9;
  let some_other_number = 2;
  multiply1(some_number, some_other_number);

  let multiply_result = multiply2(8, 9);
  println!("{}", multiply_result);

  let my_number = 8;
  println!("Hello, number {}", my_number);

  {
    let my_number = 88; // my_number starts here
    println!("{}", my_number);
  } // my_number ends here!

  println!("Hello, number {}", my_number);

  let my_number = {
    let second_number = 8_u32;
    second_number + 9 // no semicolon, so the code block returns 8 + 9
  };

  println!("My number is: {}", my_number);

  let my_number = {
    let second_number = 8;
    second_number + 9; // we don't return it!
  };

  println!("My number is: {:?}", my_number); // "my_number is ()" !!!

  // rustc: `()` doesn't implement `std::fmt::Display`
  // rustc: the trait `std::fmt::Display` is not implemented for `()`
  // rustc: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead [E0277]
  // println!("My number is: {}", my_number);
}
