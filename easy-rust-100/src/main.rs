// Lifetimes
// https://dhghomon.github.io/easy_rust/Chapter_40.html
// https://www.youtube.com/watch?v=oB1nEReqV68&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=M3LIlfGSVVs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=6Fni64brFsE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=hFN9KcWqX34&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::Display;

// fn returns_reference() -> &str { // ⚠️ missing lifetime specifier
//   let my_string = String::from("I am a string");
//   &my_string
// }

// fn returns_str() -> &str { // ⚠️ missing lifetime specifier
//   let my_string = String::from("I am a string");
//   "I am a str"
// }

fn returns_str() -> &'static str {
  let _my_string = String::from("I am a string");
  "I am a str."
}

// #[derive(Debug)]
// struct City {
//   name: &str, // ⚠️ missing lifetime specifier
//   date_founded: u32,
// }

// static lifetime
// #[derive(Debug)]
// struct City {
//   name: &'static str,
//   date_founded: u32,
// }

#[derive(Debug)]
// `City` has lifetime 'a and `name` also has lifetime 'a.
// This means that it will only take a reference for `name` if it lives at least as long as `City`.
struct City<'a> {
  name: &'a str,
  date_founded: u32,
}

// struct Adventurer<'a> {
//   name: &'a str,
//   hit_points: u32,
// }

// ⚠️
// impl Adventurer { // expected lifetime parameter [E0726]
//   fn take_damage(&mut self) {
//     self.hit_points -= 20;
//     println!("{} has {} hit points left!", self.name, self.hit_points);
//   }
// }

struct Adventurer<'a> {
  name: &'a str,
  hit_points: u32,
}

// using the anonymous lifetime
impl Adventurer<'_> {
  fn take_damage(&mut self) {
    self.hit_points -= 20;
    println!("{} has {} hit points left!", self.name, self.hit_points);
  }
}

impl Display for Adventurer<'_> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} has {} hit points.", self.name, self.hit_points)
  }
}

fn main() {
  {
    let my_str = returns_str();
    println!("{}", my_str);
  }

  println!();

  {
    let my_city = City {
      name: "Ichinomiya",
      date_founded: 1921,
    };

    println!("{} was founded in {}.", my_city.name, my_city.date_founded);
  }

  // {
  //   let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()]; // city_names does not live for the whole program
  //
  //   let my_city = City {
  //     name: &city_names[0], // ⚠️ `city_names` does not live long enough. (It should live throughout the program - it's static)
  //     date_founded: 1921,
  //   };
  //
  //   println!("{} was founded in {}", my_city.name, my_city.date_founded);
  // }

  {
    let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];

    let my_city = City {
      name: &city_names[0],
      date_founded: 1921,
    };

    println!("{} was founded in {}", my_city.name, my_city.date_founded);
  }

  println!();

  {
    let mut billy = Adventurer {
      name: "Billy",
      hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
  }
}
