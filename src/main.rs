mod todo; 
mod grades; 

pub struct Program {
    menu: String,
}

impl Program {
    pub fn choose_menu(&mut self, choice: String) -> () {
        self.menu = choice; 
    }

    pub fn show_menu(&mut self) -> () {
        println!("Current menu is {}", self.menu); 
    }
}

fn main() {
    println!("Hello, world!");
}
