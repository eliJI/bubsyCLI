
use std::io::{self, BufReader, BufRead};
use std::fs::File;
fn main() {
    let quotes:Vec<String> = Vec::new();
    let f = File::open("C:\\Users\\elija\\Documents\\GitHub\\bubsyCLI\\src\\bubsy.html").expect("File failed to open");
    let mut reader = BufReader::new(f);
    let line = String::new();

    let mut lines = reader.lines();
    
    for l in lines {
       match l {
        Ok(s) => {println!("{}",s)},
        Err(e) => println!("ERROR")
       }
    }
    println!("done");
}
/**
 * Parses quotes from raw HTMl provided in resources
 */
fn parse(lines: Option<self>) {
    
}