use std::io::{Result, stdin};

mod day_1;
mod utils;

fn main() -> Result<()> {
  let mut user_input = String::new();
  stdin().read_line(&mut user_input)?;  

  let result = match user_input.trim() {
    "1-1" => day_1::task_1::calculate(utils::get_input("input/day_1/task_1")),
    _ => "Unexpected input".to_owned()
  };

  println!("{}", result);

  Ok(())
}

