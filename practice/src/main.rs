fn main() {
    let str = String::from("Hello world");
    let word = get_first_word(str);
    println!("{}", word);
}

fn get_first_word(str: String) -> String {
    let mut word = String::new();
    for i in str.chars() {
        word.push(i);
        if i == ' ' {
            break;
        }
    }
    return word;
}
