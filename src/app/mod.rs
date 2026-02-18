use data::DataStorer;
use inventory::Inventory;
use owo_colors::{OwoColorize, colors::*};

mod billing;
mod data;
mod input;
mod inventory;
mod menus;

pub struct App {
    inventory: Inventory,
    storer: Option<DataStorer>, // If data storer fails to construct, app will enter Non-Persistence Mode.
    should_exit: bool,
}

impl App {
    pub fn new() -> Self {
        let storer: Option<DataStorer> = match DataStorer::new() {
            Ok(s) => Some(s),
            Err(e) => {
                println!("{} {}", "Se falló en inicializar los archivos de guardado. La aplicación entrará en modo no persistente.".fg::<Cyan>(), "Todos los cambios hechos NO serán guardados en el disco.".fg::<Red>());
                println!("Información del error: {}", e.fg::<Red>());
                input::halt_until_enter();
                None
            }
        };

        let mut inventory = Inventory::new();
        if let Some(s) = &storer
            && let Ok(i) = s.load_inventory()
        {
            inventory = i;
        }

        Self {
            inventory,
            storer,
            should_exit: false,
        }
    }

    pub fn run(&mut self) {
        menus::main_menu(self);
    }
}
