// Control flow
// https://dhghomon.github.io/easy_rust/Chapter_23.html
// https://www.youtube.com/watch?v=UAymDOpv_us&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=eqysTfiiQZs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// multiple wildcards and guards
fn match_colors(rgb: (i32, i32, i32)) {
  match rgb {
    (r, _, _) if r < 10 => println!("Not much red."),
    (_, g, _) if g < 10 => println!("Not much green."),
    (_, _, b) if b < 10 => println!("Not much blue."),
    _ => println!("Each color has at least 10."),
  }
}

// naming a match arm/branch value with @
fn match_number(input: i32) {
  match input {
    n @ 4 => println!("{} is an unlucky number in China - sounds close to 死!", n),
    n @ 13 => println!("{} is an unlucky in North America! In bocca al lupo!", n),
    _ => println!("Looks like a normal number."),
  }
}

fn main() {
  // if
  {
    let my_number = 5;
    if my_number == 5 {
      println!("It's five.");
    }
  }

  // if … else if … else
  {
    let my_number = 5;

    if my_number == 7 {
      println!("It's seven");
    } else if my_number == 6 {
      println!("It's six")
    } else {
      println!("It's a different number.")
    }
  }

  // logical operators (e.g. &&, ||)
  {
    let my_number = 5;

    if my_number % 2 == 1 && my_number > 0 {
      println!("It's a positive odd number.");
    } else if my_number == 6 {
      println!("It's six")
    } else {
      println!("It's a different number.")
    }
  }

  // exhaustive pattern
  // {
  //   let my_number: u8 = 5;
  //   match my_number {
  //     0 => println!("it's zero"),
  //     1 => println!("it's one"),
  //     2 => println!("it's two"), // ⚠️ non-exhaustive patterns: `3_u8..=u8::MAX` not covered
  //                                // ensure that all possible cases are being handled by adding a match arm with
  //                                // a wildcard pattern or an explicit pattern as shown: `, 3_u8..=u8::MAX => todo!()` [E0004]
  //   }
  // }

  // wildcards
  {
    let my_number: u8 = 5;
    match my_number {
      0 => println!("It's zero."),
      1 => println!("It's one."),
      2 => println!("It's two."),
      _ => println!("It's some other number."),
    }
  }

  // match is an expression
  {
    let my_number = 5;
    let another_number = match my_number {
      0 => 0,
      5 => 10,
      _ => 2,
    };
    println!("{}", another_number);
  }

  // matching on tuples
  {
    let sky = "cloudy";
    let temperature = "warm";

    match (sky, temperature) {
      ("cloudy", "cold") => println!("It's dark and unpleasant today."),
      ("clear", "warm") => println!("It's a nice day."),
      ("cloudy", "warm") => println!("It's dark but not bad."),
      _ => println!("Not sure what the weather is."),
    }
  }

  // guards
  {
    let children = 5;
    let married = true;

    match (children, married) {
      (children, married) if married == false => {
        println!("Not married with {} children.", children)
      }
      (children, married) if children == 0 && married == true => {
        println!("Married but no children.")
      }
      _ => println!("Married? {}. Number of children: {}.", married, children),
    }
  }

  {
    let color1 = (200, 0, 0);
    let color2 = (50, 50, 50);
    let color3 = (200, 50, 0);

    // multiple wildcards and guards
    match_colors(color1);
    match_colors(color2);
    match_colors(color3);
  }

  // match has to return same type in every arm/branch
  // {
  //   let my_number = 10;
  //   let some_variable = match my_number {
  //     10 => 8,
  //     _ => "not ten", // ⚠️ rustc: `match` arms have incompatible types
  //   };
  // }

  // same with if … else
  // {
  //   let some_variable = if my_number == 10 {
  //     8
  //   } else {
  //     "something else "
  //   }; // ⚠️ expected i32, found &'static str [E0308]
  // }

  // naming a match arm/branch value with @
  {
    match_number(50);
    match_number(13);
    match_number(4);
  }
}
