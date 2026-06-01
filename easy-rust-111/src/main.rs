// Cow
// https://dhghomon.github.io/easy_rust/Chapter_42.html
// https://www.youtube.com/watch?v=vO0KJRXDFKk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=q27L69kCwu4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Cow - a clone-on-write smart pointer

use std::borrow::Cow;

fn mod3(n: u8) -> Cow<'static, str> {
  match n % 3 {
    0 => "remainder is 0".into(),                              // Borrowed
    1 => "remainder is 1".into(),                              // Borrowed
    remainder => format!("remainder is {}", remainder).into(), // Owned
  }
}

fn return_slice_or_vec<'a>(input: &'a [i32]) -> Cow<'a, [i32]> {
  match input.len() {
    0..5 => Cow::Owned(input.to_vec()),
    _ => Cow::Borrowed(input),
  }
}

fn main() {
  {
    for n in 0..10 {
      match mod3(n) {
        Cow::Borrowed(msg) => println!("{} went in. The Cow is borrowed: {}", n, msg),
        Cow::Owned(msg) => println!("{} went in. The Cow is owned: {}", n, msg),
      }
    }
  }

  println!();

  {
    let x = &[1, 2, 3, 4, 5, 6, 7, 8, 9][2..7];
    println!("{:?}", x);
  }

  println!();

  {
    let x1 = return_slice_or_vec(&[1, 2, 3, 4]);
    let x2 = return_slice_or_vec(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);

    println!("{:?}", x1);
    println!("{:?}", x2);
  }
}
