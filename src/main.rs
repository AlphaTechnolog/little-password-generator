#[macro_use]
extern crate magic_crypt;

pub mod db;
pub mod menu;

pub mod password {
    pub mod generate;
    pub mod ui;
}

use crate::menu::Menu;
use crate::password::ui::PasswordGeneratorUI;

fn main() {
    let menu = Menu::new();
    let ui = PasswordGeneratorUI::new(menu.get_option());

    ui.decide_by_option();
}
