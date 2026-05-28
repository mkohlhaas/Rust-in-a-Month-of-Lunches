// Uninitialized Variables
// (No chapter in the online book.)
// https://www.youtube.com/watch?v=xfNfu40FQE8&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk
fn main() {
  {
    // let x: u8; // rustc: binding declared here but left uninitialized [E0381]
    // println!("{}", x); // rustc: `x` used here but it isn't initialized [E0381]
  }

  {
    let x;
    let y = 20;
    if y < 20 {
      x = 20;
      println!("{}", x);
    }
    // println!("{}", x); // rustc: used binding `x` is possibly-uninitialized
  }
}
