fn main(){
    let st:&'static str  = "arman";
    let length:usize   = st.len();
    println!("hello from rust your number is : {length} and string is {st}");

    if length <= 5 {
        for i in 0..length{
            println!("your current number is {i} and char at that index is {st}")
        }
        println!("The length is 5 or under")
    }else {
        println!("Its not under 5")
    }
    
}