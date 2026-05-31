// Types of &str
// https://dhghomon.github.io/easy_rust/Chapter_39.html
// https://www.youtube.com/watch?v=bnUd_KN-RXQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Rust will convert a `&String` to a `&str` when needed because it implements the Deref trait.
fn prints_str(my_str: &str) {
  println!("{}", my_str);
}

fn main() {
  let my_string: String = String::from("I am a string.");
  prints_str(&my_string);
}
