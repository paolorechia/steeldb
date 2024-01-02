//! In memory data representations.
use crate::database::config::DATA_DIR;
use crate::database::datatypes::DataType;
use crate::database::file_io::{ColumnarReader, ColumnarWriter, FileFormat, Reader, Writer};
use log::info;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::path::Path;

/// This defines a way to keep the data in-memory by the SteelDB.
/// It also represents the Table that the user receives back when querying the database.
/// This is currently in a columnar format.
/// Most of the exposed functionality here is a low level API meant to be used during the
/// database development. It is not meant to be used directly by database users.
#[derive(Debug)]
pub struct Table {
    /// The table name, this is used as an identifier for retrieving the correct table.
    pub name: String,
    /// The table fields or schema.
    pub fields: HashMap<String, DataType>,
    /// The actual data stored in columnar format.
    pub columns: HashMap<String, Vec<DataType>>,
    /// Used when retrieving data, allowing for projection push-down on query.
    /// That is, the Database do not read columns that were not specified in the query.  
    pub select_columns: Vec<String>,
}

/// The defined errors that might occur when loading or saving a table.
/// This is forwarded back by the VirtualMachine.
#[derive(Debug)]
pub enum TableErrors {
    /// The table with the given name was not found.
    TableNotFound,
    /// Attempted to save a table with a name that already exists.
    TableAlreadyExists,
    /// The select column was not found in the table.
    ColumnNotFound(String),
    /// Unspecified write error when saving the table.
    WriteError(String),
    /// Unspecified read error when loading the table.
    ReadError(String),
    /// Generic unspecified error.
    Error(String),
}

/// Defines how the table should be saved.
/// This is a low level API and not meant to used directly
/// by database users.
#[derive(Debug)]
pub enum SaveMode {
    Overwrite,
    Append,
}

impl Table {
    /// Creates the data dir if it does not yet exist.
    pub fn init_data_dir() {
        if !Path::new(DATA_DIR).exists() {
            let result = std::fs::create_dir_all(DATA_DIR);
            result.unwrap();
        }
    }
    /// Resolves the table file path based on it's name and format.
    pub fn get_table_path(name: &String, format: &FileFormat) -> String {
        match format {
            FileFormat::SimpleColumnar => format!("{}/{}.columnar", DATA_DIR, name),
        }
    }
    /// Saves the table to disk.
    pub fn save(&self, mode: SaveMode, format: FileFormat) -> Result<(), TableErrors> {
        let s = Table::get_table_path(&self.name, &format);
        let path = Path::new(&s);
        info!(
            "Saving table in format {:?} ({:?}) to path: {:?}",
            format, mode, path
        );

        // Pick up correct writer
        let writer: Box<dyn Writer>;
        match format {
            FileFormat::SimpleColumnar => {
                writer = ColumnarWriter::new();
            }
        }
        // Adapt to the given mode
        match mode {
            SaveMode::Overwrite => {
                let f = OpenOptions::new().write(true).create_new(true).open(path);
                if f.is_err() {
                    println!("{:?}", f.unwrap_err());
                    return Err(TableErrors::TableAlreadyExists);
                }
                let write_result = writer.write(&self.fields, &self.columns, f.unwrap());
                if write_result.is_err() {
                    let s = format!("{:?}", write_result.unwrap_err());
                    return Err(TableErrors::WriteError(s));
                }
            }

            SaveMode::Append => {
                let f = OpenOptions::new()
                    .append(true)
                    .write(true)
                    .create(false)
                    .open(path);
                if f.is_err() {
                    println!("{:?}", f.unwrap_err());
                    return Err(TableErrors::TableNotFound);
                }
                let write_result = writer.append(&self.fields, &self.columns, f.unwrap());
                if write_result.is_err() {
                    let s = format!("{:?}", write_result.unwrap_err());
                    return Err(TableErrors::WriteError(s));
                }
            }
        }
        return Ok(());
    }
    /// Loads the table from disk.
    pub fn load(
        table_name: String,
        select_columns: Vec<String>,
        format: FileFormat,
    ) -> Result<Table, TableErrors> {
        let s = Table::get_table_path(&table_name, &format);
        let path = Path::new(&s);
        info!("Loading table in format {:?} from path: {:?}", format, path);

        let reader: Box<dyn Reader>;
        match format {
            FileFormat::SimpleColumnar => reader = ColumnarReader::new(),
        };

        let file_ = OpenOptions::new().read(true).open(path);
        if file_.is_err() {
            let error = format!("{:?}", file_.unwrap_err());
            println!("{:?}", error);
            return Err(TableErrors::TableNotFound);
        }

        let f = file_.unwrap();
        let result = reader.read(f, select_columns.clone());
        if result.is_err() {
            let error = format!("{:?}", result.unwrap_err());
            println!("{:?}", error);
            return Err(TableErrors::ReadError(error));
        }

        let (fields, columns) = result.unwrap();
        for select_col in select_columns.iter() {
            if !fields.contains_key(select_col) {
                return Err(TableErrors::ColumnNotFound(select_col.clone()));
            }
        }
        let table = Table {
            name: table_name,
            fields,
            columns,
            select_columns,
        };
        return Ok(table);
    }
}
