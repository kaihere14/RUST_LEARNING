use std::collections::HashMap;

fn group_values_by_keys(vec_data:Vec<(String,i32)>)->HashMap<String,i32>{
    let mut hm = HashMap::new();
    for (key,value)in vec_data{
        hm.insert(key,value);
    }
    hm
}

fn main(){
    let vec_data = vec![(String::from("arman"),22),(String::from("aditya"),24),(String::from("krish"),22)];
    println!("{:?}",group_values_by_keys(vec_data));
}