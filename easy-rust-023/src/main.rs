// Tuples
// https://dhghomon.github.io/easy_rust/Chapter_22.html
// https://www.youtube.com/watch?v=U67Diy6SlTg&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// returns an empty tuple ()
fn just_prints() {
  println!("I am printing.");
}

fn main() {
  just_prints();

  println!();

  let random_tuple = ("Here is a name", 8, vec!['a'], 'b', [8, 9, 10], 7.7);
  println!(
    "First item:  {:?}
Second item: {:?}
Third item:  {:?}
Fourth item: {:?}
Fifth item:  {:?}
Sixth item:  {:?}",
    random_tuple.0, random_tuple.1, random_tuple.2, random_tuple.3, random_tuple.4, random_tuple.5,
  );

  // Pseudo Destructuring

  let str_vec = vec!["one", "two", "three"];
  let (a, b, c) = (str_vec[0], str_vec[1], str_vec[2]);
  println!("{:?}", a);
  println!("{:?}", b);
  println!("{:?}", c);

  let (_, _, c) = (str_vec[0], str_vec[1], str_vec[2]);
  println!("{:?}", c);
}
