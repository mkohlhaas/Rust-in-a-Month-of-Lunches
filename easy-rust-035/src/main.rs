// Implementing structs and enums
// https://dhghomon.github.io/easy_rust/Chapter_27.html
// https://www.youtube.com/watch?v=cxTP5gPuiu4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=vqTK35kw7wQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
enum AnimalType {
  Cat,
  Dog,
}

#[derive(Debug)]
struct Animal {
  age: u8,
  kind: AnimalType,
}

// implementing a struct
impl Animal {
  // associated/static function
  fn new() -> Self {
    Self {
      age: 10,
      kind: AnimalType::Cat,
    }
  }

  fn change_to_dog(&mut self) {
    println!("Changing animal to dog!");
    self.kind = AnimalType::Dog;
  }

  fn change_to_cat(&mut self) {
    println!("Changing animal to cat!");
    self.kind = AnimalType::Cat;
  }

  fn print_animal_type(&self) {
    use AnimalType::*;
    match self.kind {
      Dog => println!("The animal is a dog."),
      Cat => println!("The animal is a cat."),
    }
  }

  fn print_age(&self) {
    println!("The animal is {} years old.", self.age);
  }
}

enum Mood {
  Good,
  Bad,
  Sleepy,
}

// implementing an enum
impl Mood {
  fn check(&self) {
    use Mood::*;
    match self {
      Good => println!("Feeling good!"),
      Bad => println!("Eh, not feeling so good."),
      Sleepy => println!("Need sleep NOW."),
    }
  }
}

fn main() {
  {
    let mut animal = Animal::new();

    animal.print_animal_type();
    animal.print_age();
    animal.change_to_dog();
    animal.print_animal_type();
    animal.change_to_cat();
    animal.print_animal_type();
  }

  println!();

  {
    let mood1 = Mood::Sleepy;
    let mood2 = Mood::Good;
    let mood3 = Mood::Bad;

    mood1.check();
    mood2.check();
    mood3.check();
  }
}
