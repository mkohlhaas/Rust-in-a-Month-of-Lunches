// A tour of the standard library
// https://dhghomon.github.io/easy_rust/Chapter_60.html
// https://www.youtube.com/watch?v=qae47eYaU54&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=eXRGWXlgOpw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=zIAlygKSt-A&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=Ds8VD-oX9y4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=RtclkgMwkm0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=KkCwgaEJDbg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=-XTeoWOi9Uo&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=3sudD83_oqs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ji0mxtF0mHk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=Du4v23Rk2CQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=cOHUNTtir1w&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=_RHFLT-cbiE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=_nlTUyb2gXQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=m4C6NS46p2E&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(unused_imports)]

use rand::prelude::*;
use std::convert::TryFrom;
use std::fmt::{self, Display};
use std::mem;
use std::ops::{Add, Deref, DerefMut};
use std::thread::sleep;
use std::time::{Duration, Instant};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Point {
  fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }
}

impl Add for Point {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

#[derive(Clone)]
struct Country {
  name: String,
  population: u32,
  gdp: u32,
}

impl Country {
  fn new(name: &str, population: u32, gdp: u32) -> Self {
    Self {
      name: name.to_string(),
      population,
      gdp,
    }
  }
}

impl Add for Country {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Self {
      name: format!("{} and {}", self.name, other.name),
      population: self.population + other.population,
      gdp: self.gdp + other.gdp,
    }
  }
}

impl Display for Country {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "In {} are {} people and has a GDP of ${}.",
      self.name, self.population, self.gdp
    )
  }
}

fn four_operations(n: f64) {
  println!(
    "For the number {}:\n\
     floor: {}\n\
     ceiling: {}\n\
     rounded: {}\n\
     truncated: {}\n",
    n,
    n.floor(),
    n.ceil(),
    n.round(),
    n.trunc()
  );
}

// create a ring from Lord of the Rings
struct Ring {
  owner: String,
  former_owner: String,
  seeker: String,
}

impl Ring {
  fn new(owner: &str, former_owner: &str, seeker: &str) -> Self {
    Self {
      owner: owner.to_string(),
      former_owner: former_owner.to_string(),
      seeker: seeker.to_string(),
    }
  }
}

impl Display for Ring {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{} has the ring, {} used to have it, and {} wants it.",
      self.owner, self.former_owner, self.seeker
    )
  }
}

struct City {
  name: String,
}

// mem::replace
impl City {
  fn change_name(&mut self, name: &str) {
    let old_name = mem::replace(&mut self.name, name.to_string());
    println!(
      "The city once called {} is now called {}.",
      old_name, self.name
    );
  }
}

struct DeskMoney(u32);

struct Bank {
  money_inside: u32,
  money_at_desk: DeskMoney,
}

impl Default for DeskMoney {
  fn default() -> Self {
    Self(50)
  }
}

