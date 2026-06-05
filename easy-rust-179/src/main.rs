// cargo
// https://dhghomon.github.io/easy_rust/Chapter_62.html
// https://www.youtube.com/watch?v=7LeIoU85XCI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=179&pp=iAQB

use std::fmt::Display;

fn print_and_return_thing<T: Display>(input: T) -> T {
  println!("You gave me {} and now I will give it back.", input);
  input
}

fn main() {
  let my_name = print_and_return_thing("Windy");
  let small_number = print_and_return_thing(9.0);

  println!("{:?}", my_name);
  println!("{:?}", small_number);
}
