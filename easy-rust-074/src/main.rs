// Chaining methods
// https://dhghomon.github.io/easy_rust/Chapter_35.html
// https://www.youtube.com/watch?v=j70jq4ynrSk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  // imperative style
  {
    let mut new_vec = Vec::new();
    let mut counter = 1;

    while counter < 11 {
      new_vec.push(counter);
      counter += 1;
    }

    println!("{:?}", new_vec);
  }

  // functional style
  {
    // with turbo fish
    let new_vec1 = (1..=10).collect::<Vec<i32>>();

    // the same:
    let new_vec2: Vec<i32> = (1..=10).collect();

    println!("{:?}", new_vec1);
    println!("{:?}", new_vec2);
  }

  // chaining
  {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec.into_iter().skip(3).take(4).collect::<Vec<i32>>();

    println!("{:?}", new_vec);
  }

  // the same with comments
  {
    let my_vec = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let new_vec = my_vec
      .into_iter() // create (consuming) iterator
      .skip(3) // skip over first three items
      .take(4) // take the next four
      .collect::<Vec<i32>>(); // put them in a new Vec<i32>

    println!("{:?}", new_vec);
  }
}
