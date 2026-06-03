// impl Trait
// https://dhghomon.github.io/easy_rust/Chapter_48.html
// https://www.youtube.com/watch?v=xazaHXnvnDM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=plw2Yo1WvoY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::Display;

fn gives_higher_i32(one: i32, two: i32) {
  let higher = if one > two { one } else { two };
  println!("{} is higher.", higher);
}

fn gives_higher<T: PartialOrd + Display>(one: T, two: T) {
  let higher = if one > two { one } else { two };
  println!("{} is higher.", higher);
}

// `impl Trait` in a type-position can be used to designate a type that implements a trait called `Trait`.

// the same (trait bound syntax): https://doc.rust-lang.org/stable/book/ch10-02-traits.html#trait-bound-syntax
// fn prints_it<D: Display + Into<String>>(input: D) {

fn prints_it(input: impl Display + Into<String>) {
  println!("You can print many things, including {}.", input);
}

fn returns_a_closure(input: &str) -> impl FnMut(i32) -> i32 {
  match input {
    "double" => |mut n| {
      n *= 2;
      println!("Doubling number. Now it is {}.", n);
      n
    },
    "triple" => |mut n| {
      n *= 40;
      println!("Tripling number. Now it is {}.", n);
      n
    },
    _ => |n| {
      println!("Sorry, it's the same: {}.", n);
      n
    },
  }
}

enum TimeOfDay {
  Dawn,
  Day,
  Sunset,
  Night,
}

fn change_fear(tod: TimeOfDay) -> impl FnMut(f64) -> f64 {
  use TimeOfDay::*;

  match tod {
    Dawn => |fear| {
      println!("The morning sun has vanquished the horrible night. You no longer feel afraid.");
      println!("Your fear is now {}.", fear * 0.5);
      fear * 0.5
    },
    Day => |fear| {
      println!("What a nice day. Maybe put your feet up and rest a bit.");
      println!("Your fear is now {}.", fear * 0.2);
      fear * 0.2
    },
    Sunset => |fear| {
      println!("The sun is almost down! This is no good.");
      println!("Your fear is now {}.", fear * 1.4);
      fear * 1.4
    },
    Night => |fear| {
      println!("What a horrible night to have a curse.");
      println!("Your fear is now {}.", fear * 5.0);
      fear * 5.0
    },
  }
}
fn main() {
  {
    gives_higher_i32(8, 10);
  }

  println!();

  {
    gives_higher(8, 10);
  }

  println!();

  {
    let name: &str = "Tuon";
    let string_name: String = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);
  }

  println!();

  {
    let my_number = 10;

    let mut doubles = returns_a_closure("double");
    let mut triples = returns_a_closure("triple");
    let mut quadruples = returns_a_closure("quadruple");

    doubles(my_number);
    triples(my_number);
    quadruples(my_number);
  }

  println!();

  {
    use TimeOfDay::*;
    let mut character_fear = 10.0;

    let mut morning = change_fear(Dawn);
    let mut daytime = change_fear(Day);
    let mut sunset = change_fear(Sunset);
    let mut night = change_fear(Night);

    character_fear = morning(character_fear);
    character_fear = daytime(character_fear);
    character_fear = sunset(character_fear);
    night(character_fear);
  }
}
