use owo_colors::{OwoColorize, colors::*};

use crate::app::{App, input};

mod billing;
mod inventory;
mod titles;

type MenuOption = (String, fn(&mut App));

pub struct Menu {
    options: Vec<MenuOption>,
}

impl Menu {
    pub fn new(options: Vec<MenuOption>) -> Self {
        Menu { options }
    }

    pub fn exec(&self, app: &mut App, index: usize) -> Option<()> {
        if let Some(pair) = self.options.get(index - 1) {
            let (_, func) = pair;
            func(app);
            return Some(());
        }
        None
    }

    pub fn print(&self) {
        for (i, pair) in self.options.iter().enumerate() {
            let (action, _) = pair;
            println!("| {}. {}", i + 1, action.fg::<Yellow>())
        }
    }
}

pub fn main_menu(app: &mut App) {
    let main_menu = Menu::new(vec![
        (String::from("Inventario"), inventory::inventory_menu),
        (String::from("Facturacion"), billing::billing_menu),
        (String::from("Abrir Carpeta de Datos"), open_data_folder),
        (String::from("Salir"), |app: &mut App| {
            app.should_exit = true
        }),
    ]);

    while !app.should_exit {
        clearscreen::clear().ok();
        // Print main title
        println!("{}", titles::main_title().fg::<Green>());
        println!(
            "{} {}\n",
            "Port a Rust hecho por".fg::<BrightRed>(),
            "@sea2horses".fg::<Cyan>()
        );
        main_menu.print();

        let selected: usize = input::read("> ", "Ingrese un número válido.");
        if main_menu.exec(app, selected).is_none() {
            println!("{}", "Opcion Inválida".fg::<Red>())
        }
    }
    app.should_exit = false;
}

pub fn open_data_folder(app: &mut App) {
    if let Some(s) = &app.storer
        && let Err(e) = s.open_root_folder()
    {
        println!(
            "{} {}",
            "Hubo un error abriendo la carpeta de datos: ".fg::<Cyan>(),
            e.fg::<Red>()
        );
        input::halt_until_enter();
    }
}
