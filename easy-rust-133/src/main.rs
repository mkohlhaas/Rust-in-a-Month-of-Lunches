// Channels
// https://dhghomon.github.io/easy_rust/Chapter_50.html
// https://www.youtube.com/watch?v=hOP_zvUfLk4&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=iW9XpVZ13GQ&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
// https://www.youtube.com/watch?v=JffJieRyB2o&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

// Sender is Send and Sync.
// Receiver is Send.

// mpsc = multiple producer, single consumer

use std::sync::mpsc::{Receiver, Sender, channel};
use std::thread::spawn;

fn main() {
  {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = channel();
    println!("{:?}, {:?}", sender, receiver);
  }

  {
    let (sender, receiver) = channel();

    let _ = sender.send(5);
    let _ = receiver.recv(); // NOTE: blocks!
  }

  {
    let (sender, receiver) = channel();

    let _ = sender.send(5);
    println!("{}", receiver.recv().unwrap()); // 5
  }

  // ⚠️ Panics from time to time because we are not using join handles.
  // Trying to receive when both threads didn't send yet -> recv(…) returns Err(…).
  {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    spawn(move || {
      let _ = sender.send("Send a &str this time.");
    });

    spawn(move || {
      let _ = sender_clone.send("And here is another &str.");
    });

    println!("{}", receiver.recv().unwrap());
  }

  println!();

  // with threads
  {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    let mut handles = vec![];

    handles.push(spawn(move || {
      let _ = sender.send("Send a &str this time.");
    }));

    handles.push(spawn(move || {
      let _ = sender_clone.send("And here is another &str.");
    }));

    for handle in handles {
      handle.join().unwrap();
      println!("{:?}", receiver.recv().unwrap());
    }
  }

  println!();

  {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();
    let mut handles = vec![];
    let mut results = vec![];

    handles.push(spawn(move || {
      sender.send("Send a &str this time.").unwrap();
    }));

    handles.push(spawn(move || {
      sender_clone.send("And here is another &str.").unwrap();
    }));

    for handle in handles {
      handle.join().unwrap();
      results.push(receiver.recv().unwrap());
    }

    println!("{:?}", results);
  }

  println!();

  {
    const NUM_ELEMENTS: usize = 1_000_000;
    const NUM_THREADS: usize = 100;

    let (sender, receiver) = channel();

    let hugevec = vec![0; NUM_ELEMENTS];

    let batch_size = hugevec.len() / NUM_THREADS;

    let mut handles = vec![];

    for i in 0..NUM_THREADS {
      let sender_clone = sender.clone();

      let mut batch: Vec<u8> = Vec::with_capacity(batch_size);
      let start = i * batch_size;
      let end = (i + 1) * batch_size;
      batch.extend(&hugevec[start..end]);

      let handle = spawn(move || {
        for n in &mut batch {
          *n += 1;
        }
        sender_clone.send(batch).unwrap();
      });
      handles.push(handle);
    }

    for handle in handles {
      let _ = handle.join();
    }

    let mut newvec = vec![];

    // try_recv does not block
    while let Ok(result) = receiver.try_recv() {
      newvec.push(result);
    }

    let newvec: Vec<u8> = newvec.into_iter().flatten().collect();

    println!("total length: {}", newvec.len());
    println!("All elements are 1: {}", newvec.iter().all(|n| n == &1));

    assert_eq!(newvec.len(), NUM_ELEMENTS);
    assert!(newvec.iter().all(|n| n == &1));
  }
}
