// Closures & Iterators
// https://dhghomon.github.io/easy_rust/Chapter_36.html
// https://dhghomon.github.io/easy_rust/Chapter_37.html
// https://www.youtube.com/watch?v=bLsGpFTrubo&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=sjq_0qCCQm0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=_1AJeCnGSmo&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=IX8KcuZBjtk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=f71I1XhLgqs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=5lqkQ1HFsyk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=eKENnjPeCwU&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=cgQNUCXTHEU&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=4YivPkdw53M&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=4ucNNpxd5Q4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=xi_MxKVyTqw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=uTWRaYfSvvM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=jXy4-AteA-g&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=RAABcA6BTVg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=e6kH3BK_vhk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=4prgsm70Hrc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=w91X8GUBx-k&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=oCLy_E64JTs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=L6rMIVRxwDc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=AX9FZ1MJOOo&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=3zs00nC0taY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=OgcrRt84bUY&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(unused)]

use std::collections::HashMap;

struct Company {
  name: String,
  ceo: Option<String>,
}

impl Company {
  fn new(name: &str, ceo: &str) -> Self {
    let ceo = match ceo {
      "" => None,
      ceo => Some(ceo.to_string()),
    };
    Self {
      name: name.to_string(),
      ceo,
    }
  }

  fn get_ceo(&self) -> Option<String> {
    self.ceo.clone()
  }
}

#[derive(Debug)]
struct Names {
  one_word: Vec<String>,
  two_words: Vec<String>,
  three_words: Vec<String>,
}

// any(…) - tests if any element of the iterator matches a predicate.
// contains(…) might be better for this example in real life.
fn in_char_vec(char_vec: &Vec<char>, check: char) {
  println!(
    "{} inside? {}",
    check,
    char_vec.iter().any(|&char| char == check)
  );
}

// an iterator which alternates between Some and None
#[derive(Clone)]
struct State {
  state: i32,
}

impl Iterator for State {
  type Item = i32; // NOTE: associated type: The type of the elements being iterated over.

  // Other functions use the associated type, too.
  // e.g. next(…), find(…), nth(…), …

  // the same:
  // fn next(&mut self) -> Option<i32> {
  fn next(&mut self) -> Option<Self::Item> {
    let state = self.state;
    self.state += 1;
    Some(state)
  }
}

#[derive(Debug, Clone)]
enum LibraryType {
  City,
  Country,
}

#[derive(Debug, Clone)]
struct Library {
  kind: LibraryType,
  books: Vec<String>,
}

impl Library {
  fn new() -> Self {
    Self {
      kind: LibraryType::City,
      books: Vec::new(),
    }
  }

  fn add_book(&mut self, book: &str) {
    self.books.push(book.to_string());
  }
}

impl Iterator for Library {
  type Item = String;

  fn next(&mut self) -> Option<String> {
    match self.books.pop() {
      Some(book) => Some(book + " is found!"),
      None => None,
    }
  }
}

