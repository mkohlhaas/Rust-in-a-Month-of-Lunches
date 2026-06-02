// Closures in functions
// https://dhghomon.github.io/easy_rust/Chapter_47.html
// https://www.youtube.com/watch?v=qXwRQtvWfyc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkw
// https://www.youtube.com/watch?v=UNHEYK6Ihmk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=udNOv7NvEv0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Google AI: rust fn, fnmut, fnonce hierarchy
// https://share.google/aimode/Kz5RQPQycGJMHeegJ

// Google AI: rust difference between Fn, FnMut, FnOnce
//
// Fn     | takes by reference                        | &self
// FnMut  | takes by mutable reference                | &mut self
// FnOnce | takes by value; moves and takes ownership | self
//
// https://share.google/aimode/NsyM2USruFW5Bcbpf

// The primary difference between Fn, FnMut, and FnOnce lies in how they capture and
// interact with variables from their SURROUNDING environment!!!

#![allow(unused)]

fn do_something<F>(f: F)
where
  F: FnOnce(),
{
  f();
}

#[derive(Debug)]
struct City {
  name: String,
  years: Vec<u32>,
  populations: Vec<u32>,
}

impl City {
  fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
    Self {
      name: name.to_string(),
      years,
      populations,
    }
  }

  fn play_with_city_data<F>(&mut self, mut f: F)
  where
    F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
  {
    f(&mut self.years, &mut self.populations)
  }
}

fn main() {
  {
    let some_vec = vec![1, 2, 3];
    do_something(|| {
      some_vec
        .iter() // NOTE: not a consuming iterator; immutable borrow
        .for_each(|n| println!("The number is: {}", n));
    });

    // some_vec has been consumed by the closure in do_something(…)
    println!("{:?}", some_vec); // borrow of moved value: `some_vec`
  }

  println!();

  {
    let some_vec = vec![1, 2, 3];
    do_something(|| {
      some_vec // … is moved
        .into_iter() // NOTE: consuming iterator
        .for_each(|n| println!("The number is: {}", n));
    });

    //  ⚠️ some_vec has been consumed by the closure in do_something(…)
    // println!("{:?}", some_vec); // borrow of moved value: `some_vec`
  }

  println!();

  {
    let years = vec![
      1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
    ];

    let populations = vec![
      3250, 15300, 24000, 45900, 58800, 119800, 283071, 478974, 400378, 401694, 406703, 437619,
    ];

    println!("Sizes: {}, {}", years.len(), populations.len());

    let mut tallinn = City::new("Tallinn", years, populations);

    // 1. five first years
    tallinn.play_with_city_data(|years, populations| {
      let new_vec = years
        .into_iter()
        .zip(populations.into_iter())
        .take(5)
        .collect::<Vec<(_, _)>>();
      println!("(year, population): {:?}", new_vec);
    });

    // 2. add the year 2030
    tallinn.play_with_city_data(|years, populations| {
      years.push(2030);
      populations.push(500000);
    });

    // 3. remove the year 1834
    tallinn.play_with_city_data(|years, populations| {
      let position_option = years.iter().position(|year| *year == 1834);
      if let Some(position) = position_option {
        println!(
          "Going to delete {} at position {:?} now.",
          years[position], position
        );
        years.remove(position);
        populations.remove(position);
      }
    });

    println!(
      "Years left are {:?}\nPopulations left are {:?}",
      tallinn.years, tallinn.populations
    );
  }
}
