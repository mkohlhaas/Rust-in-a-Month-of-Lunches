// More on references
// https://dhghomon.github.io/easy_rust/Chapter_16.html
// https://www.youtube.com/watch?v=R13sQ8SNoEQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  let country = String::from("Austria");

  // any number of references pose no problem
  let ref_one = &country;
  let ref_two = &country;

  println!("{}", ref_one);
  println!("{}", ref_two);

  println!();

  // let country = get_country();
}

// rustc: this function's return type contains a borrowed value, but there is no value for it to be borrowed from [E0106]
// fn get_country() -> &'static str {
//   let country = String::from("Austria");
//   // rustc: cannot return reference to local variable `country`
//   // rustc: returns a reference to data owned by the current function [E0515]
//   &country // ⚠️ reference to a gone var
// }
