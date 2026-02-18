use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use chrono::prelude::*;

use binrw::io::BufReader;

use crate::app::inventory::Inventory;
use crate::app::{inventory};

pub struct DataStorer {
    root: PathBuf,
    exports: PathBuf,
    // logs: PathBuf,
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
        // let logs = root.join("logs");
        let receipts = root.join("facturas");

        fs::create_dir_all(&root)?;
        fs::create_dir_all(&exports)?;
        // fs::create_dir_all(&logs)?;
        fs::create_dir_all(&receipts)?;

        Ok(Self {
            root,
            exports,
            // logs,
            receipts,
        })
    }

    fn inventory_file(&self) -> PathBuf {
        self.root.join("inventario.bin")
    }

    fn csv_file(&self) -> Result<File, io::Error> {
        const EXTENSION: &str = ".csv";

        let local_now = Local::now();
        let base = format!(
            "ESTADO_INVENTARIO_{}-{}-{}",
            local_now.year(),
            local_now.month(),
            local_now.day()
        );

        for i in 0.. {
            let f = format!(
                "{}{}{}",
                base,
                if i > 0 {
                    format!("_({})", i)
                } else {
                    String::new()
                },
                EXTENSION
            );

            let path = self.exports.join(&f);

            if let Ok(v) = fs::exists(&path)
                && !v
            {
                return File::create(&path);
            };
        }
        unreachable!()
    }

    fn sanitize_filename(source: &str) -> String {
        let mut out = String::new();
        for c in source.chars() {
            if c.is_ascii_alphanumeric() {
                out.push(c);
            } else {
                out.push('_');
            }
        }

        if out.is_empty() {
            String::from("FACTURA")
        } else {
            out
        }
    }

    fn bill_csv_file(&self, name: &str) -> Result<File, io::Error> {
        const EXTENSION: &str = ".csv";

        let local_now = Local::now();
        let safe_name = Self::sanitize_filename(name);
        let base = format!(
            "FACTURA_{}_{}-{}-{}_{}-{}-{}",
            safe_name,
            local_now.year(),
            local_now.month(),
            local_now.day(),
            local_now.hour(),
            local_now.minute(),
            local_now.second()
        );

        for i in 0.. {
            let f = format!(
                "{}{}{}",
                base,
                if i > 0 {
                    format!("_({})", i)
                } else {
                    String::new()
                },
                EXTENSION
            );

            let path = self.receipts.join(&f);

            if let Ok(v) = fs::exists(&path)
                && !v
            {
                return File::create(&path);
            };
        }
        unreachable!()
    }

    pub fn open_root_folder(&self) -> anyhow::Result<()> {
        opener::open_browser(&self.root)?;
        Ok(())
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

    pub fn export_inventory(&self, inv: &inventory::Inventory) -> anyhow::Result<()> {
        let file = self.csv_file()?;
        let mut writer = BufWriter::new(&file);
        inv.csv_to_stream(&mut writer)
    }

    pub fn export_bill_csv(&self, name: &str, data: &[u8]) -> anyhow::Result<()> {
        let file = self.bill_csv_file(name)?;
        let mut writer = BufWriter::new(&file);
        writer.write_all(data)?;
        writer.flush()?;
        Ok(())
    }
}
