use crate::tools::select;
use std::io;

mod todo; 
mod grades; 
mod tools;
mod program; 
mod command; 

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    select(input);
}
