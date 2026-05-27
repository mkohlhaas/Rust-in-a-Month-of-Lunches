// const and static
// https://dhghomon.github.io/easy_rust/Chapter_15.html
// https://www.youtube.com/watch?v=Ky3HqkWUcI0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// On the surface, `static` items seem very similar to `const`s: both contain
// a value, both require type annotations and both can only be initialized with
// constant functions and values. However, `static`s are notably different in
// that they represent a location in memory. That means that you can have
// references to `static` items and potentially even modify them, making them
// essentially global variables.

// Almost always you want to use `const`.

const NUMBER_OF_MONTHS1: u32 = 42;
const SEASONS2: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
// const TEST_STRING: String = "Test String".to_string(); // cannot call non-const method `<str as ToString>::to_string` in constants

static NUMBER_OF_MONTHS2: u32 = 42;
static SEASONS1: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
// static TEST_STRING: String = "Test String".to_string(); // cannot call non-const method `<str as ToString>::to_string` in statics

fn main() {
  // let NUMBER_OF_MONTHS1 = 48; // no shadowing allowed

  println!("{}", NUMBER_OF_MONTHS1);
  println!("{}", NUMBER_OF_MONTHS2);
  println!("{:p}", &NUMBER_OF_MONTHS1);
  println!("{:p}", &NUMBER_OF_MONTHS2);

  println!();

  println!("{}", SEASONS2.len());
  println!("{}", SEASONS2[0]);
  println!("{}", SEASONS2[1]);
  println!("{}", SEASONS2[2]);
  println!("{}", SEASONS2[3]);

  println!();

  println!("{}", SEASONS1.len());
  println!("{}", SEASONS1[0]);
  println!("{}", SEASONS1[1]);
  println!("{}", SEASONS1[2]);
  println!("{}", SEASONS1[3]);
}
