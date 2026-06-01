// Type aliases
// https://dhghomon.github.io/easy_rust/Chapter_43.html
// https://www.youtube.com/watch?v=6K6BUQPXUSg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk&index=113&pp=iAQB

#![allow(unused)]

use std::iter::{Skip, Take};
use std::slice::Iter;

type CharacterVec = Vec<char>;

// this return type is extremely long
fn returns1<'a>(
  input: &'a Vec<char>,
) -> std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>> {
  input.iter().skip(4).take(5)
}

type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;

fn returns2<'a>(input: &'a Vec<char>) -> SkipFourTakeFive<'a> {
  input.iter().skip(4).take(5)
}

fn returns3<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
  input.iter().skip(4).take(5)
}

type SkipFourTakeFive1<'a> = Take<Skip<Iter<'a, char>>>;

fn returns4<'a>(input: &'a Vec<char>) -> SkipFourTakeFive1<'a> {
  input.iter().skip(4).take(5)
}

enum MapDirection {
  North,
  NorthEast,
  East,
  SouthEast,
  South,
  SouthWest,
  West,
  NorthWest,
}

// full qualification
fn give_direction1(direction: &MapDirection) {
  match direction {
    MapDirection::North => println!("You are heading north."),
    MapDirection::NorthEast => println!("You are heading northeast."),
    _ => println!("…"),
  }
}

// use
fn give_direction2(direction: &MapDirection) {
  use MapDirection::*;
  let m = "You are heading";

  match direction {
    North => println!("{} north.", m),
    NorthEast => println!("{} northeast.", m),
    _ => println!("…"),
  }
}

enum FileState {
  CannotAccessFile,
  FileOpenedAndReady,
  NoSuchFileExists,
  SimilarFileNameInNextDirectory,
}

// use … as
fn give_filestate(input: &FileState) {
  use FileState::{
    CannotAccessFile as NoAccess, FileOpenedAndReady as Good, NoSuchFileExists as NoFile,
    SimilarFileNameInNextDirectory as OtherDirectory,
  };
  match input {
    NoAccess => println!("Can't access file."),
    Good => println!("Here is your file"),
    NoFile => println!("Sorry, there is no file by that name."),
    OtherDirectory => println!("Please check the other directory."),
  }
}

fn main() {
  {
    type File = String;

    let my_file = File::from("I am file contents");
    let my_string = String::from("I am file contents");
    println!("{}", my_file == my_string); // true
  }

  {
    struct File(String); // File is a wrapper around String

    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
  }

  // {
  //   struct File(String); // File is a wrapper around String
  //
  //   let my_file = File(String::from("I am file contents"));
  //   let my_string = String::from("I am file contents");
  //   println!("{}", my_file == my_string); // ⚠️ cannot compare File with String
  // }

  {
    struct File(String);

    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    println!("{}", my_file.0 == my_string); // true
  }

  {
    #[derive(Clone, Debug)]
    struct File(String);
  }
}
