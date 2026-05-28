// Collection types - Arrays
// https://dhghomon.github.io/easy_rust/Chapter_20.html
// https://www.youtube.com/watch?v=Iuq3Cort3Eg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  let array1 = ["One", "Two"];
  let array2 = ["One", "Two", "Five"];

  println!("{}", array1[0]);
  println!("{}", array2[0]);
  // println!("{}", array1 == array2); // ⚠️ different types

  let seasons1 = ["Spring", "Summer", "Autumn", "Winter"];
  let seasons2 = ["Spring", "Summer", "Fall", "Autumn", "Winter"];

  println!("{}", seasons1[0]);
  println!("{}", seasons2[0]);
  // println!("{}", seasons1 == seasons2); // ⚠️ different types

  println!();

  let my_array = ["a"; 10];
  println!("{:?}", my_array);

  println!();

  let my_numbers = [10, 20, 30];
  println!("{}", my_numbers[0]);
  println!("{}", my_numbers[1]);
  println!("{}", my_numbers[2]);

  println!();

  let array_of_ten = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

  // slicing
  // type of these variables is `&[i32]`
  let three_to_five = &array_of_ten[2..5];
  let start_at_two = &array_of_ten[1..];
  let end_at_five = &array_of_ten[..5];
  let everything = &array_of_ten[..];

  println!("three to five: {:?}", three_to_five);
  println!("start at two:  {:?}", start_at_two);
  println!("end at five:   {:?}", end_at_five);
  println!("everything:    {:?}", everything);
}
