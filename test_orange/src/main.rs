use std::usize;

fn main(){
    let heate = '😻';
    println!("{}",heate);
}

fn calculate_length(s:&mut String)->usize{
    s.push_str("string");
    s.len()
}