fn main() {
  {
    // .iter()      iterator of references
    // .iter_mut()  iterator of mutable references
    // .into_iter() iterator of values; consuming iterator
  }

  {
    // Google AI: rust when to use for_each and map
    //
    // https://share.google/aimode/Ay0GbBahy8c3vhSWC

    let vector = vec![1, 2, 3];
    println!("{:?}", vector);

    let vector2: Vec<i32> = vector.iter().map(|x| x + 1).collect();
    let vector3: Vec<i32> = vector.into_iter().map(|x| x * 10).collect();

    println!("{:?}", vector2);
    println!("{:?}", vector3);

    let mut vector = vec![11, 22, 33];
    println!("{:?}", vector);

    vector.iter_mut().for_each(|x| *x += 100);
    println!("{:?}", vector);
  }

  println!();

  {
    let my_vec = vec!['a', 'b', '거', '柳'];

    let mut my_vec_iter = my_vec.iter();

    assert_eq!(my_vec_iter.next(), Some(&'a'));
    assert_eq!(my_vec_iter.next(), Some(&'b'));
    assert_eq!(my_vec_iter.next(), Some(&'거'));
    assert_eq!(my_vec_iter.next(), Some(&'柳'));
    assert_eq!(my_vec_iter.next(), None);
    assert_eq!(my_vec_iter.next(), None);
  }

  {
    let alternate = State { state: 0 };
    let result: Vec<i32> = alternate.into_iter().take(10).collect();
    println!("{:?}", result);
  }

  println!();

  {
    let mut my_library = Library::new();

    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    // item as the associated type (String)
    for item in my_library {
      println!("{}", item);
    }
  }

  println!();

  {
    let my_closure = || println!("This is a closure. Actually, an anonymous function.");
    my_closure();
  }

  println!();

  {
    let my_closure = |x: i32| println!("{}", x);

    my_closure(5); // 5
    my_closure(5 + 5); // 10
  }

  {
    let my_closure = || {
      let number = 7;
      let other_number = 10;
      println!("The two numbers are {} and {}.", number, other_number);
    };

    my_closure();
  }

  // a || that doesn't enclose a variable from outside is an ANONYMOUS FUNCTION.
  // a || that does enclose a variable from outside is a CLOSURE.

  // using vars from the outside
  {
    let number_one = 6;
    let number_two = 10;

    let my_closure = || println!("{}", number_one + number_two);

    my_closure(); // 16
  }

  {
    let number_one = 6;
    let number_two = 10;

    let my_closure = |x: i32| println!("{}", number_one + number_two + x);

    my_closure(5); // 21
  }

  {
    let my_vec = vec![8, 9, 10];

    let fourth = my_vec
      .get(3)
      .unwrap_or_else(|| if !my_vec.is_empty() { &my_vec[0] } else { &0 });

    println!("{}", fourth); // 8
  }

  println!();

  // unwrap_or_else(…)
  {
    let k = 10;
    assert_eq!(Some(4).unwrap_or_else(|| 2 * k), 4);
    assert_eq!(None.unwrap_or_else(|| 2 * k), 20);
  }

  println!();

  // map(…)
  {
    let num_vec = vec![1, 2, 3, 4, 5];

    let double_vec = num_vec
      .iter()
      .map(|n| n * 2) // for each item, multiply by two
      .collect::<Vec<i32>>();

    println!("{:?}", double_vec);
  }

  println!();

  // enumerate(…), for_each(…)
  {
    let num_vec = vec![1, 2, 3, 4, 5];

    num_vec
      .iter()
      .enumerate() // (index, number)
      .for_each(|(idx, n)| println!("index {} has number {}", idx, n));
  }

  println!();

  // map(…)
  {
    let num_vec = vec![10, 9, 8];

    // NOTE: without the `let _ =  `: unused `Map` that must be used iterators are lazy and do nothing unless consumed
    let _ = num_vec
      .iter()
      .enumerate()
      .map(|(idx, n)| println!("index {} has number {}..", idx, n));

    // no output …
  }

  // zip(…)
  {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];

    let number_word_hashmap = some_numbers
      .into_iter() // consuming iterator
      .zip(some_words.into_iter())
      .collect::<HashMap<_, _>>();

    println!("{:?}", number_word_hashmap);
  }

  // the same
  {
    let some_numbers = vec![0, 1, 2, 3, 4, 5];
    let some_words = vec!["zero", "one", "two", "three", "four", "five"];
    let number_word_hashmap: HashMap<_, _> = some_numbers // Because we tell it the type here …
      .into_iter()
      .zip(some_words.into_iter())
      .collect(); // … we don't have to tell it here!

    println!("{:?}", number_word_hashmap);
  }

  println!();

  // char_indices(…)
  {
    let numbers_together = "140399923481800622623218009598281";

    println!(
      "{}: {} chars",
      numbers_together,
      numbers_together.char_indices().count()
    );

    for (idx, n) in numbers_together.char_indices() {
      match (idx % 3, n) {
        (0..=1, n) => print!("{}", n),
        _ => print!("{} ", n),
      }
    }
    println!();
  }

  println!();

  // don't care variables (_)
  {
    let my_vec = vec![8, 9, 10];

    println!(
      "{:?}",
      my_vec
        .iter()
        .for_each(|_| println!("We didn't use the variable at all."))
    );
  }

  {
    // Google AI: rust difference between Fn, FnMut, FnOnce
    //
    // Fn     | takes by reference                        | &self
    // FnMut  | takes by mutable reference                | &mut self
    // FnOnce | takes by value; moves and takes ownership | self
    //
    // https://share.google/aimode/NsyM2USruFW5Bcbpf

    // The primary difference between Fn, FnMut, and FnOnce lies in how they capture and
    // interact with variables from their SURROUNDING environment!!!

    // Rust uses the function type with the least authority.
    // Fn in this case.
    let my_closure = || println!();

    my_closure();
  }

  // filter(…) (filters in, not out)
  {
    let months = vec![
      "January",
      "February",
      "March",
      "April",
      "May",
      "June",
      "July",
      "August",
      "September",
      "October",
      "November",
      "December",
    ];

    let filtered_months: Vec<&str> = months
      .into_iter()
      .filter(|month| month.len() < 5)
      .filter(|month| month.contains("u"))
      .collect();

    println!("{:?}", filtered_months);
  }

  println!();

  // filter_map(…); map and then filter
  {
    let a = ["1", "two", "NaN", "four", "5"];

    let a: Vec<i32> = a.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
    println!("{:?}", a);
  }

  println!();

  {
    let companies = vec![
      Company::new("Umbrella Corporation", "Unknown"),
      Company::new("Ovintiv", "Doug Suttles"),
      Company::new("The Red-Headed League", ""),
      Company::new("Stark Enterprises", ""),
    ];

    let ceos: Vec<String> = companies
      .into_iter()
      .filter_map(|company| company.get_ceo())
      .collect();

    println!("Ceo's: {:?}", ceos);
  }

  println!();

  {
    // Google AI: rust what is the difference between ok, ok_or, ok_or_else
    //
    // The core difference is that ok converts a Result into an Option, while ok_or and
    // ok_or_else do the exact opposite by converting an Option into a Result.
    //
    // https://share.google/aimode/AysscnzBBwITbiY92
  }

  // ok(…) - converts `Result<T, E>` to `Option<T>`.
  {
    let user_input = vec![
      "8.9",
      "Nine point nine five",
      "8.0",
      "7.6",
      "eleventy-twelve",
    ];

    let actual_numbers: Vec<f32> = user_input
      .into_iter()
      .filter_map(|input| input.parse::<f32>().ok())
      .collect();

    println!("{:?}", actual_numbers);
  }

  println!();

  // ok_or(…) - converts `Option<T>` into a `Result<T, E>`
  {
    let company_vec = vec![
      Company::new("Umbrella Corporation", "Unknown"),
      Company::new("Ovintiv", "Doug Suttles"),
      Company::new("The Red-Headed League", ""),
      Company::new("Stark Enterprises", ""),
    ];

    let mut results_vec = vec![]; // Pretend we need to gather error results too

    company_vec
      .iter()
      .for_each(|company| results_vec.push(company.get_ceo().ok_or("No CEO found")));

    println!("{:?}", results_vec);
  }

  println!();

  // ok_or_else(…) - converts the `Option<T>` into a `Result<T, E>`
  // ok_or -> using a value
  // …_else -> using a closure (with no arguments)
  {
    let company_vec = vec![
      Company::new("Umbrella Corporation", "Unknown"),
      Company::new("Ovintiv", "Doug Suttles"),
      Company::new("The Red-Headed League", ""),
      Company::new("Stark Enterprises", ""),
    ];

    let mut results_vec = vec![];

    company_vec.iter().for_each(|company| {
      results_vec.push(
        company
          .get_ceo()
          .ok_or_else(|| format!("No CEO found for `{}`", company.name)),
      )
    });

    println!("{:?}", results_vec);
  }

  println!();

  // and_then(…) - some languages call this operation flatmap
  //
  // Allows Happy-Path-Programming!
  //
  // lazily evaluated
  //
  // None remains None.
  // Wrapped value of Some(…) is applied to a closure.
  // Result is an Option<…>.
  {
    let new_vec = vec![0, 1, 2, 3, 4, 5];

    let number_to_add = 42;
    let mut results = vec![];

    for index in 0..10 {
      results.push(
        new_vec
          .get(index)
          .and_then(|n| Some(n + 1))
          .and_then(|n| Some(n + number_to_add)),
      );
    }

    println!("{:?}", results);
  }

  println!();

  // classic `and`
  {
    let one = true;
    let two = false;
    let three = true;
    let four = true;

    println!("{}", one && three);
    println!("{}", one && two && three && four);
  }

  println!();

  // and(…) - `and` for Options (takes an Option and returns an Option, eagerly evaluated)
  // Returns `None` if the option is `None`, otherwise returns the other Option.
  {
    let first_try = vec![
      Some("success 1!"),
      None,
      Some("success 1!"),
      Some("success 1!"),
      None,
    ];

    let second_try = vec![
      None,
      Some("success 2!"),
      Some("success 2!"),
      Some("success 2!"),
      Some("success 2!"),
    ];

    let third_try = vec![
      Some("success 3!"),
      Some("success 3!"),
      Some("success 3!"),
      Some("success 3!"),
      None,
    ];

    for i in 0..first_try.len() {
      println!(
        "index {}: {:?}",
        i,
        first_try[i].and(second_try[i]).and(third_try[i])
      );
    }
  }

  println!();

  // any(…), all(…)
  {
    let char_vec = ('a'..'働').collect::<Vec<char>>();
    println!("Number of chars: {}", char_vec.len());

    // any(…)
    in_char_vec(&char_vec, 'i');
    in_char_vec(&char_vec, '뷁');
    in_char_vec(&char_vec, '鑿');

    let smaller_vec = ('A'..'z').collect::<Vec<char>>();

    println!(
      "All alphabetic? {}",
      smaller_vec.iter().all(|x| x.is_alphabetic())
    );

    println!(
      "All less than the character 행? {}",
      smaller_vec.iter().all(|x| x < &'행')
    );
  }

  println!();

  {
    let mut big_vec = vec![6; 1000];
    big_vec.push(5);

    println!("{:?}", big_vec.iter().rev().any(|&n| n == 5));
  }

  println!();

  // rev(…)
  {
    let mut big_vec = vec![6; 1_000_000];
    big_vec.push(5);

    let mut counter = 0;
    let mut big_iter = big_vec.iter();

    loop {
      counter += 1;
      if big_iter.next() == Some(&5) {
        break;
      }
    }

    println!("Final counter is: {}", counter);

    let mut counter = 0;
    let mut rev_big_iter = big_vec.iter().rev();

    loop {
      counter += 1;
      if rev_big_iter.next() == Some(&5) {
        break;
      }
    }

    println!("Final counter is: {}", counter);
  }

  println!();

  // find(…), position(…)
  {
    let num_vec = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];

    println!("{:?}", num_vec.iter().find(|&n| n % 3 == 0)); // find returns an element
    println!("{:?}", num_vec.iter().find(|&n| n * 2 == 30));

    println!("{:?}", num_vec.iter().position(|&n| n % 3 == 0)); // position returns an index
    println!("{:?}", num_vec.iter().position(|&n| n * 2 == 30));

    // `position` might panic if the iterator has more than `usize::MAX` non-matching elements
    println!("usize::MAX: {}", usize::MAX); // 18446744073709551615 (0xFFFFFFFFFFFFFFFF)
  }

  println!();

  // cycle(…), zip(…)
  {
    let even_odd = vec!["even", "odd"];

    let even_odd_vec: Vec<(i32, &str)> = (0..=10).zip(even_odd.into_iter().cycle()).collect();

    println!("{:?}", even_odd_vec);
  }

  println!();

  // cycle(…)
  {
    let a = [1, 2, 3];

    let mut iter = a.into_iter().cycle();

    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    // …
  }

  println!();

  // ranges as iterators
  {
    let ten_chars: Vec<char> = ('a'..).take(10).collect();
    let skip_then_ten_chars: Vec<char> = ('a'..).skip(1300).take(10).collect();

    println!("{:?}", ten_chars);
    println!("{:?}", skip_then_ten_chars);
  }

  println!();

  // fold(…)
  {
    println!("{}", (1..100).fold(0, |acc, n| acc + n)); // classic Gauss: 4950
  }

  println!();

  {
    let a_string = "I don't have any dashes in me.";
    println!("{}", a_string);

    println!(
      "{}",
      a_string.chars().fold("-".to_string(), |mut acc, char| {
        acc.push(char);
        acc.push('-');
        acc
      })
    );
  }

  println!();

  // chunks(…), window(…)
  {
    let num_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for chunk in num_vec.chunks(3) {
      println!("{:?}", chunk);
    }

    println!();
    for window in num_vec.windows(3) {
      println!("{:?}", window);
    }
  }

  println!();

  // match_indices(…)
  {
    let rules = "Rule number 1: No fighting. Rule number 2: Go to bed at 8 pm. Rule number 3: Wake up at 6 am.";

    let rule_locations: Vec<_> = rules.match_indices("Rule").collect();

    println!("{:?}", rule_locations);
  }

  println!();

  // peekable(…), peek(…)
  {
    let just_numbers = vec![1, 5, 100];
    let mut number_iter = just_numbers.iter().peekable();

    for _ in 0..just_numbers.len() {
      println!("I love the number {}", number_iter.peek().unwrap());
      println!("I really love the number {}", number_iter.peek().unwrap());
      println!("{} is such a nice number", number_iter.peek().unwrap());
      println!();
      number_iter.next();
    }
  }

  println!();

  // peekable(…), peek(…)
  {
    let locations = vec![
      ("Nevis", 25),
      ("Taber", 8428),
      ("Markerville", 45),
      ("Cardston", 3585),
    ];

    let mut loc_iter = locations.iter().peekable();

    while loc_iter.peek().is_some() {
      match loc_iter.peek() {
        Some((name, inhabitants)) if inhabitants < &100 => {
          println!("Found a hamlet: {} with {} people", name, inhabitants)
        }
        Some((name, inhabitants)) => println!("Found a town: {} with {} people", name, inhabitants),
        _ => break,
      }
      loc_iter.next();
    }
  }

  println!();

  // .match_indices(…)
  // example for Clippy the linter
  {
    let names = [
      "Caesar",
      "Frodo Baggins",
      "Bilbo Baggins",
      "Jean-Luc Picard",
      "Data",
      "Rand Al'Thor",
      "Paul Atreides",
      "Donald John Trump",
    ];

    let mut names_iter = names.iter().peekable();

    let mut all_names = Names {
      one_word: vec![],
      two_words: vec![],
      three_words: vec![],
    };

    while names_iter.peek().is_some() {
      let next_item = names_iter.next().unwrap();
      // match next_item.match_indices(' ').collect::<Vec<_>>().len() {
      match next_item.match_indices(' ').count() {
        0 => all_names.one_word.push(next_item.to_string()),
        1 => all_names.two_words.push(next_item.to_string()),
        2 => all_names.three_words.push(next_item.to_string()),
        _ => (),
      }
    }

    println!("{:#?}", all_names);
  }
}
