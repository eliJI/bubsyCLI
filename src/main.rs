
use std::io::{self, BufReader, BufRead};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::path::Path;
use std::result;
fn main() {
    displayTitle();
    
    thread::sleep(Duration::from_millis(5000));
    println!("done");
}

fn displayTitle() {
    let f = File::open("logo.txt").expect("File failed to open");
    let mut reader = BufReader::new(f);
    let line = String::new();
    let lines = reader.lines();
    
    for l in lines {
       match l {
        Ok(s) => println!("{}",s),
        Err(e) => println!("ERROR")
       }
    }
    
}

fn displayQuote() {
    let f = File::open("bubsy.txt").expect("File failed to open");
    let mut reader = BufReader::new(f);
    let lines = reader.lines();
    let mut collected_lines:Vec<String> = Vec::new();
    for l in lines {
        match l {
            Ok(s) => collected_lines.push(s),
            Err(e) => println!("Error Occured: {e}")
        }
    }

}
  

