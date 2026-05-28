// Loops
// https://dhghomon.github.io/easy_rust/Chapter_26.html
// https://www.youtube.com/watch?v=-qg8wpJdSHY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=5kxpSr2p_ao&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=gX53Qr-hQ28&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn print_matching_colors(rgb: (i32, i32, i32)) {
  println!(
    "Color with {} red, {} green, and {} blue:",
    rgb.0, rgb.1, rgb.2
  );

  let new_vec = vec![(rgb.0, "red"), (rgb.1, "green"), (rgb.2, "blue")];
  let mut all_have_at_least_10 = true;

  for item in new_vec {
    if item.0 < 10 {
      all_have_at_least_10 = false;
      println!("Not much {}.", item.1)
    }
  }

  if all_have_at_least_10 {
    println!("Each color has at least 10.")
  }

  println!();
}

fn main() {
  // this program will never stop
  // loop {}

  // loop
  {
    let mut counter = 0;

    loop {
      counter += 1;
      println!("counter: {}", counter);
      if counter == 5 {
        break;
      }
    }
  }

  println!();

  // nested loops
  {
    let mut counter1 = 0;
    let mut counter2 = 0;

    println!("Entering the first loop.");

    // first loop
    'first_loop: loop {
      counter1 += 1;

      println!("first counter: {}", counter1);

      if counter1 > 9 {
        println!("Entering the second loop.");

        // second loop
        loop {
          println!("second counter: {}", counter2);
          counter2 += 1;
          if counter2 == 3 {
            break 'first_loop;
          }
        }
      }
    }
  }

  println!();

  // while loop
  {
    let mut counter = 0;

    while counter < 5 {
      counter += 1;
      println!("counter: {}", counter);
    }
  }

  println!();

  // for loop with a range
  {
    // exclusive range
    for number in 0..3 {
      println!("number: {}", number);
    }

    println!();

    // inclusive range
    for number in 0..=3 {
      println!("number: {}", number);
    }
  }

  println!();

  // using wildcards
  {
    for _ in 0..3 {
      println!("Printing the same thing three times.");
    }
  }

  println!();

  {
    for _n in 0..3 {
      println!("Printing the same thing three times.");
    }
  }

  println!();

  // break returns a value
  {
    let mut counter = 5;
    let my_number = loop {
      counter += 1;
      if counter % 53 == 3 {
        break counter;
      }
    };
    println!("{}", my_number);
  }

  println!();

  {
    let color1 = (200, 0, 0);
    let color2 = (50, 50, 50);
    let color3 = (200, 50, 0);

    print_matching_colors(color1);
    print_matching_colors(color2);
    print_matching_colors(color3);
  }
}
