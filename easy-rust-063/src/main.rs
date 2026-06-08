// Traits
// https://dhghomon.github.io/easy_rust/Chapter_34.html
// https://www.youtube.com/watch?v=YEx1ABiNeBc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=kDpqRNHIz4E&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=o9jZXLX9_Vw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=dmPKGL6Gl0I&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=lkNuhMPqaIs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ld8UV-AiMTQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=E6P73FKPwxE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=UDXl1iX-cV4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=xStMBBnfKyA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=JIv1Pv4vCHU&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=iKFljZP6JD0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::fmt::{self, Debug, Display};

struct Animal {
  name: String,
}

// A trait with default implementations.
// trait Dog {
//   fn bark(&self) {
//     println!("Woof, woof!");
//   }
//
//   fn run(&self) {
//     println!("The dog is running!");
//   }
// }

// impl Dog for Animal {}

// impl Dog for Animal {
//   fn run(&self) {
//     println!("{} is running!", self.name);
//   }
// }

// A trait just defining the functional interface.
trait Dog {
  fn bark(&self);
  fn run(&self);
}

impl Dog for Animal {
  fn bark(&self) {
    println!("{}, stop barking!!", self.name);
  }
  fn run(&self) {
    println!("{} is running!", self.name);
  }
}

#[derive(Debug)]
struct Cat {
  name: String,
  age: u8,
}

impl Display for Cat {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} is a cat who is {} years old.", self.name, self.age)
  }
}

struct Position {
  longitude: f32,
  latitude: f32,
}

impl Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.longitude, self.latitude)
  }
}

fn print_cats(pet: String) {
  println!("{}", pet);
}

// Traits without constraints.

// trait FightClose {
//   fn attack_with_sword(&self, opponent: &mut Monster) {
//     opponent.health -= 10;
//     println!(
//       "You attack with your sword. Your opponent now has {} health left.",
//       opponent.health
//     );
//   }
//
//   fn attack_with_hand(&self, opponent: &mut Monster) {
//     opponent.health -= 2;
//     println!(
//       "You attack with your hand.  Your opponent now has {} health left.",
//       opponent.health
//     );
//   }
// }

// trait FightFromDistance {
//   fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//     if distance < 10 {
//       opponent.health -= 10;
//       println!(
//         "You attack with your bow.   Your opponent now has {} health left.",
//         opponent.health
//       );
//     }
//   }
//   fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//     if distance < 3 {
//       opponent.health -= 4;
//     }
//     println!(
//       "You attack with your rock.  Your opponent now has {} health left.",
//       opponent.health
//     );
//   }
// }

// Traits with a `Debug` constraint.

// trait FightClose: Debug {
//   fn attack_with_sword(&self, opponent: &mut Monster) {
//     opponent.health -= 10;
//     println!(
//       "You attack with your sword. Your opponent now has {} health left. You are now at: {:?}",
//       opponent.health, &self
//     );
//   }
//
//   fn attack_with_hand(&self, opponent: &mut Monster) {
//     opponent.health -= 2;
//     println!(
//       "You attack with your hand. Your opponent now has {} health left.  You are now at: {:?}",
//       opponent.health, &self
//     );
//   }
// }

// trait FightFromDistance: Debug {
//   fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//     if distance < 10 {
//       opponent.health -= 10;
//       println!(
//         "You attack with your bow. Your opponent now has {} health left.   You are now at: {:?}",
//         opponent.health, self
//       );
//     }
//   }
//
//   fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//     if distance < 3 {
//       opponent.health -= 4;
//     }
//     println!(
//       "You attack with your rock. Your opponent now has {} health left.  You are now at: {:?}",
//       opponent.health, self
//     );
//   }
// }

// only trait bounds
trait Magic {}
trait FightClose {}
trait FightFromDistance {}

struct Monster {
  health: i32,
}

// A Monster does not have any special traits.

#[derive(Debug)]
struct Wizard {
  health: i32,
}
// A Wizard can fight close and do magic.
impl FightClose for Wizard {}
impl Magic for Wizard {}

#[derive(Debug)]
struct Ranger {
  health: i32,
}
// A Ranger can fight close and from distance.
impl FightClose for Ranger {}
impl FightFromDistance for Ranger {}

// Functions using these trait bounds.

fn attack_with_bow<T: FightFromDistance + Debug>(
  character: &T,
  opponent: &mut Monster,
  distance: u32,
) {
  if distance < 10 {
    opponent.health -= 10;
    println!(
      "You attack with your bow.                 Your opponent now has {} health left. You are now at: {:?}",
      opponent.health, character
    );
  }
}

fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster) {
  opponent.health -= 10;
  println!(
    "You attack with your sword.               Your opponent now has {} health left. You are now at: {:?}",
    opponent.health, character
  );
}

fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32) {
  if distance < 15 {
    opponent.health -= 20;
    println!(
      "You raise your hands and cast a fireball! Your opponent now has {} health left.  You are now at: {:?}",
      opponent.health, character
    );
  }
}

fn print_vec<T: Display>(input: &Vec<T>) {
  for item in input {
    print!("{} ", item);
  }
  println!();
}

