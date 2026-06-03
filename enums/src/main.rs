#[derive(PartialEq)]
enum Direction{
  Up,
  Down,
  Left,
  Right,
}

fn main(){
  let direction:Direction =  Direction::Up;
  if direction == Direction::Up {
    println!("Going up");
  }else if direction == Direction::Down {
    println!("Going down");
} else if direction == Direction::Left {
    println!("Going left");
} else if direction == Direction::Right {
    println!("Going right");
  }
}