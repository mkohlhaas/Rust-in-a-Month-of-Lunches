// Writing macros
// https://dhghomon.github.io/easy_rust/Chapter_61.html
// https://www.youtube.com/watch?v=nTHGbcI_te4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=174&pp=iAQB
// https://www.youtube.com/watch?v=sKKA9SV4hA0&list=PLfllocyHgsRwLkTAhG0E-2QxCf-ozBkk&index=175&pp=iAQB
// https://www.youtube.com/watch?v=E3ykobghMj8&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=176&pp=iAQB
// https://www.youtube.com/watch?v=7aY3NzriR_Y&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=177&pp=iAQB
// https://www.youtube.com/watch?v=T_mEAZEXHsg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=178&pp=iAQB

macro_rules! give_six {
  () => {
    6
  };
}

macro_rules! six_or_print {
  (6) => {
    6
  };
  () => {
    println!("You didn't give me 6.");
  };
}

macro_rules! might_print1 {
  (THis is strange input 하하はは哈哈 but it still works) => {
    println!("You guessed the secret message!")
  };
  () => {
    println!("You didn't guess it.");
  };
}

macro_rules! might_print2 {
  ($input:expr) => {
    println!("You gave me: {}", $input);
  };
}

macro_rules! might_print3 {
  ($input:expr) => {
    println!("You gave me: {:?}", $input); // debug output
  };
}

macro_rules! check {
  ($input1:ident, $input2:expr) => {
    println!(
      "Is {:?} equal to {:?}? {:?}",
      $input1,
      $input2,
      $input1 == $input2
    );
  };
}

// stringify
macro_rules! print_anything1 {
  ($input:tt) => {
    let output = stringify!($input);
    println!("{}", output);
  };
}

macro_rules! print_anything2 {
    ($($input1:tt),*) => {
        let output = stringify!($($input1),*);
        println!("{}", output);
    };
}

macro_rules! make_a_function {
    ($name:ident, $($input:tt),*) => {
        fn $name() {
            let output = stringify!($($input),*);
            println!("{}", output);
        }
    };
}

// macro calling itself
macro_rules! my_macro {
  () => {
    println!("Let's print this.");
  };
  ($input:expr) => {
    my_macro!();
  };
  ($($input:expr),*) => {
    my_macro!();
  };
}

fn main() {
  {
    let six = give_six!();

    println!("{}", six);
  }

  // {
  //   // ⚠️ rustc: `match` arms have incompatible types
  //   let my_number = 10;
  //   match my_number {
  //     10 => println!("You got a ten"),
  //     _ => 10,
  //   }
  // }

  {
    let _my_number = six_or_print!(6);
    six_or_print!();
  }

  println!();

  {
    might_print1!(THis is strange input 하하はは哈哈 but it still works);
    might_print1!();
  }

  println!();

  {
    // might_print2!(()); // `()` doesn't implement `std::fmt::Display`
    might_print2!(6);
    // might_print2!(vec![8, 9, 7, 10]); // `Vec<{integer}>` doesn't implement `std::fmt::Display`
  }

  println!();

  {
    might_print3!(());
    might_print3!(6);
    might_print3!(vec![8, 9, 7, 10]);
  }

  println!();

  {
    let x = 6;
    let my_vec = vec![7, 8, 9];

    check!(x, 6);
    check!(my_vec, vec![7, 8, 9]);
    check!(x, 10);
  }

  println!();

  {
    print_anything1!(ththdoetd);
    print_anything1!(87575oehq75onth);
  }

  println!();

  {
    print_anything2!(ththdoetd, rcofe);
    print_anything2!();
    print_anything2!(87575oehq75onth, ntohe, 987987o, 097);
  }

  println!();

  {
    make_a_function!(print_it, 5, 5, 6, I);
    print_it();

    make_a_function!(say_its_nice, this, is, really, nice);
    say_its_nice();
  }

  println!();

  {
    my_macro!(vec![8, 9, 0]);
    my_macro!(toheteh);
    my_macro!(8, 7, 0, 10);
    my_macro!();
  }
}
