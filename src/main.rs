
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::thread;
use std::time::Duration;
use std::path::Path;
use rand::Rng;
fn main() {
    let logo = process(String::from("logo.txt"));
    let quotes = process(String::from("bubsy.txt"));

    display(logo);
    randomQuote(&quotes);
    randomQuote(&quotes);

    thread::sleep(Duration::from_millis(5000));
    println!("done");
}
/**
 * Exstrats lines from a text file, returning a vector of lines
 */
fn process(pth: String) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let f = File::open(pth).expect("Error Occured");
    let reader = BufReader::new(f);
    let output = reader.lines();

    for l in output {
        match l {
            Ok(s) => lines.push(s),
            Err(e) => println!("{e}")
        }
    }
    lines
}
/**
 * generical method for display lines of a vector of strings
 */
fn display(lines: Vec<String>) {
    for line in lines {
        println!("{}",line);
    }
}
/**
 * Generates a random quote given a reference to a vector of strings
 */
fn randomQuote(quotes: &Vec<String>) {
    let mut rng = rand::thread_rng();
    let num = rng.gen_range(0..quotes.len());
    println!("{}",quotes[num]);
}


