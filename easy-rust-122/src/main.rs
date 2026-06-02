// Arc
// https://dhghomon.github.io/easy_rust/Chapter_49.html
// https://www.youtube.com/watch?v=ev8EDHau0B4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=PGEFy4sfmaQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'.

// `Arc<T>` will implement `Send` and `Sync` as long as the `T` implements `Send` and `Sync`.

// Send trait: safe to send to another thread
// Sync trait: safe to share between threads

// NOTE: Arc implements Clone. Mutex doesn't. Take the best of both worlds!

#![allow(unused)]

use std::sync::{Arc, Mutex, MutexGuard};
use std::thread::{JoinHandle, spawn};

fn make_arc(n: i32) -> Arc<Mutex<i32>> {
  Arc::new(Mutex::new(n))
}

fn clone_arc(arc_mutex: &Arc<Mutex<i32>>) -> Arc<Mutex<i32>> {
  Arc::clone(&arc_mutex)
}

fn main() {
  {
    let handle = spawn(|| println!("The thread is working!"));

    handle.join().unwrap();
    println!("Exiting.");
  }

  println!();

  {
    let handle = spawn(|| {
      for _ in 0..5 {
        println!("The thread is working!")
      }
    });

    handle.join().unwrap();
    println!("Exiting.");
  }

  println!();

  {
    let thread1 = spawn(|| {
      for _ in 0..5 {
        println!("Thread 1 is working!")
      }
    });

    let thread2 = spawn(|| {
      for _ in 0..5 {
        println!("Thread 2 is working!")
      }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
    println!("Exiting.");
  }

  println!();

  // Mutex in Arc
  {
    let my_number: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    let my_number1: Arc<Mutex<i32>> = Arc::clone(&my_number);
    let my_number2: Arc<Mutex<i32>> = Arc::clone(&my_number);

    // only my_number1 goes into Thread 1
    let thread1 = spawn(move || {
      for _ in 0..500 {
        *my_number1.lock().unwrap() += 1;
      }
    });

    // only my_number2 goes into Thread 2
    let thread2 = spawn(move || {
      for _ in 0..500 {
        *my_number2.lock().unwrap() += 1;
      }
    });

    thread1.join();
    thread2.join();

    println!("{:?}", my_number); // 20
    println!("Exiting.");
  }

  println!();

  {
    let my_number: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut handle_vec: Vec<JoinHandle<()>> = vec![];

    for _ in 0..100 {
      let my_number_clone = Arc::clone(&my_number);
      let handle = spawn(move || {
        for _ in 0..10 {
          *my_number_clone.lock().unwrap() += 1;
        }
      });
      handle_vec.push(handle);
    }

    handle_vec.into_iter().for_each(|handle| {
      handle.join();
    });

    println!("{:?}", my_number);
    println!("Exiting.");
  }

  println!();

  // just the same with some function calls for creating arc's and clone's.
  {
    let my_number: Arc<Mutex<i32>> = make_arc(0);
    let mut handle_vec: Vec<JoinHandle<()>> = vec![];

    for _ in 0..100 {
      let my_number_clone: Arc<Mutex<i32>> = clone_arc(&my_number);
      let handle = spawn(move || {
        for _ in 0..10 {
          let mut value_inside: MutexGuard<'_, i32> = my_number_clone.lock().unwrap();
          *value_inside += 1;
        }
      });
      handle_vec.push(handle);
    }

    handle_vec.into_iter().for_each(|handle| {
      handle.join();
    });

    println!("{:?}", my_number);
    println!("Exiting.");
  }

  println!();
}
