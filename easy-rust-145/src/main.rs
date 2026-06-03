// Crates and modules
// https://dhghomon.github.io/easy_rust/Chapter_57.html
// https://www.youtube.com/watch?v=GUd1j11CPqE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=uGmOr9CnbRY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=fvoqOpHCMvg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(dead_code)]

// pub for a struct:         It makes the struct public, but the items are NOT public.
//                           To make an item public, you have to write pub for each one too.
//
// pub for an enum or trait: Everything becomes public.
//
// pub for a module:         A top level module will be pub because if it isn't pub then nobody can touch anything in it at all.
//                           But modules inside modules need pub to be public.

mod print_things {
  use std::fmt::{Debug, Display};

  #[derive(Debug)]
  pub struct Billy {
    name: String,
    pub times_to_print: u32,
  }

  impl Billy {
    pub fn new(times_to_print: u32) -> Self {
      Self {
        name: "Billy".to_string(),
        times_to_print,
      }
    }

    pub fn print_billy(&self) {
      for _ in 0..self.times_to_print {
        println!("{:?}", self.name);
      }
    }
  }

  pub fn prints_one_thing<T: Display>(input: T) {
    println!("{}", input)
  }
}

mod country1 {
  fn print_country(country: &str) {
    println!("We are in the country of {}.", country);
  }

  pub mod province {
    fn print_province(province: &str) {
      println!("in the province of {}.", province);
    }

    // A child mod can always use anything inside a parent mod.
    pub mod city {
      pub fn print_city(country: &str, province: &str, city: &str) {
        crate::country1::print_country(country);
        crate::country1::province::print_province(province);
        println!("in the city of {}.", city);
      }
    }
  }
}

// using super - more compact
mod country2 {
  fn print_country(country: &str) {
    println!("We are in the country of {}.", country);
  }

  pub mod province {
    fn print_province(province: &str) {
      println!("in the province of {}.", province);
    }

    pub mod city {
      use super::super::*; // use everything in "above above": that means mod country2
      use super::*; // use everything in "above": that means mod province

      pub fn print_city(country: &str, province: &str, city: &str) {
        print_country(country);
        print_province(province);
        println!("in the city of {}.", city);
      }
    }
  }
}

fn main() {
  {
    use crate::print_things::*;

    let my_billy = Billy::new(7);
    my_billy.print_billy();
  }

  println!();

  {
    crate::country1::province::city::print_city("Canada", "New Brunswick", "Moncton");
  }

  println!();

  // more compact
  {
    use crate::country2::province::city::print_city;

    print_city("Canada", "New Brunswick", "Moncton");
    println!();
    print_city("Korea", "Gyeonggi-do", "Gwangju");
  }
}
