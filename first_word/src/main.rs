fn main() {
    let intro : String = String::from("Arman");
    let first_word = get_first_word(intro);
    print!("Your first word is : {}",first_word);
}

fn get_first_word(line:String)->String{
    let mut ans:String = String::from("");
    for i in line.chars(){
        if i==' '{
            break;
        }
        ans = ans+i.to_string().as_str();
    }
    return ans;
}
