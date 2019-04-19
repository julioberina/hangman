extern crate rand;
extern crate termion;
#[macro_use] extern crate text_io;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashSet;
use rand::Rng;
use termion::clear;
use termion::cursor;

fn print_hangman(lives: i32) {
    match lives {
        0 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|           (X^X)");
            println!("|             |");
            println!("|            /|\\");
            println!("|             |");
            println!("|            / \\");
            for _ in 0..2 { println!("|"); }
        },
        1 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|             O");
            println!("|             |");
            println!("|            /|\\");
            println!("|             |");
            println!("|            / \\");
            for _ in 0..2 { println!("|"); }
        },
        2 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|             O");
            println!("|             |");
            println!("|            /|\\");
            println!("|             |");
            for _ in 0..3 { println!("|"); }
        },
        3 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|             O");
            println!("|             |");
            println!("|            / \\");
            for _ in 0..4 { println!("|"); }
        },
        4 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|             O");
            println!("|             |");
            for _ in 0..5 { println!("|"); }
        },
        5 => {
            println!(" _ _ _ _ _ _ _");
            println!("|             |");
            println!("|             O");
            for _ in 0..6 { println!("|"); }
        },
        _ => {
            println!(" _ _ _ _ _ _ _");
            for _ in 0..8 { println!("|"); }
        }
    }
}

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
    let mut lives: i32 = 6;
    let mut phrase_letters: HashSet<char> = HashSet::new();
    let mut guessed_letters: HashSet<char> = HashSet::new();
    let mut input: String;
    let mut result: String = String::from("Guess the phrase"); // Feedback of user guess

    // insert phrase chars in phrase_letters
    for c in phrase.chars() {
        if c.is_alphabetic() {
            phrase_letters.insert(c);
        }
    }

    // Clear screen
    print!("{}", clear::All);
    print!("{}", cursor::Goto(1, 1));
    println!("Welcome to Hangman!");
    print!("\n");

    while !phrase_letters.is_subset(&guessed_letters) && lives > 0 {
        println!("{}", result);
        print!("Lives: {}   Guessed letters: ", lives);

        for letter in guessed_letters.iter() { print!("{}", letter); }
        print!("\n\n");

        print_hangman(lives);
        print!("\n");

        for character in phrase.chars() {
            if guessed_letters.contains(&character) { print!("{} ", character); }
            else if character.is_whitespace() { print!("  "); }
            else { print!("_ "); }
        }
        print!("\n\n");

        println!("Enter guess:  ");
        input = read!("{}\n");

        // Check the first letter only of a read line
        match input.chars().next() {
            Some(first_char) => {
                let mut guess = first_char.to_ascii_lowercase();

                if !guess.is_alphabetic() { result = String::from("Not a letter!"); }
                else if guessed_letters.contains(&guess) {
                    result = String::from("You already guessed that letter!");
                }
                else {
                    guessed_letters.insert(guess);
                    if phrase_letters.contains(&guess) { result = String::from("Good guess!"); }
                    else {
                        result = String::from("Sorry! Bad guess!");
                        lives = lives - 1;
                    }
                }
            },
            None => result = String::from("Invalid input!")
        }

        // Clear screen
        print!("{}", clear::All);
        print!("{}", cursor::Goto(1, 1));
    }

    print_hangman(lives);
    print!("\n");

    for character in phrase.chars() {
        print!("{} ", character);
    }

    print!("\n\n");

    if lives == 0 { println!("Sorry!  You lose!"); }
    else { println!("Congratulations!  You guessed the phrase!"); }
}
