// Box & Box around traits
// https://dhghomon.github.io/easy_rust/Chapter_53.html
// https://dhghomon.github.io/easy_rust/Chapter_54.html
// https://www.youtube.com/watch?v=fUAJshw0C7I&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=129&pp=iAQB
// https://www.youtube.com/watch?v=JxocxLwVoMk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=130&pp=iAQB
// https://www.youtube.com/watch?v=oLuqAG-kGS4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=131&pp=iAQB
// https://www.youtube.com/watch?v=ZQNbyna2O04&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=132&pp=iAQB

// Box is a smart pointer with data on the heap.
//
// Generics      = impl Trait
// Trait objects = dyn Trait

#![allow(unused)]

use std::error::Error;
use std::fmt::{self, Display};
use std::mem::size_of;

// Takes anything and drops it.
fn just_takes_a_variable<T>(_item: T) {}

// recursive data structure
#[derive(Debug)]
struct List {
  item: Option<Box<List>>,
}

impl List {
  fn new() -> List {
    List {
      item: Some(Box::new(List { item: None })),
    }
  }

  fn empty() -> List {
    List { item: None }
  }
}

// TODO: don't know how to do this!
//
// struct List<T: Default> {
//   next: Option<Box<List<T>>>, // List with no content
//   item: T,
// }

// impl List<T> {
//   fn new<T>() -> List<T> {
//     List<T> {
//       next: None,
//       item: Default::default(),
//     }
//   }
// }

trait JustATrait {}

enum EnumOfNumbers {
  I8(i8),
  AnotherI8(i8),
  OneMoreI8(i8),
}

struct StructOfNumbers {
  an_i8: i8,
  another_i8: i8,
  one_more_i8: i8,
}

enum EnumOfOtherTypes {
  I8(i8),
  AnotherI8(i8),
  Collection(Vec<String>),
}

struct StructOfOtherTypes {
  an_i8: i8,
  another_i8: i8,
  a_collection: Vec<String>,
}

struct ArrayAndI8 {
  array: [i8; 1000],
  an_i8: i8,
  in_u8: u8,
}

impl JustATrait for EnumOfNumbers {}
impl JustATrait for StructOfNumbers {}
impl JustATrait for EnumOfOtherTypes {}
impl JustATrait for StructOfOtherTypes {}
impl JustATrait for ArrayAndI8 {}

fn returns_just_a_trait1() -> impl JustATrait {
  println!("impl");

  if true {
    EnumOfNumbers::I8(8)
  } else {
    // EnumOfOtherTypes::I8(8) // ⚠️ must be the same types
    EnumOfNumbers::I8(42)
  }
}

// ⚠️ rustc: return type cannot be a trait object without pointer indirection
// fn returns_just_a_trait() -> dyn JustATrait {
//   println!("impl");
//
//   if true {
//     EnumOfNumbers::I8(8)
//   } else {
//     // EnumOfOtherTypes::I8(8)
//     EnumOfNumbers::I8(42)
//   }
// }

fn returns_just_a_trait2() -> Box<(impl JustATrait)> {
  println!("Box impl");

  if true {
    Box::new(EnumOfNumbers::I8(8))
  } else {
    // Box::new(EnumOfOtherTypes::I8(8)) // ⚠️ must be the same types
    Box::new(EnumOfNumbers::I8(42))
  }
}

fn returns_just_a_trait3() -> Box<(dyn JustATrait)> {
  println!("dynamic");

  if true {
    Box::new(EnumOfNumbers::I8(8))
  } else {
    Box::new(EnumOfOtherTypes::I8(8)) // now we can have different types
  }
}

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {}

impl Display for ErrorOne {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "You got the first error!")
  }
}

#[derive(Debug)]
struct ErrorTwo;

impl Error for ErrorTwo {}

impl Display for ErrorTwo {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "You got the second error!")
  }
}

// With `Box<dyn Error>` you can return anything that has the Error trait.
// `dyn` stands for dynamic dispatch.
fn returns_errors1(input: u8) -> Result<String, Box<dyn Error>> {
  match input {
    0 => Err(Box::new(ErrorOne)),
    1 => Err(Box::new(ErrorTwo)),
    _ => Ok("Looks fine to me.".to_string()),
  }
}

// ⚠️ does not work!
// fn returns_errors2(input: u8) -> Result<String, impl Error> {
//   match input {
//     0 => Err(ErrorOne),
//     1 => Err(ErrorTwo),
//     _ => Ok("Looks fine to me".to_string()),
//   }
// }

fn main() {
  {
    // Stack overflow with Boxed array
    // https://github.com/rust-lang/rust/issues/53827

    // stack overflow at run-time
    // let kiste = [0; 3_000_000];

    // ⚠️ still stack overflow at run-time!!!
    // let kiste = Box::new([0; 3_000_000]);
    // Box first allocates everything on the stack and then copies into the heap.

    let kiste = vec![0; 3_000_000];
    let slice = kiste.into_boxed_slice();
    println!("Length: {}", slice.len());

    // size of a box is always 8 bytes
    println!("{}", size_of::<Box<i32>>()); // 8
    println!("{}", size_of::<Box<f64>>()); // 8
    println!("{}", size_of::<Box<List>>()); // 8
    println!("{}", size_of::<Box<EnumOfNumbers>>()); // 8
  }

  println!();

  {
    let my_number = 1;
    just_takes_a_variable(my_number);
    just_takes_a_variable(my_number); // ✓ my_number is Copy

    let my_box = Box::new(1);
    just_takes_a_variable(my_box.clone()); // Box is not Copy; we need to clone
    just_takes_a_variable(my_box);
  }

  {
    let my_box: Box<i32> = Box::new(42);
    let an_integer: i32 = *my_box;

    println!("{:?}", my_box);
    println!("{:?}", an_integer);
  }

  println!();

  {
    let my_list = List::new();
    println!("{:?}", my_list);

    let my_list = List::empty();
    println!("{:?}", my_list);
  }

  println!();

  {
    println!(
      "{:#?}",
      vec![
        size_of::<EnumOfNumbers>(),      // 2
        size_of::<StructOfNumbers>(),    // 3
        size_of::<EnumOfOtherTypes>(),   // 24
        size_of::<StructOfOtherTypes>(), // 32
        size_of::<ArrayAndI8>(),         // 1002
      ]
    );
  }

  println!();

  {
    returns_just_a_trait1();
    returns_just_a_trait2();
    returns_just_a_trait3();
  }

  println!();

  {
    let vec_of_u8s: Vec<u8> = vec![0, 1, 42];

    for n in vec_of_u8s {
      match returns_errors1(n) {
        Ok(msg) => println!("{}", msg),
        Err(msg) => println!("{}", msg),
      }
    }
  }
}
