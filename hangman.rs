extern crate rand;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;

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
    let mut words = String::new();

    match file.read_to_string(&mut words) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => (),
    }

    let wordbank: Vec<&str> = words.lines().collect(); // convert words string to vector
    let mut rng = rand::thread_rng(); // random number generator

    for c in wordbank[rng.gen_range(0, wordbank.len())].chars() {
        print!("{} ", c);
    }

    print!("\n");

    // println!("Contents of wordbank:\n");
    // for i in 0..wordbank.len() {
    //     println!("{}", wordbank[i]);
    // }
    //
    // for _ in 0..10 {
    //     println!("{}", rng.gen_range(0, wordbank.len()));
    // }
}
