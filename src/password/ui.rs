use crate::db::DB;
use crate::password::generate::PasswordGenerator;

use clearscreen;
use figlet_rs::FIGfont;

pub struct PasswordGeneratorUI {
    generator: Option<PasswordGenerator<'static>>,
    option: u8,
}

impl PasswordGeneratorUI {
    pub fn new(option: u8) -> Self {
        Self {
            option,
            generator: None,
        }
    }

    fn banner_by_title(title: &str) {
        let standard_font = FIGfont::standard().unwrap();
        let figure = standard_font.convert(title).unwrap();
        println!("{}", figure);
    }

    fn generate_password(&self) {
        Self::banner_by_title("Generate Password");

        if self.generator.is_some() {
            eprintln!("Password already generated!");
            std::process::exit(1);
        }

        let mut input = String::new();
        let stdin = std::io::stdin();

        println!("Enter a semi-complicated password-phrase to generate a secure password: ");

        stdin
            .read_line(&mut input)
            .expect("Failed to read user input");

        let generator = PasswordGenerator::new(input.trim());
        let passwd = generator.generate();

        println!("Generated password: {}", &passwd);

        let mut description = String::new();

        println!("Write a description for that password to store it in the database: ");

        stdin
            .read_line(&mut description)
            .expect("Failed to read user input");

        DB::new()
            .store_password(&passwd, description.trim())
            .expect("Cannot store the password in the database!");

        println!("Password stored in the database!");
    }

    fn obtain_password(&self) {
        self.generate_password();
    }

    pub fn decide_by_option(&self) {
        clearscreen::clear().expect("Cannot clear terminal!");

        match self.option {
            1 => self.generate_password(),
            2 => self.obtain_password(),
            _ => std::process::exit(0),
        }
    }
}
