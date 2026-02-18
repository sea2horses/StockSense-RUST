use std::collections::HashMap;

use defer_rs::defer;
use owo_colors::{OwoColorize, colors::*};

use crate::app::{
    App,
    billing::Bill,
    input,
    inventory::{Inventory, InventoryError},
    menus::{Menu, titles},
};

pub fn billing_menu(app: &mut App) {
    let billing_menu = Menu::new(vec![
        (String::from("Facturar"), billing_make),
        (String::from("Salir"), |app: &mut App| {
            app.should_exit = true;
        }),
    ]);

    while !app.should_exit {
        clearscreen::clear().ok();
        println!("{}", titles::billing_title().fg::<Green>());
        billing_menu.print();

        let selected: usize = input::read("> ", "Ingrese un número válido.");
        if billing_menu.exec(app, selected).is_none() {
            println!("{}", "Opcion Inválida".fg::<Red>())
        }
    }
    app.should_exit = false;
}

fn billing_make(app: &mut App) {
    clearscreen::clear().ok();
    println!("{}", titles::billing_title().fg::<Green>());

    defer! {
        input::halt_until_enter();
    };

    if app.inventory.is_empty() {
        println!("{}", "No hay productos en el inventario".fg::<Red>());
        return;
    }

    println!("\n{}", "+ Ingrese el nombre de la factura".fg::<Cyan>());
    let name = input::read_string("> ");
    let mut bill = Bill::new(name.as_str(), &app.inventory);

    loop {
        app.inventory.print();
        println!("\n{}", "+ Elija un Producto para Facturar:".fg::<Cyan>());
        println!("{}", "(ENTER para terminar la factura)".fg::<BrightBlack>());
        let id = input::read_string("> ");

        if id.is_empty() {
            break;
        }

        println!(
            "\n{} {}:",
            "+ Elija una Cantidad del producto para Facturar".fg::<Cyan>(),
            "(Si el producto ya esta facturado se añadirá la cantidad.)"
        );
        let quantity: usize = input::read(
            "> ",
            "Ingrese un numero entero válido. (Debe ser mayor a 0)",
        );

        match bill.add_quantity(id.as_str(), quantity) {
            Ok(_) => println!("{}", "Producto agregado a la factura.".fg::<Green>()),
            Err(e) => println!("{} {}", "Error al facturar:".fg::<Cyan>(), e.fg::<Red>()),
        }
        println!();
    }

    if bill.is_empty() {
        println!("{}", "No se agregaron productos a la factura.".fg::<Red>());
        return;
    }

    println!("\n{}", "Resumen de la factura".fg::<Cyan>());
    for (id, quantity) in bill.iter_items() {
        if let Some(item) = app.inventory.find(id) {
            let subtotal = item.get_price() * quantity as f32;
            println!(
                "- |{}| {} x{} = ${:.2}",
                item.get_id().fg::<Yellow>(),
                item.get_name().fg::<Green>(),
                quantity,
                subtotal
            );
        } else {
            println!("- |{}| x{}", id.fg::<Yellow>(), quantity);
        }
    }
    println!("{} ${:.2}", "Total:".fg::<Cyan>(), bill.total_price());

    println!("\n{}", "Confirmar factura? (s/n)".fg::<Cyan>());
    let confirm = input::read_string("> ");
    let confirm = confirm.trim().to_lowercase();

    if confirm == "s" || confirm == "si" {
        let quantities = bill.quantities_map();
        let bill_name = bill.get_name().to_string();
        let bill_csv = match bill.csv_to_vec() {
            Ok(data) => Some(data),
            Err(e) => {
                println!(
                    "{} {}",
                    "No se pudo generar la factura en CSV: ".fg::<Cyan>(),
                    e.fg::<Red>()
                );
                None
            }
        };
        drop(bill);
        match apply_bill_quantities(&mut app.inventory, &quantities) {
            Ok(_) => {
                println!("{}", "Factura confirmada.".fg::<Green>());
                if let Some(s) = &app.storer {
                    if let Err(e) = s.save_inventory(&app.inventory) {
                        println!(
                            "{} {}",
                            "No se pudo guardar el inventario al disco: ".fg::<Cyan>(),
                            e.fg::<Red>()
                        );
                    }

                    if let Some(data) = bill_csv
                        && let Err(e) = s.export_bill_csv(bill_name.as_str(), &data)
                    {
                        println!(
                            "{} {}",
                            "No se pudo exportar la factura: ".fg::<Cyan>(),
                            e.fg::<Red>()
                        );
                    }
                }
            }
            Err(e) => println!(
                "{} {}",
                "Error al confirmar la factura:".fg::<Cyan>(),
                e.fg::<Red>()
            ),
        }
    } else {
        println!("{}", "Factura cancelada.".fg::<Red>());
    }
}

fn apply_bill_quantities(
    inventory: &mut Inventory,
    quantities: &HashMap<String, usize>,
) -> Result<(), InventoryError> {
    for (id, quantity) in quantities {
        let item = match inventory.find_mut(id.as_str()) {
            Some(item) => item,
            None => return Err(InventoryError::ProductNotFound),
        };

        if *quantity > item.get_quantity_left() {
            return Err(InventoryError::NotEnoughStock(*quantity));
        }

        item.set_quantity_left(item.get_quantity_left() - *quantity);
    }

    Ok(())
}
