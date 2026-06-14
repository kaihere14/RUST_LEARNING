fn main() {
    let string_answer = largest("thakur","arman");
    println!("Your string_ans is {}",string_answer);
    let number_answer = largest(10,5);
    println!("Your num_ans is {}",number_answer);

}

fn largest<T: std::cmp::PartialOrd>(a:T,b:T)->T{
    if a>b {
        a
    }
    else {
        b
    }
}
