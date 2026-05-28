// Vectors
// https://dhghomon.github.io/easy_rust/Chapter_21.html
// https://www.youtube.com/watch?v=Eh-DsRnDKmw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  let name1 = String::from("Windy");
  let name2 = String::from("Gomesy");

  let mut my_vec = Vec::new();

  my_vec.push(name1);
  my_vec.push(name2);

  // println!("{}", name1); // rustc: borrow of moved value: `name1`

  println!("{}", my_vec[0]);
  println!("{}", my_vec[1]);

  println!();

  let name1 = String::from("Windy");
  let name2 = String::from("Gomesy");

  // type annotation
  let mut my_vec: Vec<String> = Vec::new();
  my_vec.push(name1);
  my_vec.push(name2);
  println!("{}", my_vec[0]);
  println!("{}", my_vec[1]);

  println!();

  let my_vec = vec![8, 10, 12];
  println!("length: {}", my_vec.len());
  println!("{}", my_vec[0]);
  println!("{}", my_vec[1]);
  println!("{}", my_vec[2]);

  println!("-- Slicing --");

  let my_vec: Vec<u32> = (1..11).collect();
  println!("{:?}", my_vec);
  println!("{}", my_vec.len());

  println!("Three to five: {:?}", &my_vec[2..5]);
  println!("start at two:  {:?}", &my_vec[1..]);
  println!("end at five:   {:?}", &my_vec[..5]);
  println!("everything:    {:?}", &my_vec[..]);

  println!();

  let vec_of_ten = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
  println!("{:?}", vec_of_ten);
  println!("{}", vec_of_ten.len());

  println!();

  let three_to_five = &vec_of_ten[2..5];
  let start_at_two = &vec_of_ten[1..];
  let end_at_five = &vec_of_ten[..5];
  let everything = &vec_of_ten[..];

  println!("Three to five: {:?}", three_to_five);
  println!("start at two:  {:?}", start_at_two);
  println!("end at five:   {:?}", end_at_five);
  println!("everything:    {:?}", everything);

  println!();

  // two reallocations
  let mut num_vec = Vec::new();
  println!("{}", num_vec.capacity()); // 0
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 4
  num_vec.push('a');
  num_vec.push('a');
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 4
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 8

  println!();

  for _ in 0..10000 {
    num_vec.push('a');
  }
  println!("{}", num_vec.capacity()); // 8

  println!();

  // no reallocations
  let mut num_vec = Vec::with_capacity(8);
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 8
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 8
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 8.
  num_vec.push('a');
  num_vec.push('a');
  println!("{}", num_vec.capacity()); // 8

  println!();

  let my_vec1: Vec<u8> = [1, 2, 3].into();
  let my_vec2: Vec<_> = [1, 2, 3].into();

  println!("{}", my_vec1.len());
  println!("{}", my_vec2.len());
}
