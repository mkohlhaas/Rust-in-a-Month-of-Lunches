// Structs
// https://dhghomon.github.io/easy_rust/Chapter_24.html
// https://www.youtube.com/watch?v=W23uQghBOFk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=GSVhrjLCuNA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Unit Struct
#[derive(Debug)]
struct FileDirectory;

// Tuple Struct
#[derive(Debug)]
struct Color(u8, u8, u8);

// Named Struct
#[derive(Debug)]
struct SizeColor {
  size: u32,
  color: Color,
}

#[derive(Debug)]
struct Country {
  population: u32,
  capital: String,
  leader_name: String,
}

fn main() {
  {
    let my_color = Color(50, 0, 50);

    println!("red:   {}", my_color.0);
    println!("green: {}", my_color.1);
    println!("blue:  {}", my_color.2);
    println!("The second part of the colour is: {}", my_color.1);
  }

  {
    let fd = FileDirectory;
    println!("{:?}", fd);
  }

  {
    let my_color = Color(50, 0, 50);

    let size_and_color = SizeColor {
      size: 150,
      color: my_color,
    };
    println!("{:?}", size_and_color);
    println!("{:?}", size_and_color.size);
    println!("{:?}", size_and_color.color);
  }

  {
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    let kalmykia = Country {
      population: population,
      capital: capital,
      leader_name: leader_name,
    };

    println!("{:?}", kalmykia);
    println!("{:?}", kalmykia.population);
    println!("{:?}", kalmykia.capital);
    println!("{:?}", kalmykia.leader_name);
  }

  {
    let population = 500_000;
    let capital = String::from("Elista");
    let leader_name = String::from("Batu Khasikov");

    // without hiccup
    let kalmykia = Country {
      population,
      capital,
      leader_name,
    };

    println!("{:?}", kalmykia);
    println!("{:?}", kalmykia.population);
    println!("{:?}", kalmykia.capital);
    println!("{:?}", kalmykia.leader_name);
  }
}
