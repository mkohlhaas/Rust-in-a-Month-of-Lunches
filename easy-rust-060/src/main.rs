// The question mark (?) operator (aka try)
// https://dhghomon.github.io/easy_rust/Chapter_33.html
// https://www.youtube.com/watch?v=XvXlrcESzjY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=Q6LZ4KzwZfw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=LfpILBEN6fE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(dead_code)]

use std::num::ParseIntError;

fn parse_str1(input: &str) -> Result<i32, ParseIntError> {
  let parsed_number = input.parse::<i32>()?;
  Ok(parsed_number)
}

// `?` allows Happy-Path-Programming
fn parse_str2(input: &str) -> Result<u32, ParseIntError> {
  let parsed_number = input
    .parse::<u16>()?
    .to_string()
    .parse::<u8>()?
    .to_string()
    .parse::<u32>()?;
  Ok(parsed_number)
}

fn prints_three_things1(vector: Vec<i32>) {
  println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn prints_three_things2(vector: Vec<i32>) {
  if vector.len() != 3 {
    panic!("Vector must always have three items!")
  }
  println!("{}, {}, {}", vector[0], vector[1], vector[2]);
}

fn get_fourth1(input: &Vec<i32>) -> i32 {
  let fourth = input.get(3).unwrap(); // ⚠️ risky: unwrap can panic if given a None
  *fourth
}

// ⚠️ Can still panic but at least we have a customized error message.
fn get_fourth2(input: &Vec<i32>) -> i32 {
  let fourth = input.get(3).expect("Input vector needs at least 4 items.");
  *fourth
}

fn try_two_unwraps1(input: Vec<Option<i32>>) {
  println!("Index 0 is: {}", input[0].unwrap());
  println!("Index 1 is: {}", input[1].unwrap());
}

fn try_two_unwraps2(input: Vec<Option<i32>>) {
  println!(
    "Index 0 is: {}",
    input[0].expect("The first unwrap had a None!")
  );
  println!(
    "Index 1 is: {}",
    input[1].expect("The second unwrap had a None!")
  );
}

fn main() {
  match parse_str1("42") {
    Ok(n) => println!("{}", n),
    Err(_) => println!("Couldn't parse string!"),
  }

  match parse_str1("42fu") {
    Ok(n) => println!("{}", n),
    Err(_) => println!("Couldn't parse string!"),
  }

  println!();

  {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
      let parsed = parse_str1(item);
      println!("{:?}", parsed);
    }
  }

  // {
  //   // finding the Result type
  //   let failure = "Not a number".parse::<i32>();
  //   failure.rbrbrb(); // ⚠️ method not found in `Result<i32, ParseIntError>` [E0599]
  // }

  println!();

  {
    let str_vec = vec!["Seven", "8", "9.0", "nice", "6060"];
    for item in str_vec {
      let parsed = parse_str2(item);
      println!("{:?}", parsed);
    }
  }

  // {
  //   panic!("Time to panic!");
  // }

  println!();

  {
    let my_vec = vec![8, 9, 10];
    prints_three_things1(my_vec);
  }

  {
    let my_vec = vec![8, 9, 10, 10, 55, 99];
    prints_three_things1(my_vec);
  }

  {
    let my_vec = vec![8, 9, 10];
    prints_three_things2(my_vec);
  }

  // panics!
  // {
  //   let my_vec = vec![8, 9, 10, 10, 55, 99];
  //   prints_three_things2(my_vec);
  // }

  println!();

  {
    let my_name = "Loki Laufeyson";

    // `assert's` panic if not fullfilled!
    assert!(my_name == "Loki Laufeyson");
    assert_eq!(my_name, "Loki Laufeyson");
    assert_ne!(my_name, "Mithridates");
  }

  {
    let my_name = "Loki Laufeyson";

    // with error messages
    assert!(
      my_name == "Loki Laufeyson",
      "{} should be Loki Laufeyson",
      my_name
    );

    assert_eq!(
      my_name, "Loki Laufeyson",
      "{} and Loki Laufeyson should be equal",
      my_name
    );

    assert_ne!(
      my_name, "Mithridates",
      "You entered {}. Input must not equal Mithridates",
      my_name
    );
  }

  // panics!
  // {
  //   let my_name = "Mithridates";
  //
  //   assert_ne!(
  //     my_name, "Mithridates",
  //     "You enter {}. Input must not equal Mithridates",
  //     my_name
  //   );
  // }

  // panics!
  // {
  //   let my_vec = vec![9, 0, 10];
  //   let fourth = get_fourth(&my_vec);
  //   println!("{}", fourth);
  // }

  // panics!
  // {
  //   let my_vec = vec![9, 0, 10];
  //   let fourth = get_fourth2(&my_vec);
  //   println!("{}", fourth);
  // }

  // panics!
  // {
  //   let vector = vec![None, Some(1000)];
  //   try_two_unwraps1(vector);
  // }

  // panics!
  // {
  //   let vector = vec![None, Some(1000)];
  //   try_two_unwraps2(vector);
  // }

  // unwrap_or
  {
    let my_vec = vec![8, 9, 10];
    let fourth1 = my_vec.get(3).unwrap_or(&0);
    println!("{}", fourth1);
  }
}
