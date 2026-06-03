
enum Direction{
  Up,
  Down,
  Left,
  Right,
}

enum Shapes{
  Circle(u32),
}

fn main(){
  let direction:Direction =  Direction::Right;
  match direction{
    Direction::Up=>println!("You are going up "),
    Direction::Down=>println!("You are going down "),
    Direction::Left=>println!("You are going left "),
    Direction::Right=>println!("You are going right "),
  };

  let shape = Shapes::Circle(10);
  println!("Your area is {}",calculate_area(shape));
}

fn calculate_area(shape:Shapes)->f32{
  //pattern matching enums 
  match shape{
    Shapes::Circle(radius)=>return (3*radius*radius) as f32
  };
}