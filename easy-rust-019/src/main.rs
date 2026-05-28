// Copy types
// https://dhghomon.github.io/easy_rust/Chapter_19.html
// https://www.youtube.com/watch?v=g0QM2wM1X5o&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=20

fn prints_number(number: i32) {
  // There is no -> so it's not returning anything.
  // If number was not copy type, it would take it and we couldn't use it again
  println!("{}", number);
}

fn prints_country(country_name: String) {
  println!("{}", country_name);
}

// takes ownership of a String
fn get_length(input: String) {
  println!("Number of words: {}", input.split_whitespace().count()); // splits to count the number of words
}

fn main() {
  let my_number = 42;
  prints_number(my_number);
  prints_number(my_number);

  println!();

  let country = String::from("Kiribati");
  prints_country(country);
  // prints_country(country); // ⚠️ rustc: use of moved value: `country`

  println!();

  let country = String::from("Kiribati");
  prints_country(country.clone());
  prints_country(country.clone());
  prints_country(country.clone());
  prints_country(country.clone());
  prints_country(country.clone());
  prints_country(country);
  // prints_country(country); // ⚠️ rustc: use of moved value: `country`

  println!();

  let mut my_string = String::new();
  for _ in 0..50 {
    my_string.push_str("whatever ");
    get_length(my_string.clone());
  }
  println!("\n{}", my_string);
}
