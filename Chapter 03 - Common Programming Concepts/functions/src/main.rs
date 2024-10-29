fn another_function(x: i32) {
  println!("the value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}

fn main() {
  another_function(five());
  print_labeled_measurement(5, 'h');

  // statement and expressions
  let y = {
    let x = 3;
    plus_one(x + five())
  };
  println!("The value of y is: {y}");
}
