trait Summary{
    fn print_info(&self)->String;
}

struct User{
    name:String,
    roll:i32,
}

impl Summary for User{
    fn print_info(&self)->String {
        return format!("The name is : {} and roll is : {} ",self.name,self.roll);
    }
}
fn main(){
    let user = User{
        name:String::from("arman"),
        roll : 10,
    };
    println!("{}",user.print_info());

}