fn main() {
    let mut counter = 0;
    let answer = loop {
        if counter > 5{
            break counter * 2;
        };
        println!("using loop : {counter}");
        counter += 1;
        
    };
    println!("Final answer! {answer}");

    let mut counter2 = 0;
    while counter2 != 5{
        println!("using while : {counter2}");
        counter2 += 1;
    }

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the number : {}",arr[index]);
        index += 1;
    }
 
    let arr2 = [15, 25, 35, 45, 55];

    for element in arr2{
        println!("the number using for loop : {}",element);
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