impl Deref for DeskMoney {
  type Target = u32;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for DeskMoney {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

impl Bank {
  fn check_money(&self) {
    println!(
      "There is ${} in the back and ${} at the desk.",
      self.money_inside, *self.money_at_desk
    );
  }
}

struct Robber {
  money_in_pocket: u32,
}

impl Robber {
  fn check_money(&self) {
    println!("The robber has ${} right now.\n", self.money_in_pocket);
  }

  fn rob_bank(&mut self, bank: &mut Bank) {
    let new_money = mem::take(&mut bank.money_at_desk);
    self.money_in_pocket += *new_money;
    bank.money_inside -= *new_money;

    println!(
      "She robbed the bank. She now has ${}!",
      self.money_in_pocket
    );
  }
}

fn bad_random_number(digits: usize) {
  if digits > 9 {
    panic!("Random number can only be up to 9 digits");
  }
  let now = Instant::now();
  let output = format!("{:?}", now);

  output
    .chars()
    .rev()
    .skip(2)
    .take(digits)
    .for_each(|character| print!("{}", character));
  println!();
}

enum UkrainePlaces {
  Kiev,
  Kharkiv,
  Chernobyl, // pretend we can't change the enum - Chernobyl will always be here
  Odesa,
  Dnipro,
}

// unreachable!()
fn choose_city(place: &UkrainePlaces) {
  use UkrainePlaces::*;

  match place {
    Kiev => println!("You will live in Kiev."),
    Kharkiv => println!("You will live in Kharkiv."),
    Chernobyl => unreachable!(), // ⚠️ ppanics at runtime
    Odesa => println!("You will live in Odesa."),
    Dnipro => println!("You will live in Dnipro."),
  }
}

// module_path!() - string that represents the current module path
pub mod something {
  pub mod third_mod {
    pub fn print_a_country(countries: &mut Vec<&str>) {
      println!(
        "The last country is {} inside the module {}.",
        countries.pop().unwrap(),
        module_path!()
      );
    }
  }
}

#[cfg(test)]
mod testing {
  use super::*;

  #[test]
  fn check_if_seven() {
    assert_eq!(bring_number(true), 7);
  }
}

fn bring_number(should_run: bool) -> u32 {
  if cfg!(test) && should_run {
    7
  } else if should_run {
    println!("Returning 5. This is not a test.");
    5
  } else {
    println!("This shouldn't run, returning 0.");
    0
  }
}

fn main() {
  let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];

  for city in my_cities {
    println!("{}", city);
  }

  println!();

  for city in &my_cities {
    println!("{}", city);
  }

  println!();

  for city in my_cities.iter() {
    println!("{}", city);
  }

  println!();

  {
    let my_cities = ["Beirut", "Tel Aviv", "Nicosia"];
    let [city1, _city2, _city3] = my_cities;

    println!("{}", city1);
  }

  println!();

  {
    let korean_word = "청춘예찬";

    for c in korean_word.chars() {
      print!("{} ", c.escape_unicode());
    }
  }

  println!();

  {
    let some_character = char::from(65);
    println!("{}\n", some_character);

    // let mut random_generator = rand::rng();

    for c in 10_000u32..11_000 {
      // let bigger_character = char::try_from(random_generator.random_range(u32::MIN..u32::MAX));
      let bigger_character = char::try_from(c);

      if bigger_character.is_ok() {
        print!("{}", bigger_character.unwrap());
      }
    }

    println!();
  }

  println!();

  {
    let some_number = 200u8;

    println!("{:?}", some_number.checked_add(some_number));
    println!("{:?}", some_number.checked_add(1));
  }

  println!();

  {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(2, 3);

    println!("{:?}", p1 + p2);
  }

  println!();

  {
    let nauru = Country::new("Nauru", 10_670, 160_000_000);
    let vanuatu = Country::new("Vanuatu", 307_815, 820_000_000);
    let micronesia = Country::new("Micronesia", 104_468, 367_000_000);

    println!("{}", nauru.clone());
    println!("{}", nauru.clone() + vanuatu.clone());
    println!("{}", nauru + vanuatu + micronesia);
  }

  println!();

  {
    four_operations(9.1);
    four_operations(100.7);
    four_operations(-1.1);
    four_operations(-19.9);
  }

  println!();

  {
    let my_bool = (true, false);
    println!("{} {}", my_bool.0 as u8, my_bool.1 as i32);
  }

  {
    let my_bool: (i128, u16) = (true.into(), false.into());
    println!("{} {}", my_bool.0, my_bool.1);
  }

  println!();

  {
    let (t, f) = (true.then(|| 8), false.then(|| 8));
    println!("{:?}, {:?}", t, f);
  }

  println!();

  {
    let bools = vec![true, false, true, false, false];

    let options = bools
      .iter()
      .map(|item| {
        item.then(|| {
          println!("Got a {}!", item);
          "It's true, you know"
        })
      })
      .collect::<Vec<_>>();

    println!("Now we have: {:?}", options);

    let filtered_vec = options
      .into_iter()
      .filter_map(|opt| opt)
      .collect::<Vec<_>>();

    println!("And without the Nones: {:?}", filtered_vec);
  }

  println!();

  {
    let mut my_vec = vec![100, 90, 80, 1, 4, 8, 14, 18];
    my_vec.sort();
    println!("{:?}", my_vec);
  }

  println!();

  {
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.dedup(); // removes consecutive repeated elements
    println!("{:?}", my_vec);
  }

  println!();

  {
    let mut my_vec = vec!["sun", "sun", "moon", "moon", "sun", "moon", "moon"];
    my_vec.sort();
    my_vec.dedup();
    println!("{:?}", my_vec);
  }

  println!();

  {
    let mut push_string = String::new();
    let mut capacity_counter = 0;

    for _ in 0..100_000 {
      if push_string.capacity() != capacity_counter {
        println!("Capacity changed. It is now {}.", push_string.capacity());
        capacity_counter = push_string.capacity();
      }
      push_string.push_str("I'm getting pushed into the string!");
    }
  }

  println!();

  {
    let mut push_string = String::with_capacity(4587520);
    let mut capacity_counter = 4587520;

    for _ in 0..100_000 {
      if push_string.capacity() != capacity_counter {
        println!("Capacity changed. It is now {}.", push_string.capacity()); // never called
        capacity_counter = push_string.capacity();
      }
      push_string.push_str("I'm getting pushed into the string!");
    }
    println!("Capacity hasn't been changed!");
  }

  println!();

  {
    let mut push_string = String::with_capacity(4587520);
    let mut capacity_counter = 4587520;

    for _ in 0..100_000 {
      if push_string.capacity() != capacity_counter {
        println!("Capacity changed. It is now {}.", push_string.capacity());
        capacity_counter = push_string.capacity();
      }
      push_string.push_str("I'm getting pushed into the string!");
    }

    println!("Current capicity: {}", push_string.capacity());

    push_string.shrink_to_fit();
    println!("Capicity after shrinkage: {}", push_string.capacity());

    push_string.push('a');
    println!(
      "Capicity after pushing a character: {}",
      push_string.capacity()
    );

    push_string.shrink_to_fit();
    println!("Capicity after shrinkage: {}", push_string.capacity());
  }

  println!();

  {
    let mut my_string = String::from(".daer ot drah tib elttil a si gnirts sihT");

    while let Some(c) = my_string.pop() {
      print!("{}", c);
    }
  }

  println!();

  {
    let mut my_string = String::from("Age: 20 Height: 194 Weight: 80");

    my_string.retain(|c| c.is_alphabetic() || c == ' ');
    println!("{}", my_string);
  }

  println!();

  {
    println!("Size of i32: {}", mem::size_of::<i32>());

    let my_array = [8; 50];
    println!("Size of my_array: {}", mem::size_of_val(&my_array));

    let my_string = String::from("You can drop a String because it's on the heap.");
    mem::drop(my_string);
    // some_string.clear();   If we did this it would panic
  }

  println!();

  // mem::swap
  {
    let mut one_ring = Ring::new("Frodo", "Gollum", "Sauron");
    println!("{}", one_ring);

    mem::swap(&mut one_ring.owner, &mut one_ring.former_owner);
    println!("{}", one_ring);
  }

  println!();

  {
    let mut capital_city = City {
      name: "Constantinople".to_string(),
    };

    capital_city.change_name("Istanbul");
  }

  println!();

  // mem::take
  {
    let mut numbers = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];

    numbers.iter_mut().for_each(|n| {
      let prev_val = mem::take(n); // replaces old value with its default value, returning the previous value
      new_vec.push(prev_val);
    });

    println!("{:?}\n{:?}", numbers, new_vec);
  }

