use::std::fs;

fn main() {
  let res = fs::read_to_string("arman.txt");
  match res{
    Ok(res)=>{println!("File contents: {}", res);}
    Err(e)=>println!("error :  {}",e)
  }
}