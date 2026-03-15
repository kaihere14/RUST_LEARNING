use std::io;

fn main() {
    let  x = 5;
    println!("The value of x is: {}", x);
    let x = x + 5; // This will cause a compile-time error because x is immutable by default
    println!("The value of x is: {}", x);

    {
        let x = x+1;
        println!("The value of x is : {x}")
    }
    println!("The value of x is: {}", x);

    let spaces: &str = "  ";
    let spaces = spaces.len();
    println!("{spaces}");

    let a = [1,2,3,4,5];
    let mut index = String::new();
    println!("Enter your index : ");
    io::stdin()
        .read_line(&mut index)
        .expect("failed to edit ");

    let index:usize= index
        .trim()
        .parse()
        .expect("please enteer a valid input");

    let element = a[index];
    println!("Your element is : {element}");
    

}
