// Enums
// https://dhghomon.github.io/easy_rust/Chapter_25.html
// https://www.youtube.com/watch?v=SRnqNTJUgjs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=F_EcbWM63lk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=2uh64U9JesA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=LOHVUYTc5Us&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#[derive(Debug)]
enum ThingsInTheSky1 {
  Sun,
  Stars,
}

#[derive(Debug)]
enum ThingsInTheSky2 {
  Sun(String),
  Stars(u32),
}

enum Mood {
  Happy,
  Sleepy,
  NotBad,
  Angry,
}

enum Season {
  Spring, // If this was Spring(String) or something it wouldn't work
  Summer,
  Autumn,
  Winter,
}

enum Star {
  BrownDwarf = 10,
  RedDwarf = 50,
  YellowStar = 100,
  RedGiant = 1000,
  DeadStar,
}

#[derive(Debug)]
enum Number {
  U32(u32),
  I32(i32),
}

fn create_skystate1(time: u8) -> ThingsInTheSky1 {
  match time {
    6..=18 => ThingsInTheSky1::Sun,
    _ => ThingsInTheSky1::Stars,
  }
}

fn check_skystate1(state: &ThingsInTheSky1) {
  match state {
    ThingsInTheSky1::Sun => println!("I can see the sun!"),
    ThingsInTheSky1::Stars => println!("I can see the stars!"),
  }
}

fn create_skystate2(time: u8) -> ThingsInTheSky2 {
  match time {
    6..=18 => ThingsInTheSky2::Sun(String::from("I can see the sun!")),
    _ => ThingsInTheSky2::Stars(123),
  }
}

fn check_skystate2(state: &ThingsInTheSky2) {
  match state {
    ThingsInTheSky2::Sun(desc) => println!("{}", desc),
    ThingsInTheSky2::Stars(num_stars) => println!("I can see {} stars.", num_stars),
  }
}

fn match_mood1(mood: &Mood) -> u8 {
  let happiness_level = match mood {
    Mood::Happy => 10,
    Mood::Sleepy => 6,
    Mood::NotBad => 7,
    Mood::Angry => 2,
  };
  happiness_level
}

// importing Mood
fn match_mood2(mood: &Mood) -> u8 {
  use Mood::*;
  let happiness_level = match mood {
    Happy => 10, // We don't have to write Mood:: anymore
    Sleepy => 6,
    NotBad => 7,
    Angry => 2,
  };
  happiness_level
}

fn get_number(input: i32) -> Number {
  match input.is_positive() {
    true => Number::U32(input as u32),
    false => Number::I32(input),
  }
}

fn main() {
  {
    let sun = ThingsInTheSky1::Sun;
    let stars = ThingsInTheSky1::Stars;

    println!("{:?}", sun);
    println!("{:?}", stars);
  }

  {
    let time = 5;
    let skystate = create_skystate1(time);
    check_skystate1(&skystate);
  }

  {
    let time = 5;
    let skystate = create_skystate2(time);
    check_skystate2(&skystate);
  }

  println!();

  {
    let happy = Mood::Happy;
    let happiness_level = match_mood1(&happy);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let sleepy = Mood::Sleepy;
    let happiness_level = match_mood1(&sleepy);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let not_bad = Mood::NotBad;
    let happiness_level = match_mood1(&not_bad);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let angry = Mood::Angry;
    let happiness_level = match_mood1(&angry);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);
  }

  println!();

  {
    let happy = Mood::Happy;
    let happiness_level = match_mood2(&happy);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let sleepy = Mood::Sleepy;
    let happiness_level = match_mood2(&sleepy);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let not_bad = Mood::NotBad;
    let happiness_level = match_mood2(&not_bad);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);

    let angry = Mood::Angry;
    let happiness_level = match_mood2(&angry);
    println!("Out of 1 to 10 my happiness level is {}.", happiness_level);
  }

  println!();

  // convert enums to numbers
  {
    use Season::*;
    let four_seasons = vec![Spring, Summer, Autumn, Winter];
    for season in four_seasons {
      println!("{}", season as u32);
    }
  }

  println!();

  {
    use Star::*;
    let starvec = vec![BrownDwarf, RedDwarf, YellowStar, RedGiant];

    for star in starvec {
      match star as u16 {
        size if size < 100 => println!("Not the biggest star."),
        size if size < 999 => println!("This is a good-sized star."),
        _ => println!("That star is pretty big!"),
      }
    }
    println!("What about DeadStar? It's the number {}.", DeadStar as u16);
  }

  println!();

  {
    let n1 = Number::U32(42);
    let n2 = Number::I32(-42);

    println!("{:?}", n1);
    println!("{:?}", n2);

    match n1 {
      Number::U32(n) => println!("{}", n),
      Number::I32(n) => println!("{}", n),
    }

    match n2 {
      Number::U32(n) => println!("{}", n),
      Number::I32(n) => println!("{}", n),
    }
  }

  {
    let my_vec = vec![get_number(-800), get_number(8)];

    use Number::*;

    for item in my_vec {
      match item {
        U32(n) => println!("It's an u32 with the value {}.", n),
        I32(n) => println!("It's an i32 with the value {}.", n),
      }
    }
  }
}
