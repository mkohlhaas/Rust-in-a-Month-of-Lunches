// External crates
// https://dhghomon.github.io/easy_rust/Chapter_59.html
// https://www.youtube.com/watch?v=vVzAyS99-og&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=25bFCysCPOg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=MvVRLi6WsJQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use chrono::prelude::*;
use rand::*;
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::{self, Display};

struct Character {
  strength: u8,
  dexterity: u8,
  constitution: u8,
  intelligence: u8,
  wisdom: u8,
  charisma: u8,
}

fn three_die_six() -> u8 {
  let mut generator = rand::rng();
  let mut total = 0;

  for _ in 0..3 {
    total += generator.random_range(1..=6);
  }

  total
}

fn four_die_six() -> u8 {
  let mut generator = rand::rng();
  let mut results = vec![];

  for _ in 0..4 {
    results.push(generator.random_range(1..=6));
  }

  results.sort();
  results.remove(0);
  results.iter().sum()
}

enum Dice {
  Three,
  Four,
}

impl Display for Character {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Your character has these stats:\n \
        strength: {}\n \
        dexterity: {}\n \
        constitution: {}\n \
        intelligence: {}\n \
        wisdom: {}\n \
        charisma: {}",
      self.strength,
      self.dexterity,
      self.constitution,
      self.intelligence,
      self.wisdom,
      self.charisma
    )
  }
}

impl Character {
  fn new(dice: Dice) -> Self {
    match dice {
      Dice::Three => Self {
        strength: three_die_six(),
        dexterity: three_die_six(),
        constitution: three_die_six(),
        intelligence: three_die_six(),
        wisdom: three_die_six(),
        charisma: three_die_six(),
      },
      Dice::Four => Self {
        strength: four_die_six(),
        dexterity: four_die_six(),
        constitution: four_die_six(),
        intelligence: four_die_six(),
        wisdom: four_die_six(),
        charisma: four_die_six(),
      },
    }
  }

  fn display(&self) {
    println!("{}", self);
    println!();
  }
}

fn sum_of_squares(input: &[i32]) -> i32 {
  input.par_iter().map(|&i| i * i).sum()
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  {
    for _ in 0..5 {
      println!("{}", rand::random::<u16>());
    }
  }

  println!();

  {
    let mut number_maker = rand::rng();
    for _ in 0..5 {
      println!("{} ", number_maker.random::<u16>());
    }
  }

  println!();

  {
    for _ in 0..5 {
      print!("{} ", rand::random_range(0..10));
    }
  }

  println!();
  println!();

  {
    let words: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    println!("{}", words[rand::random_range(..words.len())]);
  }

  println!();

  {
    let weak_billy = Character::new(Dice::Three);
    let strong_billy = Character::new(Dice::Four);

    weak_billy.display();
    strong_billy.display();
  }

  println!();

  {
    let numbers: Vec<i32> = (1..=100).collect();
    println!("Gauss sum:      {}", numbers.iter().sum::<i32>());
    println!("Sum of Squares: {}", sum_of_squares(&numbers));
  }

  println!();

  {
    let mut my_vec: Vec<i64> = vec![0; 200_000];

    my_vec
      .iter_mut()
      .enumerate()
      .for_each(|(idx, n)| *n += idx as i64);

    println!("{:?}", &my_vec[5000..5010]);
    println!("{:?}", my_vec.iter().sum::<i64>());
  }

  println!();

  {
    let point = Point { x: 1, y: 2 };
    println!("{:?}", point);

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized:   {}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();
    println!("deserialized: {:?}", deserialized);
  }

  println!();

  {
    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

    let v: Value = serde_json::from_str(data).unwrap();

    println!(
      "Please call {} at the number {}.",
      v["name"], v["phones"][0]
    );
  }

  println!();

  {
    let re = Regex::new(
      r"(?x) (?P<year>\d{4})  # the year
           - (?P<month>\d{2}) # the month
           - (?P<day>\d{2})   # the day
           ",
    )
    .unwrap();

    let caps = re.captures("2010-03-14").unwrap();

    println!("year:  {:?}", &caps["year"]);
    println!("month: {:?}", &caps["month"]);
    println!("day:   {:?}", &caps["day"]);
  }

  println!();

  {
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    println!("{:?}", utc);
    println!("{:?}", local);
  }
}
