fn main() {
  date_type1();

  let x = 11;
  let y = 22;
  let sum = my_function(11, 22);
  println!("{} + {} = {}", x, y, sum);
}

fn date_type1() {
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

fn my_function(x: i32, y: i32) -> i32 {
  println!("[my_function]");
  let sum = x + y;
  return sum;
}
