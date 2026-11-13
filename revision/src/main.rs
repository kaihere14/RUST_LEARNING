fn main() {
    println!("Hello World!");

    let x: i32 = 1;

    // println!("Your defined number is {}", z); will not able to run because z is not in the current scope

    {
        let y: i32 = 3;
        println!(
            "Your x is {} as there is no x in inner world it took it form outer world / main world and y is {}",
            x, y
        );

        let x: i32 = 2;
        println!(
            "Your x is {} as the inner scope is complete new world so we first check if x is there in inner and y is {}",
            x, y
        );
    }

    println!(
        "Your x is  {} as the inner world is sep and outer world x is seperate ",
        x
    );

    let x: i32 = 2;
    println!(
        "Your x is now {} as x is change in the outer world / main world ",
        x
    );

    define_z();

    // println!("Your x is {} and y is {}", x, y); will not able to run because y lifetime got expired as its scope got closed
}

fn define_z() {
    let z: i32 = 5;
    println!("Your defined number is {}", z);
}
