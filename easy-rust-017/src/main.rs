// Giving references to functions
// https://dhghomon.github.io/easy_rust/Chapter_18.html
// https://www.youtube.com/watch?v=mKWXt9YTavc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=kJV1wIvAbyk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn print_country1(country_name: String) {
  // rustc: consider changing this parameter type in function `print_country` to borrow instead if owning the value isn't necessary [E0382]
  println!("{}", country_name);
}

fn print_country2(country_name: String) -> String {
  println!("{}", country_name);
  country_name // return it here
}

fn print_country3(country_name: &String) {
  println!("{}", country_name);
}

fn add_hungary(country_name: &mut String) {
  country_name.push_str("-Hungary");
  println!("Now it says: {}", country_name);
}

// Here's how: adds_hungary takes the String and declares it mutable!
// Takes ownership!
fn adds_hungary(mut country_name: String) {
  country_name.push_str("-Hungary");
  println!("{}", country_name);
}
fn main() {
  let country = String::from("Austria"); // rustc: move occurs because `country` has type `String`, which does not implement the `Copy` trait [E0382]
  print_country1(country); // rustc: value moved here [E0382]
  // print_country(country); // ⚠️ rustc: use of moved value: `country`

  println!();

  let country = String::from("Austria");
  let country = print_country2(country);
  print_country2(country);
  // print_country2(country); // ⚠️ rustc: use of moved value: `country`

  println!();

  let country = String::from("Austria");
  print_country3(&country);
  print_country3(&country);
  print_country3(&country);
  print_country3(&country);
  print_country3(&country);

  println!();

  let mut country = String::from("Austria");
  add_hungary(&mut country);
  println!("{}", country);

  println!();

  // country is not mutable, but we are going to print Austria-Hungary. How?
  let country = String::from("Austria");
  adds_hungary(country);
  // println!("{}", country); // ⚠️ rustc: borrow of moved value: `country`
}
