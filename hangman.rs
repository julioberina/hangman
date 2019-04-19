extern crate rand;
extern crate termion;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;
use termion::clear;
use termion::cursor;

fn main() {
    // Create a path to the desired file
    let path = Path::new("words.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut phrases = String::new();

    match file.read_to_string(&mut phrases) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => (),
    }

    let phrasebank: Vec<&str> = phrases.lines().collect(); // convert phrases string to vector
    let mut rng = rand::thread_rng(); // random number generator
    let phrase = phrasebank[rng.gen_range(0, phrasebank.len())];
    let mut lives = 5;

    print!("{}", clear::All);
    print!("{}", cursor::Goto(1, 1));
    println!("Welcome to Hangman!");
    println!("Guess the sentence");
    print!("\n");

    for c in phrase.chars() {
        print!("{} ", c);
    }

    print!("\n");
}
