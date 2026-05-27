// Strings
// https://dhghomon.github.io/easy_rust/Chapter_14.html
// https://www.youtube.com/watch?v=pSyaGzGg26o&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  let name = "서태지";
  println!("{}", name);

  let name = "서태지".to_string();
  println!("{}", name);

  let name = String::from("Adrian Fahrenheit Țepeș");
  println!("My name is {}.", name);

  let name = "😂";
  println!("My name is actually {}.", name);

  println!();

  // Module mem
  // https://doc.rust-lang.org/std/mem/index.html

  println!(
    "A String is always {:?} bytes. It is Sized.",
    std::mem::size_of::<String>()
  );

  println!(
    "An i8 is always {:?} bytes. It is Sized.",
    std::mem::size_of::<i8>()
  );

  println!(
    "An f64 is always {:?} bytes. It is Sized.",
    std::mem::size_of::<f64>()
  );

  println!(
    "But a &str? It can be anything. '서태지' is {:?} bytes. It is not Sized.",
    std::mem::size_of_val("서태지")
  );

  println!(
    "And 'Adrian Fahrenheit Țepeș' is {:?} bytes. It is not Sized.",
    std::mem::size_of_val("Adrian Fahrenheit Țepeș")
  );

  println!();

  let my_name = "Billy";
  let my_country = "USA";
  let my_home = "Korea";

  // creating a `String` using interpolation
  let together = format!(
    "I am {} and I come from {} but I live in {}.",
    my_name, my_country, my_home
  );
  println!("{}", together);

  println!();

  // let my_string = "Try to make this a String".into(); // ⚠️ type annotations needed

  let my_string: String = "Trying to make this a String.".into();
  println!("{}", my_string);

  let my_string: String = "Trying to make this a String.".to_string();
  println!("{}", my_string);

  let my_string: &str = "Trying to make this a &str.".into();
  println!("{}", my_string);
}
