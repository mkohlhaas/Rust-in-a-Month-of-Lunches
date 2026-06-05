// Taking user input
// https://dhghomon.github.io/easy_rust/Chapter_63.html
// https://www.youtube.com/watch?v=JkJxRn1OnWA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=afGsq8rV41k&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=IO0l89Hz4Is&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::env::args;
use std::io;

enum Letters {
  Capitalize,
  Lowercase,
  Nothing,
}

fn main() {
  {
    println!("Please type something, or x to escape:");
    let mut input_string = String::new();

    while input_string.trim() != "x" {
      input_string.clear();
      io::stdin().read_line(&mut input_string).unwrap();
      println!("You wrote {:?}", input_string);
    }
    println!("See you later!");
  }

  {
    println!("{:?}", std::env::args());
  }

  {
    let input = args();

    for entry in input {
      println!("You entered: {}", entry);
    }
  }

  println!();

  {
    let input = args();

    input.skip(1).for_each(|item| {
      println!(
        "You wrote {}, which in capital letters is {}",
        item,
        item.to_uppercase()
      );
    })
  }

  println!();

  {
    let mut changes = Letters::Nothing;
    let input = args().collect::<Vec<_>>();

    if input.len() > 2 {
      match input[1].as_str() {
        "capital" => changes = Letters::Capitalize,
        "lowercase" => changes = Letters::Lowercase,
        _ => {}
      }
    }

    for word in input.iter().skip(2) {
      match changes {
        Letters::Capitalize => println!("{}", word.to_uppercase()),
        Letters::Lowercase => println!("{}", word.to_lowercase()),
        _ => println!("{}", word),
      }
    }
  }

  println!();

  {
    for item in std::env::vars() {
      println!("{:?}", item);
    }
  }

  println!();

  {
    println!("{}", env!("USER"));
    println!("{}", option_env!("ROOT").unwrap_or("Can't find ROOT."));
    println!("{}", option_env!("CARGO").unwrap_or("Can't find CARGO."));
  }
}
