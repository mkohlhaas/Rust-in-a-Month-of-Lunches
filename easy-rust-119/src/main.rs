// Multiple threads
// https://dhghomon.github.io/easy_rust/Chapter_46.html
// https://www.youtube.com/watch?v=iNJ4PcdO2-0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=DO4la8vhcvk&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=iw3WysQeAGE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// NOTE: In the videos is much better code presented!

use std::thread::spawn;

fn main() {
  {
    spawn(|| {
      println!("I am printing something #1");
    });
  }

  println!();

  {
    for _ in 0..10 {
      spawn(|| {
        println!("I am printing something #2");
      });
    }
  }

  println!();

  {
    for _ in 0..10 {
      spawn(|| {
        println!("I am printing something #3");
      });
    }

    for _ in 0..1_000_000 {
      // Make the program declare "let x = 9" one million times.
      // It has to finish this before it can exit the main function.
      let _x = 9;
    }
  }

  println!();

  {
    for _ in 0..10 {
      let _handle = spawn(|| {
        println!("I am printing something #4");
      });
    }
  }

  println!();

  {
    for _ in 0..10 {
      // NOTE: should be a vec of handles that can join the main thread
      let handle = spawn(|| {
        println!("I am printing something #5");
      });

      // NOTE: here we should call .join() for every spawned thread
      let _ = handle.join(); // wait for the threads to finish
    }
  }

  println!();

  // Google AI: rust difference between Fn, FnMut, FnOnce
  //
  // Fn     | takes by reference                        | &self
  // FnMut  | takes by mutable reference                | &mut self
  // FnOnce | takes by value; moves and takes ownership | self
  //
  // https://share.google/aimode/NsyM2USruFW5Bcbpf

  // The primary difference between Fn, FnMut, and FnOnce lies in how they capture and
  // interact with variables from their SURROUNDING environment!!!

  // Fn
  {
    let my_string = String::from("I will go into the closure #1");
    let my_closure = || println!("{}", my_string);
    my_closure();
    my_closure();
  }

  println!();

  // FnMut
  {
    let mut my_string = String::from("I will go into the closure #2");
    let mut my_closure = || {
      my_string.push_str(" now");
      println!("{}", my_string);
    };
    my_closure();
    my_closure();
  }

  println!();

  // FnOnce
  {
    let my_vec: Vec<i32> = vec![8, 9, 10];
    let my_closure = || {
      my_vec
        .into_iter() // into_iter takes ownership
        .map(|x| x as u8) // turn it into u8
        .map(|x| x * 2) // multiply by 2
        .collect::<Vec<u8>>() // collect into a Vec
    };
    let new_vec = my_closure();
    println!("{:?}", new_vec);
  }

  println!();

  // {
  //   let my_string = String::from("Can I go inside the thread? #1");
  //
  //   // ⚠️ closure may outlive the current function; it borrows `my_string`, which is owned by the current function
  //   let handle = std::thread::spawn(|| {
  //     println!("{}", my_string);
  //   });
  //
  //   handle.join();
  // }

  // {
  //   let my_string = String::from("Can I go inside the thread? #2");
  //
  //   // ⚠️ cclosure may outlive the current function, but it borrows `my_string`, which is owned by the current function
  //   let handle = std::thread::spawn(|| {
  //     println!("{}", my_string); // now my_string is being used as a reference
  //   });
  //
  //   // ⚠️ cannot move out of `my_string` because it is borrowed
  //   drop(my_string);
  //
  //   handle.join();
  // }

  // move
  // {
  //   let my_string = String::from("Can I go inside the thread? #3");
  //
  //   let handle = std::thread::spawn(move || {
  //     println!("{}", my_string);
  //   });
  //
  //   drop(my_string); // ⚠️ we can't drop, because handle has it. So this won't work
  //
  //   handle.join();
  // }

  // `move` converts any variables captured by reference or mutable reference to variables captured by value.
  //
  // Note: `move` closures may still implement [`Fn`](https://doc.rust-lang.org/stable/core/ops/function/trait.Fn.html) or [`FnMut`](https://doc.rust-lang.org/stable/core/ops/function/trait.FnMut.html), even though
  // they capture variables by `move`. This is because the traits implemented by
  // a closure type are determined by *what* the closure does with captured
  // values, not *how* it captures them:
  {
    let my_string = String::from("Can I go inside the thread? #4");

    let handle = spawn(move || {
      println!("{}", my_string);
    });

    handle.join().unwrap();
  }
}
