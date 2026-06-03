// Deref and DerefMut
// https://dhghomon.github.io/easy_rust/Chapter_56.html
// https://www.youtube.com/watch?v=wXAbnDCACGY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=6WR7AiyU1cg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=C-CViDtRcaw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=O7Ta07s3pNM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(dead_code)]

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct HoldsANumber(u8);

impl HoldsANumber {
  fn prints_the_number_times_two(&self) {
    println!("{}", self.0 * 2);
  }
}

impl Deref for HoldsANumber {
  type Target = u8;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for HoldsANumber {
  // You don't need type Target = u8; here because it already knows thanks to Deref
  fn deref_mut(&mut self) -> &mut Self::Target {
    // Everything else is the same except it says mut everywhere
    &mut self.0
  }
}

struct DerefExample<T> {
  value: T,
}

impl<T> Deref for DerefExample<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.value
  }
}

enum Alignment {
  Good,
  Neutral,
  Evil,
}

struct Character {
  name: String,
  strength: u8,
  dexterity: u8,
  health: u8,
  intelligence: u8,
  wisdom: u8,
  charm: u8,
  hit_points: i8,
  alignment: Alignment,
}

impl Character {
  fn new(
    name: String,
    strength: u8,
    dexterity: u8,
    health: u8,
    intelligence: u8,
    wisdom: u8,
    charm: u8,
    hit_points: i8,
    alignment: Alignment,
  ) -> Self {
    Self {
      name,
      strength,
      dexterity,
      health,
      intelligence,
      wisdom,
      charm,
      hit_points,
      alignment,
    }
  }
}

// impl Deref for Character. Now we can do any integer math we want! ⚠️ WEIRD!!!
impl Deref for Character {
  type Target = i8;

  fn deref(&self) -> &Self::Target {
    &self.hit_points
  }
}

fn main() {
  // ⚠️ can't compare `i32` with `&i32`
  // {
  //   let value = 7; // This is an i32
  //   let reference = &7; // This is a &i32
  //   println!("{}", value == reference);
  // }

  {
    let value = 7;
    let reference = &7;
    println!("{}", value == *reference);
  }

  // ⚠️ type `HoldsANumber` cannot be dereferenced
  // {
  //   let my_number = HoldsANumber(20);
  //   println!("{}", *my_number + 20);
  // }

  {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);
  }

  // Deref
  {
    let my_number = HoldsANumber(20);
    println!("{:?}", *my_number + 20);
  }

  println!();

  {
    let my_number = HoldsANumber(20);
    println!("{:?}", my_number.checked_sub(100)); // 20 - 100 = -80; doesn't fit in u8
    my_number.prints_the_number_times_two();
  }

  println!();

  // DerefMut
  {
    let mut my_number = HoldsANumber(20);
    *my_number = 30; // DerefMut lets us do this
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
  }

  println!();

  {
    let brandy1 = Character::new(
      "Brandy1".to_string(),
      9,
      8,
      7,
      10,
      19,
      19,
      5,
      Alignment::Good,
    );

    let brandy2 = Character::new(
      "Brandy2".to_string(),
      9,
      8,
      7,
      10,
      19,
      19,
      5,
      Alignment::Good,
    );

    let mut hit_points = vec![];
    hit_points.push(*brandy1); // ⚠️ looks frigging WEIRD! 
    hit_points.push(*brandy2); // ⚠️ looks frigging WEIRD! 

    println!("{:?}", hit_points);
  }
}
