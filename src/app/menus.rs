use crate::app::App;
use crate::app::input;
use crate::app::inventory::InventoryItem;
use crate::app::titles;
use defer_rs::defer;
use owo_colors::{OwoColorize, colors::*};

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
        (String::from("Inventario"), inventory_menu),
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

pub fn inventory_menu(app: &mut App) {
    let inventory_menu = Menu::new(vec![
        (String::from("Agregar al inventario"), inventory_add),
        (String::from("Remover del inventario"), inventory_remove),
        (String::from("Editar inventario"), inventory_edit),
        (String::from("Mostrar inventario"), inventory_show),
        (String::from("Salir"), |app: &mut App| {
            app.should_exit = true
        }),
    ]);

    while !app.should_exit {
        clearscreen::clear().ok();
        println!("{}", titles::inventory_title().fg::<Green>());
        inventory_menu.print();

        let selected: usize = input::read("> ", "Ingrese un número válido.");
        if inventory_menu.exec(app, selected).is_none() {
            println!("{}", "Opcion Inválida".fg::<Red>())
        }

        if let Some(s) = &app.storer
            && let Err(e) = s.save_inventory(&app.inventory)
        {
            println!(
                "{} {}",
                "No se pudo guardar el inventario al disco: ".fg::<Cyan>(),
                e.fg::<Red>()
            );
            input::halt_until_enter();
        }
    }
    app.should_exit = false;
}

pub fn inventory_add(app: &mut App) {
    clearscreen::clear().ok();
    println!("{}", titles::inventory_title().fg::<Green>());

    defer! {
        input::halt_until_enter();
    };

    println!("{}", "+ Nombre del Producto: ".fg::<Cyan>());
    let name = input::read_string("> ");

    println!("\n{}", "+ Proveedor: ".fg::<Cyan>());
    let provider = input::read_string("> ");

    println!("\n{}", "+ Cantidad en Stock: ".fg::<Cyan>());
    let quantity_left: u16 = input::read(
        "> ",
        "Ingrese un numero entero válido. (Debe ser mayor o igual a 0)",
    );

    println!("\n{}", "+ Precio: ".fg::<Cyan>());
    let price: f32 = input::read("> ", "Ingrese un numero válido.");

    let result = app
        .inventory
        .add(name.as_str(), provider.as_str(), quantity_left, price);

    match result {
        Ok(_) => println!("{}", "Producto añadido exitosamente.".fg::<Green>()),
        Err(e) => println!("Error al añadir producto: {}", e.fg::<Red>()),
    }
}

pub fn inventory_remove(app: &mut App) {
    clearscreen::clear().ok();
    println!("{}", titles::inventory_title().fg::<Green>());

    defer! {
        input::halt_until_enter();
    };

    if app.inventory.is_empty() {
        println!("{}", "No hay productos en el inventario".fg::<Red>());
        return;
    }

    app.inventory.print();
    println!("{}", "+ ID del Producto: ".fg::<Cyan>());
    let id = input::read_string("> ");

    let removed = app.inventory.remove(id.as_str());
    match removed {
        Ok(_) => println!("{}", "El producto se eliminó exitosamente".fg::<Green>()),
        Err(e) => println!("Error al remover producto: {}", e.fg::<Red>()),
    }
}

pub fn inventory_edit(app: &mut App) {
    clearscreen::clear().ok();
    println!("{}", titles::inventory_title().fg::<Green>());

    defer! {
        input::halt_until_enter();
    };

    if app.inventory.is_empty() {
        println!("{}", "No hay productos en el inventario".fg::<Red>());
        return;
    }

    app.inventory.print();

    println!("{}", "+ ID del Producto: ".fg::<Cyan>());
    let id = input::read_string("> ");

    let temp: &InventoryItem = if let Some(item) = app.inventory.find(id.as_str()) {
        item
    } else {
        println!("{}", "No se encontró el producto".fg::<Red>());
        return;
    };

    println!(
        "{}\n",
        "En el caso de no querer editar un campo, dejelo vacío.".fg::<Cyan>()
    );

    println!("\n+ + Nombre Anterior: {}", temp.get_name().fg::<Green>());
    println!("\n{}", "+ Nombre del Producto: ".fg::<Cyan>());
    let name = input::read_optional_string("> ");

    println!(
        "\n+ + Proveedor Anterior: {}",
        temp.get_provider().fg::<Green>()
    );
    println!("\n{}", "+ Proveedor: ".fg::<Cyan>());
    let provider = input::read_optional_string("> ");

    println!(
        "\n+ + Cantidad Anterior: {}",
        temp.get_quantity_left().fg::<Green>()
    );
    println!("\n{}", "+ Cantidad en Stock: ".fg::<Cyan>());
    let quantity_left: Option<u16> = input::read_optional(
        "> ",
        "Ingrese un numero entero válido. (Debe ser mayor o igual a 0)",
    );

    println!("\n+ + Precio Anterior: {}", temp.get_price().fg::<Green>());
    println!("\n{}", "+ Precio: ".fg::<Cyan>());
    let price: Option<f32> = input::read_optional("> ", "Ingrese un numero válido.");

    match app.inventory.edit(
        id.as_str(),
        name.as_deref(),
        provider.as_deref(),
        quantity_left,
        price,
    ) {
        Ok(_) => println!("{}", "El producto se editó exitosamente".fg::<Green>()),
        Err(e) => println!("Error al editar producto: {}", e.fg::<Red>()),
    }
}

pub fn inventory_show(app: &mut App) {
    clearscreen::clear().ok();
    println!("{}\n", titles::inventory_title().fg::<Green>());

    defer! {
        input::halt_until_enter();
    };

    app.inventory.print();
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
