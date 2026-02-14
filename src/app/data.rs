use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

use binrw::io::BufReader;

use crate::app::inventory;
use crate::app::{self, inventory::Inventory};

pub struct DataStorer {
    root: PathBuf,
    exports: PathBuf,
    logs: PathBuf,
    receipts: PathBuf,
}

impl DataStorer {
    const FOLDER_NAME: &str = ".stocksense";

    pub fn new() -> anyhow::Result<Self> {
        #[cfg(unix)]
        let app_data = std::env::var("HOME")?;
        #[cfg(windows)]
        let app_data = std::env::var("APP_DATA")?;

        let root = PathBuf::from(app_data).join(Self::FOLDER_NAME);
        let exports = root.join("exportes");
        let logs = root.join("logs");
        let receipts = root.join("facturas");

        fs::create_dir_all(&root)?;
        fs::create_dir_all(&exports)?;
        fs::create_dir_all(&logs)?;
        fs::create_dir_all(&receipts)?;

        Ok(Self {
            root,
            exports,
            logs,
            receipts,
        })
    }

    fn inventory_file(&self) -> PathBuf {
        self.root.join("inventario.bin")
    }

    pub fn save_inventory(&self, inv: &inventory::Inventory) -> anyhow::Result<()> {
        let file = File::create(self.inventory_file())?;
        let mut writer = BufWriter::new(&file);
        inv.save_to_stream(&mut writer)?;
        Ok(())
    }

    pub fn load_inventory(&self) -> anyhow::Result<Inventory> {
        let file = File::open(self.inventory_file())?;
        let mut reader = BufReader::new(file);
        Inventory::load_from_stream(&mut reader)
    }
}
