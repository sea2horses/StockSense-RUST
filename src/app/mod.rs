use inventory::Inventory;

mod input;
mod inventory;
mod menus;
mod titles;

pub struct App {
    inventory: Inventory,
    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            inventory: Inventory::new(),
            should_exit: false,
        }
    }

    pub fn run(&mut self) {
        menus::main_menu(self);
    }
}
