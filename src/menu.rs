use std::io::stdin;

use clearscreen;
use figlet_rs::FIGfont;

pub struct Menu {
    choices: &'static [&'static str],
}

impl Menu {
    pub fn new() -> Self {
        Self {
            choices: &["Generate password", "Obtain password", "Exit"],
        }
    }

    fn gen_banner() {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert("Password Manager").unwrap();
        println!("{}", figure);
    }

    pub fn get_option(&self) -> u8 {
        clearscreen::clear().unwrap();

        Self::gen_banner();

        println!("Select an option.");

        for (i, choice) in self.choices.into_iter().enumerate() {
            println!("{}. {}", i + 1, choice);
        }

        let mut input = String::new();
        let stdin = stdin();

        stdin
            .read_line(&mut input)
            .expect("Failed to read user input");

        if let Err(_) = input.trim().parse::<u8>() {
            eprintln!("Invalid number!");
            std::process::exit(1);
        }

        let input: u8 = input.trim().parse().unwrap();

        if input < 1 || input > self.choices.len() as u8 {
            println!("Option out of range!");
            std::process::exit(1);
        }

        return input;
    }
}
