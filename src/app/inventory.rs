use core::fmt;

use owo_colors::{OwoColorize, colors::*};
use rand::distr::{Alphanumeric, SampleString};

pub enum InventoryError {
    ProductNotFound,
    EmptyID,
    EmptyName,
    EmptyProvider,
    NegativePrice(f32),
}

impl fmt::Display for InventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProductNotFound => write!(f, "El producto no se encontró."),
            Self::EmptyID => write!(f, "El ID no puede estar vacío"),
            Self::EmptyName => write!(f, "El nombre no puede estar vacío"),
            Self::EmptyProvider => write!(f, "El proveedor no puede estar vacío"),
            Self::NegativePrice(price) => write!(f, "El precio ({}) no puede ser negativo", price),
        }
    }
}

pub struct InventoryItem {
    id: String,
    name: String,
    provider: String,
    quantity_left: u16,
    price: f32,
}

impl InventoryItem {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            provider: String::new(),
            quantity_left: 0,
            price: 0f32,
        }
    }

    pub fn get_id(&self) -> &str {
        self.id.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_provider(&self) -> &str {
        self.provider.as_str()
    }

    pub fn get_quantity_left(&self) -> u16 {
        self.quantity_left
    }

    pub fn get_price(&self) -> f32 {
        self.price
    }

    fn set_id(&mut self, id: &str) -> Result<(), InventoryError> {
        if id.is_empty() {
            Err(InventoryError::EmptyID)
        } else {
            self.id = String::from(id);
            Ok(())
        }
    }

    pub fn set_name(&mut self, name: &str) -> Result<(), InventoryError> {
        if name.is_empty() {
            Err(InventoryError::EmptyName)
        } else {
            self.name = String::from(name);
            Ok(())
        }
    }

    pub fn set_provider(&mut self, provider: &str) -> Result<(), InventoryError> {
        if provider.is_empty() {
            Err(InventoryError::EmptyProvider)
        } else {
            self.provider = String::from(provider);
            Ok(())
        }
    }

    pub fn set_quantity_left(&mut self, quantity: u16) {
        self.quantity_left = quantity;
    }

    pub fn set_price(&mut self, price: f32) -> Result<(), InventoryError> {
        if price < 0f32 {
            Err(InventoryError::NegativePrice(price))
        } else {
            self.price = price;
            Ok(())
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            name: self.name.clone(),
            provider: self.provider.clone(),
            quantity_left: self.quantity_left,
            price: self.price,
        }
    }

    pub fn new(
        id: &str,
        name: &str,
        provider: &str,
        quantity_left: u16,
        price: f32,
    ) -> Result<Self, InventoryError> {
        let mut new = Self::default();

        new.set_id(id)?;
        new.set_name(name)?;
        new.set_provider(provider)?;
        new.set_quantity_left(quantity_left);
        new.set_price(price)?;

        Ok(new)
    }
}

pub struct Inventory {
    items: Vec<InventoryItem>,
}

impl Inventory {
    const ID_LENGTH: usize = 8;

    pub fn new() -> Self {
        Inventory { items: Vec::new() }
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.size() == 0
    }

    fn generate_unique_id(&self) -> String {
        loop {
            let id = Alphanumeric.sample_string(&mut rand::rng(), Self::ID_LENGTH);

            if self.find(&id).is_none() {
                return id;
            }
        }
    }

    pub fn add(
        &mut self,
        name: &str,
        provider: &str,
        quantity_left: u16,
        price: f32,
    ) -> Result<(), InventoryError> {
        let item = InventoryItem::new(
            &self.generate_unique_id()[..],
            name,
            provider,
            quantity_left,
            price,
        )?;
        self.items.push(item);
        Ok(())
    }

    pub fn find(&self, id: &str) -> Option<&InventoryItem> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn find_mut(&mut self, id: &str) -> Option<&mut InventoryItem> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    pub fn remove(&mut self, id: &str) -> Result<(), InventoryError> {
        if let Some(index) = self.items.iter().position(|item| item.id == id) {
            self.items.remove(index);
            Ok(())
        } else {
            Err(InventoryError::ProductNotFound)
        }
    }

    pub fn edit(
        &mut self,
        id: &str,
        name: Option<&str>,
        provider: Option<&str>,
        quantity_left: Option<u16>,
        price: Option<f32>,
    ) -> Result<(), InventoryError> {
        if let Some(item) = self.find_mut(id) {
            let mut temp = item.clone();

            if let Some(name) = name {
                temp.set_name(name)?;
            }
            if let Some(provider) = provider {
                temp.set_provider(provider)?;
            }
            if let Some(quantity_left) = quantity_left {
                temp.set_quantity_left(quantity_left);
            }
            if let Some(price) = price {
                temp.set_price(price)?;
            }

            *item = temp;
            Ok(())
        } else {
            Err(InventoryError::ProductNotFound)
        }
    }

    pub fn print(&self) {
        for item in &self.items[..] {
            println!(
                "|{}|: {}, Proveedor: {}\nCantidad Disponible: {}, Precio (Unitario): {}",
                item.get_id().fg::<Yellow>(),
                item.get_name().fg::<Green>(),
                item.get_provider().fg::<Cyan>(),
                item.get_quantity_left().fg::<Cyan>(),
                item.get_price().fg::<Cyan>()
            )
        }
    }

    // TODO: Inventory Load from File (or stream) using Serialization
    // TODO: Inventory Save to File (or stream) using Serialization
    // TODO: Same thing but to CSV
}
