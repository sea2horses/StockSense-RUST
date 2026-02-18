use std::{collections::HashMap, io::Write};

use anyhow::Result;
use serde::Serialize;

use crate::app::inventory::{Inventory, InventoryError, InventoryItem};

#[derive(Clone)]
struct BillDetail {
    quantity: usize,
}

impl BillDetail {
    fn new() -> Self {
        Self { quantity: 0 }
    }

    fn get_quantity(&self) -> usize {
        self.quantity
    }

    fn set_quantity(&mut self, quantity: usize) {
        self.quantity = quantity;
    }
}

pub struct Bill<'a> {
    inventory: &'a Inventory,
    items: HashMap<String, BillDetail>,
    name: String,
}

#[derive(Serialize)]
struct BillCsvRow {
    id: String,
    name: String,
    quantity: usize,
    price: f32,
    subtotal: f32,
}

impl<'a> Bill<'a> {
    pub fn new(name: &str, inventory: &'a Inventory) -> Self {
        Bill {
            name: String::from(name),
            inventory,
            items: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn compatible_detail(item: &InventoryItem, detail: &BillDetail) -> Result<(), InventoryError> {
        if detail.get_quantity() > item.get_quantity_left() {
            return Err(InventoryError::NotEnoughStock(detail.get_quantity()));
        }
        Ok(())
    }

    fn update_detail<F>(&mut self, id: &str, update: F) -> Result<(), InventoryError>
    where
        F: FnOnce(&BillDetail) -> BillDetail,
    {
        let item: &InventoryItem = match self.inventory.find(id) {
            Some(i) => i,
            None => {
                return Err(InventoryError::ProductNotFound);
            }
        };

        let detail = match self.items.get(id) {
            Some(detail) => detail.clone(),
            None => BillDetail::new(),
        };

        let updated = update(&detail);
        Self::compatible_detail(item, &updated)?;
        self.items.insert(String::from(id), updated);
        Ok(())
    }

    pub fn add_quantity(&mut self, id: &str, quantity: usize) -> Result<(), InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::InvalidStock(quantity));
        }

        self.update_detail(id, |detail: &BillDetail| {
            let mut detail = detail.clone();
            detail.set_quantity(detail.get_quantity() + quantity);
            detail
        })
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn iter_items(&self) -> impl Iterator<Item = (&str, usize)> + '_ {
        self.items
            .iter()
            .map(|(id, detail)| (id.as_str(), detail.get_quantity()))
    }

    pub fn total_price(&self) -> f32 {
        self.items
            .iter()
            .filter_map(|(id, detail)| {
                self.inventory
                    .find(id.as_str())
                    .map(|item| item.get_price() * detail.get_quantity() as f32)
            })
            .sum()
    }

    pub fn quantities_map(&self) -> HashMap<String, usize> {
        self.items
            .iter()
            .map(|(id, detail)| (id.clone(), detail.get_quantity()))
            .collect()
    }

    pub fn csv_to_stream(&self, s: &mut impl Write) -> Result<()> {
        let mut w = csv::Writer::from_writer(s);

        for (id, detail) in &self.items {
            let quantity = detail.get_quantity();
            if let Some(item) = self.inventory.find(id.as_str()) {
                let price = item.get_price();
                let subtotal = price * quantity as f32;
                w.serialize(BillCsvRow {
                    id: item.get_id().to_string(),
                    name: item.get_name().to_string(),
                    quantity,
                    price,
                    subtotal,
                })?;
            } else {
                w.serialize(BillCsvRow {
                    id: id.clone(),
                    name: String::from("(Desconocido)"),
                    quantity,
                    price: 0.0,
                    subtotal: 0.0,
                })?;
            }
        }

        w.serialize(BillCsvRow {
            id: String::from("TOTAL"),
            name: self.name.clone(),
            quantity: 0,
            price: 0.0,
            subtotal: self.total_price(),
        })?;

        w.flush()?;
        Ok(())
    }

    pub fn csv_to_vec(&self) -> Result<Vec<u8>> {
        let mut buf: Vec<u8> = Vec::new();
        self.csv_to_stream(&mut buf)?;
        Ok(buf)
    }
}