#[derive(Debug)] // So we can print City
struct City {
  name: String,
  population: u32,
}

impl City {
  fn new(name: &str, population: u32) -> Self {
    Self {
      name: name.to_string(),
      population,
    }
  }
}
#[derive(Debug)]
struct Country {
  cities: Vec<City>,
}

// NOTE: we don't have to write From<City>, we can also do From<Vec<City>>.
// So we can also implement on a type that we didn't create.
impl From<Vec<City>> for Country {
  fn from(cities: Vec<City>) -> Self {
    Self { cities }
  }
}

// only support function for Country's.
impl Country {
  fn print_cities(&self) {
    for city in &self.cities {
      println!("{} has a population of {}.", city.name, city.population);
    }
  }
}

struct EvenOddVec(Vec<Vec<i32>>);

// We can convert a `Vec<i32>` into an EvenOddVec.
impl From<Vec<i32>> for EvenOddVec {
  fn from(input: Vec<i32>) -> Self {
    let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]];

    for item in input {
      if item % 2 == 0 {
        even_odd_vec[0].push(item);
      } else {
        even_odd_vec[1].push(item);
      }
    }

    Self(even_odd_vec)
  }
}

// too generic for what we want
fn print_it1<T: Display>(input: T) {
  println!("{}", input)
}

// We only want to print things that can be converted to a &str.
// T is a type that can be converted to a &str.
fn print_it2<T: AsRef<str> + Display>(input: T) {
  println!("{}", input)
}

// Use `where` syntax when there are too many constraints.
fn print_it3<T>(input: T)
where
  T: AsRef<str> + Debug + Display,
{
  println!("{}\n{:?}", input, input)
}

#[derive(Debug)]
struct Number {
  value: i32,
}

impl From<i32> for Number {
  fn from(item: i32) -> Self {
    Number { value: item }
  }
}

fn main() {
  {
    let rover = Animal {
      name: "Rover".to_string(),
    };

    rover.bark();
    rover.run();
  }

  {
    let mr_mantle = Cat {
      name: "Reggie Mantle".to_string(),
      age: 4,
    };
    println!("{}", mr_mantle.name);
    println!("{}", mr_mantle.age);

    println!("{:?}", mr_mantle);

    println!("{}", mr_mantle);

    // if you implement Display then you get the ToString trait for free
    print_cats(mr_mantle.to_string());
    println!(
      "Mr. Mantle's String is {} letters long.",
      mr_mantle.to_string().chars().count()
    );
  }

  {
    let pos = Position {
      latitude: 43.23,
      longitude: 23.53,
    };
    println!("{}", pos);
  }

  println!();

  // {
  //   let radagast = Wizard {};
  //   let aragorn = Ranger {};
  //   let mut uruk_hai = Monster { health: 40 };
  //
  //   radagast.attack_with_sword(&mut uruk_hai);
  //   radagast.attack_with_hand(&mut uruk_hai);
  //
  //   aragorn.attack_with_bow(&mut uruk_hai, 8);
  //   aragorn.attack_with_rock(&mut uruk_hai, 8);
  //   aragorn.attack_with_rock(&mut uruk_hai, 2);
  // }

  {
    let radagast = Wizard { health: 60 };
    let aragorn = Ranger { health: 80 };

    let mut uruk_hai = Monster { health: 40 };

    attack_with_sword(&radagast, &mut uruk_hai);
    attack_with_bow(&aragorn, &mut uruk_hai, 8);
    fireball(&radagast, &mut uruk_hai, 8);
  }

  println!();

  {
    let vec_from_array = Vec::from([8, 9, 10]);
    print_vec(&vec_from_array);

    let vec_from_str = Vec::from("What kind of vec will I be?");
    print_vec(&vec_from_str);

    let vec_from_string = Vec::from("What kind of vec will a String be?".to_string());
    print_vec(&vec_from_string);
  }

  println!();

  {
    let helsinki = City::new("Helsinki", 631_695);
    let turku = City::new("Turku", 186_756);

    let finland_cities = vec![helsinki, turku];
    let finland = Country::from(finland_cities);

    finland.print_cities();
  }

  println!();

  {
    let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
    let even_odds = EvenOddVec::from(bunch_of_numbers);

    println!(
      "Even numbers: {:?}\nOdd numbers: {:?}",
      even_odds.0[0], even_odds.0[1]
    );
  }

  println!();

  {
    print_it1("Please print me!");
    print_it1("Also, please print me!".to_string());
    print_it1(7);
  }

  println!();

  {
    print_it2("Please print me!");
    print_it2("Also, please print me!".to_string());
    // print_it2(7); // ⚠️ the trait `AsRef<str>` is not implemented for `{integer}` [E0277]
  }

  println!();

  {
    print_it3("Please print me!");
    print_it3("Also, please print me!".to_string());
    // print_it3(7); // ⚠️ the trait `AsRef<str>` is not implemented for `{integer}` [E0277]
  }

  println!();

  {
    let int = 5;

    // `From` gives you automatically `Into`.
    let num: Number = int.into();
    println!("{}", num.value);
    println!("My number is {:?}", num);
  }
}
