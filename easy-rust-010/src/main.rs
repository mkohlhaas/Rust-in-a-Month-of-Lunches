// The Stack, the Heap, and Pointers
// https://dhghomon.github.io/easy_rust/Chapter_12.html

fn main() {
  let my_number = 42;
  let single_reference = &my_number;
  let double_reference = &single_reference;
  let five_references = &&&&&my_number;
  println!("{}", double_reference);
  println!("{}", five_references);
  println!("{}", my_number == **double_reference);
  println!("{}", double_reference == ***five_references);
}
