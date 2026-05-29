// Other Collections
// https://dhghomon.github.io/easy_rust/Chapter_32.html
// https://www.youtube.com/watch?v=Ovlt82Jz0GQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=DBdbe2QUlf8&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ph9CdWb9zXk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=hHTzhNci4VE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=r3VPQSdraaw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=oQsvekvt-oI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=9EOtSysFI-s&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ASZnjtCUNhs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=6CfwTBx9pos&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

struct City1 {
  name: String,
  population: HashMap<u32, u32>, // year and population
}

struct City2 {
  name: String,
  population: BTreeMap<u32, u32>, // year and population
}

// Calculates the remainder in the BinaryHeap.
// (Actually, an iterator would be faster than a function - we will learn them later.)
fn calc_remaining_numbers(input: &BinaryHeap<i32>) -> Vec<i32> {
  let mut remainder_vec = vec![];
  for number in input {
    remainder_vec.push(*number)
  }
  remainder_vec
}

fn print_unfinished_tasks(input: &VecDeque<(&str, bool)>) {
  for item in input {
    if !item.1 {
      println!("You must {}!", item.0);
    }
  }
}

fn finish_last_task(input: &mut VecDeque<(&str, bool)>) {
  let mut task_done = input.pop_back().unwrap();
  task_done.1 = true;
  input.push_front(task_done);
}

