// Using files
// https://dhghomon.github.io/easy_rust/Chapter_64.html
// https://www.youtube.com/watch?v=kLBmgtMyLf4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=qXU1tZ0-cFA&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=hLmV4YXvFwI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=epLChR7TXRw&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

#![allow(dead_code)]

use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::num::ParseIntError;

fn give_number(input: &str) -> Result<i32, ParseIntError> {
  input.parse::<i32>()
}

// fn main() -> Result<(), ParseIntError> {
//   {
//     println!("{:?}", give_number("88")?);
//     println!("{:?}", give_number("5")?);
//     Ok(())
//   }
// }

fn main() -> std::io::Result<()> {
  {
    let mut file = fs::File::create("myfilename.txt")?;
    file.write_all(b"Let's put this in the file.")?;
  }

  {
    fs::write("calvin_with_dad_1.txt", 
"Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
Calvin: Really?
Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;
  }

  {
    fs::write("calvin_with_dad_2.txt", 
"Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
Calvin: Really?
Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    let mut calvin_file = File::open("calvin_with_dad_2.txt")?;
    let mut calvin_string = String::new();
    calvin_file.read_to_string(&mut calvin_string)?;

    calvin_string
      .split_whitespace()
      .for_each(|word| print!("{} ", word.to_uppercase()));
  }

  //   {
  //     fs::write("calvin_with_dad_3.txt",
  // "Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
  // Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
  // Calvin: Really?
  // Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;
  //
  //     OpenOptions::new()
  //       .write(true)
  //       .create_new(true)
  //       .open("calvin_with_dad_3.txt")?;
  //   }

  {
    fs::write("calvin_with_dad_4.txt",
  "Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
  Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
  Calvin: Really?
  Dad: Yep. The world didn't turn color until sometimes in the 1930s...")?;

    let mut calvin_file = OpenOptions::new()
      .append(true)
      .read(true)
      .open("calvin_with_dad_4.txt")?;

    calvin_file.write_all(b"And it was a pretty grainy color for a while too.\n")?;
    write!(&mut calvin_file, "That's really weird.\n")?;
    write!(&mut calvin_file, "Well, truth is stranger than fiction.")?;

    println!("{}", fs::read_to_string("calvin_with_dad_4.txt")?);
  }

  Ok(())
}
