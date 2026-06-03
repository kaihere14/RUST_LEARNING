struct Dim{
  length:u32,
  width:u32,
}

impl Dim{
  fn area(&self)->u32{
    self.length * self.width
  }
}

fn main(){
  let square = Dim{
    length: 10,
    width: 10,
  };
  let rectangle = Dim{ 
  length: 10, 
  width: 20 
};
  println!("Your are is of square is : {:?}",square.area());
  println!("Your are is of rectangle is : {:?}",rectangle.area());
  
}