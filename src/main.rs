
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::path::Path;
fn main() {
    let quotes:Vec<String> = Vec::new();
    //let path = Path::new("\\logo.txt");
    let f = File::open("logo.txt").expect("File failed to open");
    let mut reader = BufReader::new(f);
    let line = String::new();
    let lines = reader.lines();


    
    for l in lines {
       match l {
        Ok(s) => {println!("{}",s)},
        Err(e) => println!("ERROR")
       }
    }
    thread::sleep(Duration::from_millis(5000));
    println!("done");
}

