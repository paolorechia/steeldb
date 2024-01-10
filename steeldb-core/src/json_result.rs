use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HelloJSON {
    pub hello: String,
}