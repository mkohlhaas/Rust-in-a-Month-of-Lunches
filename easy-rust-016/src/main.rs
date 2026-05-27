// Mutable References
// https://dhghomon.github.io/easy_rust/Chapter_17.html
// https://www.youtube.com/watch?v=R13sQ8SNoEQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Rule 1: You can have as many immutable references as you want.
//         As many readers as you want at one point in time.
// Rule 2: You can have only one mutable reference.
//         Only one writer at one point in time! That's it! No readers at the same time!

fn main() {
  let mut my_number = 42;
  let num_ref = &mut my_number;

  println!("{}", num_ref);

  let mut my_number = 32;
  let num_ref = &mut my_number;
  *num_ref += 10;
  println!("{}", my_number);

  let second_number = 42;
  let triple_reference = &&&second_number;
  println!(
    "Second_number = triple_reference? {}",
    second_number == ***triple_reference
  );

  // let mut number = 42;
  // let number_ref = &number;        // immutable borrow occurs here [E0502]
  // let number_change = &mut number; // cannot borrow `number` as mutable because it is also borrowed as immutable
  // *number_change += 10;
  // println!("{}", number_ref);      // ⚠️ immutable borrow later used here [E0502]

  let mut number = 32;
  let number_change = &mut number; // create a mutable reference
  *number_change += 10; // use mutable reference to add 10

  // mutable reference not used any more: OK ✓
  let number_ref = &number; // create an immutable reference
  println!("{}", number_ref); // print the immutable reference

  let country = String::from("Austria");
  let country_ref = &country;

  // shadowing
  let country = 8;
  println!("{}, {}", country_ref, country);

  let country = String::from("Austria"); // Now we have a String called country
  let country_ref = &country; // country_ref is a reference to this data. It's not going to change.

  // shadowing
  let country = 8; // Now we have a variable called country that is an i8. But it has no relation to the other one, or to country_ref.
  println!("{}, {}", country_ref, country); // country_ref still refers to the data of String::from("Austria") that we gave it.
}
