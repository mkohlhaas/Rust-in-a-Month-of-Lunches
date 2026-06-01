// Rc
// https://dhghomon.github.io/easy_rust/Chapter_45.html
// https://www.youtube.com/watch?v=T9UXIn-nUkw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=kdh08-JTG78&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=JUPyGrGPzaY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=RhlBvuZvwy8&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(unused)]

// A single-threaded reference-counting pointer.
// 'Rc' stands for 'Reference Counted'.

use std::rc::Rc;

fn takes_a_string(input: String) {
  println!("It is: {}", input)
}

fn also_takes_a_string(input: String) {
  println!("It is: {}", input)
}

#[derive(Debug)]
struct City1 {
  name: String,
  population: u32,
  city_history: String,
}

#[derive(Debug)]
struct CityData1 {
  names: Vec<String>,
  histories: Vec<String>,
}

#[derive(Debug)]
struct City2 {
  name: String,
  population: u32,
  city_history: Rc<String>, // String inside an Rc
}

#[derive(Debug)]
struct CityData2 {
  names: Vec<String>,
  histories: Vec<Rc<String>>, // Vec of Strings inside Rcs
}

fn main() {
  // {
  //   let user_name = String::from("User MacUserson");
  //
  //   takes_a_string(user_name);
  //   also_takes_a_string(user_name); // ⚠️ use of moved value: `user_name`
  // }

  // {
  //   let calgary = City1 {
  //     name: "Calgary".to_string(),
  //     population: 1_200_000,
  //     city_history: "Calgary began as a fort called Fort Calgary that …".to_string(),
  //   };
  //
  //   let canada_cities = CityData1 {
  //     names: vec![calgary.name], // this is using calgary.name, which is short
  //     histories: vec![calgary.city_history], // but this String is very long; ⚠️ value moved here [E0382]
  //   };
  //
  //   println!("Calgary's history is: {}", calgary.city_history); // ⚠️ borrow of moved value: `calgary.city_history`
  // }

  {
    let calgary = City2 {
      name: "Calgary".to_string(),
      population: 1_200_000,
      // Pretend that this string is very very long
      city_history: Rc::new("Calgary began as a fort called Fort Calgary that...".to_string()),
    };

    // A clone of an Rc just clones a pointer - it's basically free!

    let canada_cities = CityData2 {
      names: vec![calgary.name],                     // this is actually a move
      histories: vec![calgary.city_history.clone()], // .clone() to increase the count
    };

    println!("Calgary's history is: {}", calgary.city_history);

    println!("{}", Rc::strong_count(&calgary.city_history)); // 2
    let new_owner = calgary.city_history.clone();
    println!("{}", Rc::strong_count(&calgary.city_history)); // 3
    drop(canada_cities);
    println!("{}", Rc::strong_count(&calgary.city_history)); // 2
  }
}
