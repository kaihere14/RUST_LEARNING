struct User{
  name:String,
  age: u32,
  email:String,
}

fn main(){
  let user = User{
    name:String::from("Arman Thakur"),
    age: 30,
    email: String::from("arman.thakur@example.com"),
  };
println!("Your name is {} and your age is {} and your email is {}",user.name, user.age, user.email);
}