fn main() {
  // ////////////////////// //
  // HashMap (and BTreeMap) //
  // ////////////////////// //

  {
    let mut tallinn = City1 {
      name: "Tallinn".to_string(),
      population: HashMap::new(),
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    // not necessarily ordered by year
    for (year, population) in tallinn.population {
      println!(
        "In the year {} the city of {} had a population of {}.",
        year, tallinn.name, population
      );
    }
  }

  println!();

  {
    let mut tallinn = City2 {
      name: "Tallinn".to_string(),
      population: BTreeMap::new(), // only change
    };

    tallinn.population.insert(1372, 3_250);
    tallinn.population.insert(1851, 24_000);
    tallinn.population.insert(2020, 437_619);

    // ordered by year
    for (year, population) in tallinn.population {
      println!(
        "In the year {} the city of {} had a population of {}.",
        year, tallinn.name, population
      );
    }
  }

  println!();

  {
    let canadian_cities = vec!["Calgary", "Vancouver", "Gimli"];
    let german_cities = vec!["Karlsruhe", "Bad Doberan", "Bielefeld"];

    let mut city_hashmap = HashMap::new();

    for city in canadian_cities {
      city_hashmap.insert(city, "Canada");
    }

    for city in german_cities {
      city_hashmap.insert(city, "Germany");
    }

    println!("{:?}", city_hashmap["Bielefeld"]);
    println!("{:?}", city_hashmap.get("Bielefeld"));
    println!("{:?}", city_hashmap.get("Bielefeldt"));
  }

  println!();

  // existing entries are overwritten
  {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");
    book_hashmap.insert(1, "Le Petit Prince");
    book_hashmap.insert(1, "섀도우 오브 유어 스마일");
    book_hashmap.insert(1, "Eye of the World");

    println!("{:?}", book_hashmap.get(&1));
  }

  println!();

  {
    let mut book_hashmap = HashMap::new();

    book_hashmap.insert(1, "L'Allemagne Moderne");

    // if book_hashmap.get(&1).is_none() {
    if !book_hashmap.contains_key(&1) {
      book_hashmap.insert(1, "Le Petit Prince");
    }

    println!("{:?}", book_hashmap.get(&1));
  }

  println!();

  {
    let book_collection = vec![
      "L'Allemagne Moderne",
      "Le Petit Prince",
      "Eye of the World",
      "Eye of the World", // doublette
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
      book_hashmap.entry(book).or_insert(true);
    }

    for (book, true_or_false) in book_hashmap {
      println!("Do we have {}? {}", book, true_or_false);
    }
  }

  println!();

  {
    let book_collection = vec![
      "L'Allemagne Moderne",
      "Le Petit Prince",
      "Eye of the World",
      "Eye of the World",
    ];

    let mut book_hashmap = HashMap::new();

    for book in book_collection {
      let return_value = book_hashmap.entry(book).or_insert(0);
      *return_value += 1;
    }

    for (book, number) in book_hashmap {
      println!("{}, {}", book, number);
    }
  }

  println!();

  {
    // ratings of politicians
    let data = vec![
      ("male", 9),
      ("female", 5),
      ("male", 0),
      ("female", 6),
      ("female", 5),
      ("male", 10),
    ];

    let mut survey_hash = HashMap::new();

    for item in data {
      survey_hash.entry(item.0).or_insert(Vec::new()).push(item.1);
    }

    for (gender, count) in survey_hash {
      println!("{:?}: {:?}", gender, count);
    }
  }

  println!();

  // //////////////////// //
  // HashSet and BTreeSet //
  // //////////////////// //

  {
    let many_random_numbers = vec![
      94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
      51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
      35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
      96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82, 58,
      64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_hashset = HashSet::new();

    for number in &many_random_numbers {
      number_hashset.insert(number);
    }

    let hashset_length = number_hashset.len();
    println!(
      "There are {} unique numbers, so we are missing {}.",
      hashset_length,
      many_random_numbers.len() - hashset_length
    );

    // collect missing numbers in a vector
    let mut missing_vec = vec![];

    for number in 0..100 {
      // if number_hashset.get(&number).is_none() {
      if !number_hashset.contains(&number) {
        missing_vec.push(number);
      }
    }

    // prints random numbers in random order
    println!("Random numbers in random order:");
    for entry in number_hashset {
      print!("{} ", entry);
    }

    println!();

    // print missing numbers
    print!("It does not contain: ");
    for number in missing_vec {
      print!("{} ", number);
    }
    println!();
  }

  println!();

  {
    let many_random_numbers = vec![
      94, 42, 59, 64, 32, 22, 38, 5, 59, 49, 15, 89, 74, 29, 14, 68, 82, 80, 56, 41, 36, 81, 66,
      51, 58, 34, 59, 44, 19, 93, 28, 33, 18, 46, 61, 76, 14, 87, 84, 73, 71, 29, 94, 10, 35, 20,
      35, 80, 8, 43, 79, 25, 60, 26, 11, 37, 94, 32, 90, 51, 11, 28, 76, 16, 63, 95, 13, 60, 59,
      96, 95, 55, 92, 28, 3, 17, 91, 36, 20, 24, 0, 86, 82, 58, 93, 68, 54, 80, 56, 22, 67, 82, 58,
      64, 80, 16, 61, 57, 14, 11,
    ];

    let mut number_btreeset = BTreeSet::new(); // only change

    for number in many_random_numbers {
      number_btreeset.insert(number);
    }

    println!("Random numbers in numerical order:");
    for entry in number_btreeset {
      print!("{} ", entry);
    }
  }

  println!();
  println!();

  // ////////// //
  // BinaryHeap //
  // ////////// //

  // BinaryHeap = Priority Queue

  {
    let many_numbers_in_order = vec![0, 5, 10, 15, 20, 25, 30];

    let mut my_heap = BinaryHeap::new();

    for number in many_numbers_in_order {
      my_heap.push(number);
    }

    println!("Priority Queue: {:?}", &my_heap);

    while let Some(number) = my_heap.pop() {
      println!(
        "Popped off {}. Remaining numbers are: {:?}",
        number,
        calc_remaining_numbers(&my_heap)
      );
    }
  }

  println!();

  {
    let mut jobs = BinaryHeap::new();

    // adding jobs with their priorities
    jobs.push((100, "Write back to email from the CEO"));
    jobs.push((80, "Finish the report today"));
    jobs.push((5, "Watch some YouTube"));
    jobs.push((70, "Tell your team members thanks for always working hard"));
    jobs.push((30, "Plan who to hire next for the team"));

    println!("Jobs to do in order of their priorities:");
    while let Some(job) = jobs.pop() {
      println!("{}", job.1);
    }
  }

  println!();

  // //////// //
  // VecDeque //
  // //////// //

  // VecDeque = double-ended queue implemented with a growable ring buffer.

  {
    let mut my_vec = vec![9, 8, 7, 6, 5];
    println!("{:?}", my_vec);
    my_vec.remove(0);
    println!("{:?}", my_vec);
  }

  println!("\n----------------------------------------\n");

  // {
  //   // Too slow - don't run this code!
  //   let mut my_vec = vec![0; 600_000];
  //   for _ in 0..600000 {
  //     my_vec.remove(0);
  //   }
  //   println!("{:?}", my_vec);
  // }

  {
    // much faster
    let mut my_vec = VecDeque::from(vec![0; 600000]);
    for _ in 0..600000 {
      my_vec.pop_front();
    }
    println!("{:?}", my_vec);
  }

  println!();

  {
    let mut my_vecdeque = VecDeque::new();
    let things_to_do = vec![
      "send email to customer",
      "add new product to list",
      "phone Loki back",
    ];

    // populating VecDeque, thereby reversing order
    for thing in things_to_do {
      my_vecdeque.push_front((thing, false));
    }

    println!("{:?}", my_vecdeque);

    println!("Finishing last two tasks:");
    finish_last_task(&mut my_vecdeque);
    finish_last_task(&mut my_vecdeque);

    print_unfinished_tasks(&my_vecdeque);

    println!("{:?}", my_vecdeque);
  }
}
