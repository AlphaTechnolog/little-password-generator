use magic_crypt::MagicCryptTrait;

pub struct PasswordGenerator<'a> {
    pub phrase: &'a str,
}

impl<'a> PasswordGenerator<'a> {
    pub fn new(phrase: &'a str) -> Self {
        Self { phrase }
    }

    pub fn generate(&self) -> String {
        let mcrypt = new_magic_crypt!("SECRET", 256);
        mcrypt.encrypt_str_to_base64(self.phrase)
    }
}