  println!();

  {
    let mut bank_of_klezkavania = Bank {
      money_inside: 5000,
      money_at_desk: DeskMoney::default(),
    };

    let mut robber = Robber {
      money_in_pocket: 150,
    };

    bank_of_klezkavania.check_money();
    robber.check_money();

    robber.rob_bank(&mut bank_of_klezkavania);
    robber.check_money();
    bank_of_klezkavania.check_money();

    robber.rob_bank(&mut bank_of_klezkavania); // Do it again
    robber.check_money();
    bank_of_klezkavania.check_money();
  }

  println!();

  {
    let time = Instant::now();
    println!("{:?}", time);
  }

  println!();

  {
    let time1 = Instant::now();
    let time2 = Instant::now();

    let mut my_string = String::new();

    loop {
      my_string.push('წ');
      if my_string.len() > 100_000 {
        break;
      }
    }

    let time3 = Instant::now();

    println!("{:?}", time2 - time1);
    println!("{:?}", time3 - time1);
  }

  println!();

  {
    let time1 = format!("{:?}", Instant::now());
    println!("{}", time1);
  }

  println!();

  {
    bad_random_number(1);
    bad_random_number(2);
    bad_random_number(3);
    bad_random_number(4);
    bad_random_number(5);
    bad_random_number(6);
    bad_random_number(7);
    bad_random_number(8);
    bad_random_number(9);
  }

  println!();

  {
    let _one_second = Duration::from_secs(1);
    println!("I must sleep now.");
    // sleep(one_second);
    println!("Did I miss anything?");
  }

  println!();

  {
    let user_input = UkrainePlaces::Kiev;
    // let user_input = UkrainePlaces::Chernobyl; // ⚠️ panics

    choose_city(&user_input);
  }

  println!();

  // file!(), line!(), column!()
  {
    use something::third_mod::*;

    let mut countries = vec!["Portugal", "Czechia", "Finland"];

    println!("Hello from file {}", file!());

    println!(
      "On line {} we got the country {}",
      line!(),
      countries.pop().unwrap()
    );

    // do some more stuff

    println!(
      "The next country is {} on line {} and column {}.",
      countries.pop().unwrap(),
      line!(),
      column!(),
    );

    // lots more code …

    print_a_country(&mut countries);
  }

  println!();

  {
    let helpful_message = if cfg!(target_os = "windows") {
      "`\\`"
    } else {
      "`/`"
    };

    println!(
      "...then in your hard drive, type the directory name followed by a {}. Then you...",
      helpful_message
    );
  }

  println!();

  {
    bring_number(true);
    bring_number(false);
  }
}
