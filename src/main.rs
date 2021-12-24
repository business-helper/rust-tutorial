fn main() {
  let x: i32 = 5;
  println!("The value of x is: {}", x);
  let x: &str = "six";
  println!("The value of x is: {}", x);

  const SUBSCRIBER_COUNT: u32 = 100_000;

  let smile_face: char = '😀';

  let tup: (&str, i32) = ("Let's Get Rusty!😀", 100_000);

  let (channel, sub_count) = tup;

  let error_codes = [200, 404, 500];
  let not_found: i32 = error_codes[1];
  
  let byte = [0; 8];
}
