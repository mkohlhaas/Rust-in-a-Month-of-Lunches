// The todo! macro
// https://dhghomon.github.io/easy_rust/Chapter_44.html
// https://www.youtube.com/watch?v=In0js0GOdvM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Another similar macro is `unimplemented!`.
//
// The difference between `unimplemented!` and `todo!` is that while `todo!`
// conveys an intent of implementing the functionality later and the message is "not yet
// implemented", `unimplemented!` makes no such claims. Its message is "not implemented".
//
// Also, some IDEs will mark `todo!`s.

#![allow(unused)]

struct Book {}

enum BookType {
  HardCover,
  SoftCover,
}

fn get_book(book: &Book) -> Option<String> {
  todo!()
}

fn delete_book(book: Book) -> Result<(), String> {
  unimplemented!(); // ⚠️ panics at runtime!
  // todo!() // ⚠️ panics at runtime!
}

// ⚠️ needs real type for compilation
// fn get_book(book: &Book) -> WorldsBestType {
//   todo!()
// }

// TODO: impl block and make these functions methods...

// let's make sure the match statement works …
fn check_book_type(book_type: &BookType) {
  match book_type {
    BookType::HardCover => println!("It's hardcover."),
    BookType::SoftCover => println!("It's softcover."),
  }
}

fn main() {
  let book_type = BookType::HardCover;
  check_book_type(&book_type);

  let my_book = Book {};
  delete_book(my_book);
}
