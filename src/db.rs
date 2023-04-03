use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use home;

pub struct DB {
    path: Option<PathBuf>,
}

impl DB {
    pub fn new() -> Self {
        Self { path: None }
    }

    fn prepare_db(&mut self) {
        if self.path.is_some() {
            return;
        }

        match home::home_dir() {
            None => {
                eprintln!("Cannot get home directory");
                std::process::exit(1);
            }
            Some(home) => {
                let home_path = Path::new(home.to_str().unwrap());
                let db_path = home_path.join(".password-manager");

                self.path = Some(db_path);
            }
        }
    }

    pub fn store_password(
        &mut self,
        password: &str,
        description: &str,
    ) -> Result<(), std::io::Error> {
        if self.path.is_none() {
            self.prepare_db();
        }

        let db_path = self.path.as_ref().unwrap();
        let was_created = db_path.exists() == false;

        let mut file = OpenOptions::new()
            .create_new(!db_path.exists())
            .write(true)
            .append(true)
            .open(db_path)
            .unwrap();

        if !was_created {
            file.write_all("\n".as_bytes()).unwrap();
        }

        file.write_all(format!("{}:{}", description, password).as_bytes())
            .unwrap();

        Ok(())
    }
}
