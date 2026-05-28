// Destructuring
// https://dhghomon.github.io/easy_rust/Chapter_28.html
// https://www.youtube.com/watch?v=vJSb7-YcrHc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
struct Person {
  name: String,
  real_name: String,
  height: u8,
  happiness: bool,
}

fn main() {
  {
    let papa_doc = Person {
      name: "Papa Doc".to_string(),
      real_name: "Clarence".to_string(),
      height: 170,
      happiness: false,
    };

    println!("{:?}", papa_doc);

    // destructuring a struct
    let Person {
      name,
      real_name,
      height,
      happiness,
    } = papa_doc;

    println!(
      "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
      name, real_name, height, happiness
    );
  }

  println!();

  {
    let papa_doc = Person {
      name: "Papa Doc".to_string(),
      real_name: "Clarence".to_string(),
      height: 170,
      happiness: false,
    };

    println!("{:?}", papa_doc);

    // giving different names
    let Person {
      name: a,
      real_name: b,
      height: c,
      happiness: d,
    } = papa_doc;

    println!(
      "They call him {} but his real name is {}. He is {} cm tall and is he happy? {}",
      a, b, c, d
    );
  }

  println!();

  {
    let papa_doc = Person {
      name: "Papa Doc".to_string(),
      real_name: "Clarence".to_string(),
      height: 170,
      happiness: false,
    };

    println!("{:?}", papa_doc);

    // don't care variables with ..
    let Person {
      name: a,
      real_name: b,
      ..
    } = papa_doc;

    println!("They call him {} but his real name is {}.", a, b);
  }
}
