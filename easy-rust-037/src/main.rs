// Destructuring
// https://dhghomon.github.io/easy_rust/Chapter_28.html
// https://www.youtube.com/watch?v=vJSb7-YcrHc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// References and the dot operator
// https://dhghomon.github.io/easy_rust/Chapter_29.html

#[derive(Debug)]
struct Person {
  name: String,
  real_name: String,
  height: u8,
  happiness: bool,
}

struct Item {
  number: u8,
}

impl Item {
  fn compare_number(&self, other_number: u8) {
    println!(
      "Are {} and {} equal? {}",
      self.number, // we don't need to write *self.number
      other_number,
      self.number == other_number
    );
  }
}

fn main() {
  // ///////////// //
  // Destructuring //
  // ///////////// //

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

  println!();

  // /////////////////////////////// //
  // References and the dot operator //
  // /////////////////////////////// //

  // {
  //   let my_number = 9;
  //   let reference = &my_number;
  //
  //   println!("{}", my_number == reference); // ⚠️ can't compare `{integer}` with `&{integer}`
  // }

  {
    let my_number = 9;
    let reference = &my_number;

    println!("{}", my_number == *reference);
  }

  // {
  //   let item = Item { number: 8 };
  //
  //   let reference_number = &item.number;
  //
  //   println!("{}", reference_number == 8); // ⚠️ can't compare `&u8` with `{integer}`
  // }

  {
    // with the dot operator, we don't need * (does the dereferencing for you)

    let item = Item { number: 8 };

    let reference_item = &item;

    println!("{}", reference_item.number == 8); // we don't need to write *reference_item.number
  }

  {
    // when you use the dot operator (.), you don't need to worry about dereferencing (*)

    let item = Item { number: 8 };

    let ref_item = &item;
    let ref_ref_item = &ref_item;

    // works in any case
    item.compare_number(8);
    ref_item.compare_number(8);
    ref_ref_item.compare_number(8);
  }
}
