/// main() is where Rust programs start to run.
fn main() {
  let _first_letter = 'A';
  let _space = ' '; // a space inside ' ' is also a char
  let _other_language_char = 'Ꮔ'; // thanks to Unicode, other languages like Cherokee display just fine too
  let _cat_face = '😺'; // emojis are chars too

  // we didn't write a type of integer, so Rust chooses i32
  let my_number1 = 2147483647;
  // my_number1 += 1;

  // println!("{}", my_number1 as char); // ⚠️

  println!("{}", my_number1 as u8);
  println!("{}", my_number1 as u8 as char);

  let my_number2: u8 = 65;
  println!("{}", my_number2 as char);

  println!("Size of a char: {} bytes", std::mem::size_of::<char>());
  println!("Size of string containing 'a':  {} bytes", "a".len());
  println!("Size of string containing 'ß':  {} bytes", "ß".len());
  println!("Size of string containing '国': {} bytes", "国".len());
  println!("Size of string containing '𓅱':  {} bytes", "𓅱".len());

  let slice1 = "Hello!";
  println!("Slice1 is {} bytes.", slice1.len());

  let slice2 = "안녕!"; // Korean for "hi"
  println!("Slice2 is {} bytes.", slice2.len());

  let slice3 = "Hello!";
  println!(
    "Slice is {} bytes and also {} characters.",
    slice3.len(),
    slice3.chars().count()
  );
  let _fu = slice3.chars();

  let slice4 = "안녕!";
  println!(
    "Slice4 is {} bytes but only {} characters.",
    slice4.len(),
    slice4.chars().count()
  );
}
