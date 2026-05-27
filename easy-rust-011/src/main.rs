// More about printing
// https://dhghomon.github.io/easy_rust/Chapter_13.html
// https://www.youtube.com/watch?v=BdU9JphfBaI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=ycjZtvqyRHc&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  print!("\t Start with a tab\nand move to a new line");

  println!();
  println!();

  println!(
    "Inside quotes
you can write over
many lines
and it will print just fine."
  );

  println!();

  println!(
    "If you forget to write
    on the left side, the spaces
    will be added when you print."
  );

  println!();

  println!("Here are two escape characters: \n newline and \t tab");
  println!("Here are two escape characters: \\n and \\t");

  println!();

  // backslash binging ;-)
  println!(
    "He said, \"You can find the file at c:\\files\\my_documents\\file.txt.\" Then I found the file."
  );

  // raw strings `r#"..."#`
  println!(
    r#"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."#
  );
  println!(
    r##"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."##
  );
  println!(
    r###"He said, "You can find the file at c:\files\my_documents\file.txt." Then I found the file."###
  );

  println!();

  let my_string = "'Ice to see you,' he said.";
  let quote_string = r#""Ice to see you," he said."#;
  let hashtag_string = r##"The hashtag #IceToSeeYou had become very popular."##;
  let many_hashtags = r####""You don't have to type ### to use a hashtag. You can just use #.""####;

  println!(
    "{}\n{}\n{}\n{}\n",
    my_string, quote_string, hashtag_string, many_hashtags
  );

  println!();

  let _let = 6;
  let mut _mut = 10;

  // function inside of main
  fn r#return() -> u8 {
    println!("Here is your number.");
    8
  }

  let my_number = r#return();
  println!("{}", my_number);

  println!();

  println!("{}", "This will look like numbers:");
  println!("{:?}", b"This will look like numbers");
  println!("{:?}", br#"This will look like numbers"#);
  println!("{:?}", br##"This will look like numbers"##);

  println!();

  // pretty printing
  println!("{}", "This will look like numbers:");
  println!("{:#?}", b"This will look like numbers");

  println!();

  // print hexadecimal values
  println!("{:X}", '행' as u32);
  println!("{:X}", 'H' as u32);
  println!("{:X}", '居' as u32);
  println!("{:X}", 'い' as u32);

  println!();

  println!("{:x}", '행' as u32);
  println!("{:x}", 'h' as u32);
  println!("{:x}", '居' as u32);
  println!("{:x}", 'い' as u32);

  println!();

  // unicode (\u{…})
  println!("\u{D589}, \u{48}, \u{5C45}, \u{3044}");

  println!();

  // printing pointers
  let number = 9;
  let number_ref = &number;
  println!("{:p}", number_ref);
  println!("{:p}", &number_ref);
  println!("{:p}", &&number_ref);
  println!("{:p}", &&&number_ref);

  println!();

  // print number in different bases
  let number = 555;
  println!("binary:      {:b}", number);
  println!("hexadecimal: {:x}", number);
  println!("hexadecimal: {:X}", number);
  println!("octal:       {:o}", number);

  println!();

  // using indexes in println!
  let father_name = "Vlad";
  let son_name = "Adrian Fahrenheit";
  let family_name = "Țepeș";
  println!(
    "This is {1} {2}, son of {0} {2}.",
    father_name, son_name, family_name
  );

  println!();

  // naming indexes
  println!(
    "{city1} is in {country} and {city2} is also in {country}, but {city3} is not in {country}.",
    city1 = "Seoul",
    city2 = "Busan",
    city3 = "Tokyo",
    country = "Korea"
  );

  println!();

  // more complex stuff
  // https://doc.rust-lang.org/std/fmt/
  // {variable:padding alignment minimum.maximum}

  let letter = "a";
  println!("{:ㅎ^11}", letter);

  println!();

  let title = "TODAY'S NEWS";
  println!("{:-^30}", title); // no variable name, pad with -, put in centre, 30 characters long
  let bar = "|";
  println!("{: <15}{: >15}", bar, bar); // no variable name, pad with space, 15 characters each, one to the left, one to the right
  let a = "SEOUL";
  let b = "TOKYO";
  println!("{city1:-<15}{city2:->15}", city1 = a, city2 = b); // variable names city1 and city2, pad with -, one to the left, one to the right
}
