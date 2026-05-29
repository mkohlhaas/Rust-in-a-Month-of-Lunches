// Generics
// https://dhghomon.github.io/easy_rust/Chapter_30.html
// https://www.youtube.com/watch?v=K3EbxHmVByM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ljcXsogCMSU&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=IYXby69VMrU&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=9jHr72qeAh0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn return_number1(number: i32) -> i32 {
  print!("Here is your number: ");
  number
}

fn return_number2<T>(number: T) -> T {
  print!("Here is your number: ");
  number
}

// fn print_number<T>(number: T) {
//   println!("Here is your number: {:?}", number); // ⚠️ `T` cannot be formatted using `{:?}` because it doesn't implement `Debug` [E0277]
// }

use std::fmt::{Debug, Display};

fn print_number<T: Debug>(number: T) {
  println!("Here is your number: {:?}", number);
}

#[derive(Debug)]
struct Animal {
  name: String,
  age: u8,
}

fn print_item<T: Debug>(item: T) {
  println!("Here is your item: {:?}", item);
}

use std::cmp::PartialOrd;

fn compare_and_display1<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
  println!(
    "{}, is {} greater than {}? {}",
    statement,     // Display
    num_1,         // Display
    num_2,         // Display
    num_1 > num_2  // PartialOrd
  );
}

// with where clause
fn compare_and_display2<T, U>(statement: T, num_1: U, num_2: U)
where
  T: Display,
  U: Display + PartialOrd,
{
  println!(
    "{}, is {} greater than {}? {}",
    statement,
    num_1,
    num_2,
    num_1 > num_2
  );
}

fn say_two<T: Display, U: Display>(statement_1: T, statement_2: U) {
  println!(
    "I have two things to say: {} and {}",
    statement_1, statement_2
  );
}

fn main() {
  {
    let number = return_number1(5);
    // let number = return_number1(5.); // ⚠️ rust-analyzer: expected i32, found f64 [E0308]
    println!("{}", number);
  }

  {
    let number = return_number2(5);
    println!("{}", number);

    let number = return_number2(5.5);
    println!("{}", number);
  }

  // {
  //   print_number(5);
  // }

  {
    print_number(5);
  }

  println!();

  {
    // print a number
    let number = 55;
    print_item(number);

    // print an Animal
    let charlie = Animal {
      name: "Charlie".to_string(),
      age: 42,
    };
    println!("{}", charlie.name);
    println!("{}", charlie.age);
    print_item(charlie);
  }

  println!();

  {
    compare_and_display1("Listen up", 9, 8);
    compare_and_display1("Listen up".to_string(), 9, 8);
  }

  println!();

  {
    compare_and_display2("Listen up", 9, 8);
    compare_and_display2("Listen up".to_string(), 9, 8);
  }

  println!();

  {
    // different types
    say_two("Hello there", String::from("I hate sand."));

    // same types
    say_two(
      String::from("This is Padme"),
      String::from("is she all right?"),
    );
  }
}
