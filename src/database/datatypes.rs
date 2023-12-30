#[derive(Debug, Clone)]
pub enum DataType {
    String(String),
    Integer32(i32),
    Float32(f32),
}

pub enum Unknown {
    Wrapped(DataType),
}

impl DataType {
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
