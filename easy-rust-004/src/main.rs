// Type inference
// https://dhghomon.github.io/easy_rust/Chapter_8.html
// https://www.youtube.com/watch?v=q1D2vpy3kEI&list=PLfllocyHVgsRwLkTAhG0E-2QxCf-ozBkk

fn main() {
  let _small_number1: u8 = 10;
  let _small_number2 = 10u8; // 10u8 = 10 of type u8
  let _small_number3 = 10_u8; // this is easier to read

  let _big_number = 100_000_000_i32; // 100 million is easy to read with _

  let number1 = 42________u8;
  let number2 = 1___7______8____9______i32;
  println!("{}, {}", number1, number2);

  let _my_float = 64.;

  let _my_float1 = 5.;
  let _my_float2: f64 = 5.0;
  let _my_float3: f32 = 8.5;

  let _third_float = _my_float1 + _my_float3;

  // let _fourth_float = _my_float2 + _my_float3; // ⚠️ cannot add `f32` to `f64`

  let _my_float: f64 = 5.0;
  let _my_other_float: f32 = 8.5;

  // let third_float = _my_float + _my_other_float; // ⚠️ but it found an f32. It can't add them.
  let my_float: f64 = 5.0;
  let my_other_float: f32 = 8.5;

  let _third_float = my_float + my_other_float as f64; // my_other_float as f64 = use my_other_float like an f64
  let my_float = 5.0; // Rust will choose f64
  let my_other_float = 8.5; // Here again it will choose f64

  let _third_float = my_float + my_other_float;
  let my_float: f32 = 5.0;
  let my_other_float = 8.5; // Usually Rust would choose f64,

  let _third_float = my_float + my_other_float; // but now it knows that you need to add it to an f32. So it chooses f32 for my_other_float too
}
