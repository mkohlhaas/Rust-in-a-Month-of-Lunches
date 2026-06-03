// Default and the builder pattern
// https://dhghomon.github.io/easy_rust/Chapter_55.html
// https://www.youtube.com/watch?v=BA4eCOv--3M&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=138
// https://www.youtube.com/watch?v=mY1kVT5JajI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=139
// https://www.youtube.com/watch?v=JsXp3zI4c_M&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=140

#![allow(dead_code)]

#[derive(Debug)]
enum LifeState {
  Alive,
  Dead,
  NeverAlive,
  Uncertain,
}

#[derive(Debug)]
struct Character {
  name: String,
  age: u8,
  height: u32,
  weight: u32,
  lifestate: LifeState,
  can_use: bool,
}

impl Character {
  fn new() -> Self {
    Self {
      name: "Billy".to_string(),
      age: 15,
      height: 170,
      weight: 70,
      lifestate: LifeState::Alive,
      can_use: true,
    }
  }

  // for builder pattern; consumes self and always returns Self
  fn height(mut self, height: u32) -> Self {
    self.height = height;
    self.can_use = false;
    self
  }

  fn weight(mut self, weight: u32) -> Self {
    self.weight = weight;
    self.can_use = false;
    self
  }

  fn name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self.can_use = false;
    self
  }

  fn build(mut self) -> Result<Character, String> {
    if self.height < 200 && self.weight < 300 && !self.name.to_lowercase().contains("smurf") {
      self.can_use = true;
      Ok(self)
    } else {
      Err("Could not create character!".to_string())
    }
  }
}

impl Default for Character {
  fn default() -> Self {
    Self {
      name: "Billy".to_string(),
      age: 15,
      height: 170,
      weight: 70,
      lifestate: LifeState::Alive,
      can_use: true,
    }
  }
}

fn main() {
  {
    let default_i8_1: i8 = Default::default();
    let default_i8_2 = i8::default();
    let default_str_1: String = Default::default();
    let default_str_2 = String::default();
    let default_bool_1: bool = Default::default();
    let default_bool_2 = bool::default();

    println!(
      "{:#?}",
      (
        default_i8_1,   // 0
        default_i8_2,   // 0
        default_str_1,  // ""
        default_str_2,  // ""
        default_bool_1, // false
        default_bool_2, // false
      )
    );
  }

  println!();

  {
    let character = Character::default();

    println!("{:?}", character);
  }

  println!();

  // builder pattern
  {
    let mut character1: Character = Default::default();
    character1 = character1.height(180).weight(60).name("Bobby");

    let character2 = Character::default().height(180).weight(60).name("Bobby");

    println!("{:?}", character1);
    println!("{:?}", character2);
  }

  println!();

  {
    let character = Character::new().height(180).weight(60).name("Bobby");

    println!("{:?}", character);
  }

  println!();

  {
    // will be declined
    let character_with_smurf = Character::new().name("Lol I am Smurf!!").build();
    let character_too_tall = Character::new().height(400).build();
    let character_too_heavy = Character::new().weight(500).build();

    // ✓
    let character_ok = Character::new()
      .name("Billybrobby")
      .height(180)
      .weight(100)
      .build();

    let character_vec = vec![
      character_with_smurf,
      character_too_tall,
      character_too_heavy,
      character_ok,
    ];

    println!("{:#?}", character_vec);
  }
}
