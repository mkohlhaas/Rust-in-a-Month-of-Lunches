// Option and Result
// https://dhghomon.github.io/easy_rust/Chapter_31.html
// https://www.youtube.com/watch?v=wb8ez9raMDY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=GKnbGUX7OB4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=LxjLR3zJQ0o&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=hyPbjVRSu4Y&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=EbtvILqrUcg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=L3xYMTmjnKM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=smex41M4CRw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=x2pmYS41cd0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=LslG4GTQXfY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// ⚠️ panics at run-time (index out of bounds: the len is 2 but the index is 4)
// fn take_fifth1(value: Vec<i32>) -> i32 {
//   value[4]
// }

fn take_fifth2(value: Vec<i32>) -> Option<i32> {
  if value.len() < 5 {
    None
  } else {
    Some(value[4])
  }
}

fn handle_option(my_options: Vec<Option<i32>>) {
  for item in my_options {
    match item {
      Some(number) => println!("Found Some {}!", number),
      None => println!("Found None!"),
    }
  }
}

fn check_error() -> Result<(), ()> {
  Ok(())
}

fn is_even(input: i32) -> Result<(), ()> {
  if input % 2 == 0 { Ok(()) } else { Err(()) }
}

fn check_if_five(number: i32) -> Result<i32, i32> {
  match number {
    5 => Ok(number),
    _ => Err(number),
  }
}

fn main() {
  // ⚠️ panics!
  // {
  //   let new_vec = vec![1, 2];
  //   let index = take_fifth1(new_vec);
  //   println!("{}", index);
  // }

  // using Option
  {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    println!("{:?}", take_fifth2(new_vec));
    println!("{:?}", take_fifth2(bigger_vec));
  }

  println!();

  // unwrap panics in the presence of None
  // ⚠️ panics!
  // {
  //   let new_vec = vec![1, 2];
  //   let bigger_vec = vec![1, 2, 3, 4, 5];
  //   println!(
  //     "{:?}, {:?}",
  //     take_fifth2(new_vec).unwrap(), // panics!
  //     take_fifth2(bigger_vec).unwrap()
  //   );
  // }

  // expects also panics but will at least print an error message
  // {
  //   let new_vec = vec![1, 2];
  //   let bigger_vec = vec![1, 2, 3, 4, 5];
  //   println!(
  //     "{:?}, {:?}",
  //     take_fifth2(new_vec).expect("Vector is too short!"),
  //     take_fifth2(bigger_vec).unwrap(),
  //   );
  // }

  {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let mut option_vec: Vec<Option<i32>> = Vec::new();

    option_vec.push(take_fifth2(new_vec)); // None
    option_vec.push(take_fifth2(bigger_vec)); // Some(5)

    handle_option(option_vec);
  }

  println!();

  {
    let new_vec = vec![1, 2];
    let bigger_vec = vec![1, 2, 3, 4, 5];
    let vec_of_vecs = vec![new_vec, bigger_vec];
    for vec in vec_of_vecs {
      let inside_number = take_fifth2(vec);
      if inside_number.is_some() {
        println!("We got: {}", inside_number.unwrap());
      } else {
        println!("We got nothing.");
      }
    }
  }

  println!();

  {
    match check_error() {
      Ok(_) => println!("Everything OK! ✓"),
      Err(_) => println!("Failure!"),
    }
  }

  println!();

  {
    if is_even(4).is_ok() {
      println!("It's okay!")
    } else {
      println!("It's an error!")
    }

    if is_even(5).is_ok() {
      println!("It's okay!")
    } else {
      println!("It's an error!")
    }
  }

  println!();

  {
    let mut result_vec = Vec::new();

    for number in 1..=7 {
      result_vec.push(check_if_five(number));
    }

    println!("{:?}", result_vec);
  }

  // unwrap panics in the presence of Err
  // ⚠️ panics!
  // {
  //   let error_value: Result<i32, _> = Err("There was an error");
  //   println!("{}", error_value.unwrap());
  // }

  println!();

  {
    let my_vec = vec![2, 3, 4];
    let get_one = my_vec.get(0);
    let get_two = my_vec.get(10);
    println!("{:?}", get_one);
    println!("{:?}", get_two);
  }

  println!();

  {
    let my_vec = vec![2, 3, 4];

    for index in 0..10 {
      match my_vec.get(index) {
        Some(number) => println!("The number is: {}", number),
        None => (),
      }
    }
  }

  println!();

  // `if let` for when you don't care about matching for every case
  // `if let` looks like backwards pattern matching
  // `let` in Rust is pattern matching (let x = 42;)
  {
    let my_vec = vec![2, 3, 4];

    // only interested in Some's
    for index in 0..10 {
      if let Some(number) = my_vec.get(index) {
        println!("The number is: {}", number);
      }
    }
  }

  println!();

  // `while let`
  {
    let weather_vec = vec![
      vec!["Berlin", "cloudy", "5", "-7", "78"],
      vec!["Athens", "sunny", "not humid", "20", "10", "50"],
    ];

    for mut city in weather_vec {
      println!("For the city of {}:", city[0]);
      while let Some(information) = city.pop() {
        // turbofish `::<…>`
        // https://rust.code-maven.com/turbofish
        if let Ok(number) = information.parse::<i32>() {
          println!("The number is: {}", number);
        }
      }
    }
  }
}
