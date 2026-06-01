// Interior mutability
// https://dhghomon.github.io/easy_rust/Chapter_41.html
// https://www.youtube.com/watch?v=MMcG-bXEBOM&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=xLhI6c0BDgI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=lM9xkW_2Ixs&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=4jKWpv5Fmz0&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=15Vbo7KK9H8&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=z3G_7_hNltE&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=HXpssrRV784&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Cell    | set(…), get(…)
// RefCell | borrow(…), borrow_mut(…), dereference with `*`
// Mutex   | lock(…), dereference with `*`
// RwLock  | read(…), write(…)

#![allow(unused)]

use std::cell::{Cell, RefCell}; // NOTE: not thread-safe!
use std::sync::{Mutex, MutexGuard, RwLock}; // NOTE: thread-safe!

#[derive(Debug)]
struct Book<'a> {
  title: Cell<&'a str>,
  author: RefCell<&'a str>,
}

#[derive(Debug)]
struct PhoneModel {
  company_name: String,
  model_name: String,
  screen_size: f32,
  memory: usize,
  date_issued: u32,
  on_sale: Cell<bool>,
}

#[derive(Debug)]
struct User {
  id: u32,
  year_registered: u32,
  username: String,
  active: RefCell<bool>,
  // …
}

fn main() {
  // Cell
  {
    let super_phone_3000 = PhoneModel {
      company_name: "YY Electronics".to_string(),
      model_name: "Super Phone 3000".to_string(),
      screen_size: 7.5,
      memory: 4_000_000,
      date_issued: 2020,
      on_sale: Cell::new(true),
    };

    println!("{:#?}", super_phone_3000);

    // 10 years later, super_phone_3000 is not on sale anymore
    super_phone_3000.on_sale.set(false);

    println!("On sale: {}", super_phone_3000.on_sale.get());

    println!("{:#?}", super_phone_3000);
  }

  println!();

  // RefCell - a mutable memory location with dynamically (aka at run-time!) checked borrow rules
  // • many borrows is ok
  // • one mutable borrow is ok
  // • mutable and immutable together is not ok
  {
    let user = User {
      id: 1,
      year_registered: 2020,
      username: "John".to_string(),
      active: RefCell::new(true),
    };

    // using replace(…)
    println!("{:?}", user.active);
    user.active.replace(false);
    println!("{:?}", user.active);

    let date = 2020;

    // using replace_with(…)
    user
      .active
      .replace_with(|_| if date > 2000 { true } else { false });
    println!("{:?}", user.active);
  }

  println!();

  {
    let user = User {
      id: 1,
      year_registered: 2020,
      username: "John".to_string(),
      active: RefCell::new(true),
    };

    // change the value
    *user.active.borrow_mut() = false;

    println!("{:?}", user); // active: RefCell { value: false }

    let borrow1 = user.active.borrow_mut(); // first mutable borrow - okay
    println!("{:?}", borrow1); // true

    println!("{:?}", user); // active: RefCell { value: <borrowed> }

    // let borrow2 = user.active.borrow_mut(); // second mutable borrow - not okay; ⚠️ panics at run-time!
  }

  println!();

  // Mutex - a mutual exclusion primitive useful for protecting shared data
  {
    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex); // data: 5

    // lock the mutex
    let mut mutex_guard: MutexGuard<'_, i32> = my_mutex.lock().unwrap();

    println!("{:?}", my_mutex); // data: "<locked>"

    println!("{:?}", mutex_guard); // 5

    // deref mutex changer
    *mutex_guard = 6;
    println!("{:?}", mutex_guard); // 6
  }

  println!();

  // A Mutex changer unlocks automatically when it goes out of scope.
  {
    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex); // data: 5
    {
      let mut mutex_guard = my_mutex.lock().unwrap();
      println!("{:?}", my_mutex); // data: "<locked>"
      *mutex_guard = 6;
    }

    println!("{:?}", my_mutex); // data: 6
  }

  println!();

  // drop(…)
  {
    let my_mutex = Mutex::new(5);
    println!("{:?}", my_mutex); // data: 5
    //
    let mut mutex_guard = my_mutex.lock().unwrap();
    println!("{:?}", my_mutex); // data: "<locked>"
    *mutex_guard = 6;

    // it will be gone and my_mutex is unlocked automatically
    std::mem::drop(mutex_guard);

    println!("{:?}", my_mutex); // data: 6
  }

  println!();

  // {
  //   let my_mutex = Mutex::new(5);
  //
  //   let mut mutex_guard1 = my_mutex.lock().unwrap();
  //   let mut mutex_guard2 = my_mutex.lock().unwrap(); // ⚠️ waits forever!!!
  //
  //   println!("This will never print...");
  // }

  // try_lock(…)
  {
    let my_mutex = Mutex::new(5);
    let mut mutex_guard1 = my_mutex.lock().unwrap();

    let mut mutex_guard2 = my_mutex.try_lock();

    if let Ok(mutex_guard) = mutex_guard2 {
      println!("The Mutex Guard has: {}!", mutex_guard)
    } else {
      println!("Didn't get a lock!")
    }
  }

  println!();

  // short version - no need to call drop(…) on a mutex changer
  {
    let my_mutex = Mutex::new(5);

    *my_mutex.lock().unwrap() = 6;

    println!("{:?}", my_mutex); // 6
  }

  println!();

  {
    let my_mutex = Mutex::new(0);

    // locks and unlocks 100 times
    for _ in 0..100 {
      // change in one go
      *my_mutex.lock().unwrap() += 1;
    }

    println!("{:?}", my_mutex); // 100
  }

  println!();

  // RwLock - a reader-writer lock
  {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap(); // one .read() is fine
    let read2 = my_rwlock.read().unwrap(); // two .read()s is also fine

    println!("{:?}, {:?}", read1, read2); // 5, 5

    // let write1 = my_rwlock.write().unwrap(); // ⚠️ uh oh, now the program will wait forever!
  }

  println!();

  // use std::mem::drop; // We will use drop() many times

  {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    println!("{:?}, {:?}", read1, read2); // 5, 5

    drop(read1);
    drop(read2); // we dropped both, so we can use .write() now

    let mut write1 = my_rwlock.write().unwrap();
    *write1 = 6;

    drop(write1);
    println!("{:?}", my_rwlock);
  }

  println!();

  // try_read(…), try_write(…)
  {
    let my_rwlock = RwLock::new(5);

    let read1 = my_rwlock.read().unwrap();
    let read2 = my_rwlock.read().unwrap();

    drop(read1);
    drop(read2);

    if let Ok(mut number) = my_rwlock.try_write() {
      *number += 10;
      println!("Now the number is {}.", number);
    } else {
      println!("Couldn't get write access, sorry!")
    };
  }
}
