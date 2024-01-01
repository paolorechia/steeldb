//! The module for defining data types supported by the Database.

/// The supported data type stored by the Database.
/// By using the Enum, we can resolve the column type dynamically in run time.
#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

impl DataType {
    /// Although a public function, this will rarely have an utility
    /// for a database user. This is mostly used when reading data from disk.
    pub fn name(&self) -> String {
        match self {
            Self::String(_) => {
                return "String".to_string();
            }
            Self::Integer32(_) => {
                return "i32".to_string();
            }
            Self::Float32(_) => {
                return "f32".to_string();
            }
        }
    }
}
