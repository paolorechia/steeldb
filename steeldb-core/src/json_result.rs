use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct HelloJSON {
    hello: String,
}