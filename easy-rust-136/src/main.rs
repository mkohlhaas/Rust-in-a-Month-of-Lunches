// Attributes
// https://dhghomon.github.io/easy_rust/Chapter_52.html
// https://www.youtube.com/watch?v=9bgI1SKIsv0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=h3vG59kUf-o&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// The Rust Reference: Attributes
// https://doc.rust-lang.org/reference/attributes.html

// #![…] global
// #[…]  local

#![allow(unused)]
// #![allow(dead_code)]
#![allow(unused_variables)]
struct JustAStruct1 {}
struct _JustAStruct2 {}

struct Struct1 {} // Create five structs
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone)]
struct HoldsAString {
  the_string: String,
}

#[derive(Clone, Copy)] // Copy needs Clone
struct NumberAndBool {
  number: i32,
  true_or_false: bool,
}

fn does_nothing(input: NumberAndBool) {}

fn main() {
  // using _ to quiet the compiler
  {
    let _some_char = 'ん';
  }

  {
    let char1 = 'ん';
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];
  }

  {
    let holds_a_string = HoldsAString {
      the_string: "Here I am!".to_string(),
    };

    println!("{:?}", holds_a_string);
  }

  {
    let number_and_bool = NumberAndBool {
      number: 8,
      true_or_false: true,
    };

    does_nothing(number_and_bool); // is copied …
    does_nothing(number_and_bool); // … no error!
  }
}
