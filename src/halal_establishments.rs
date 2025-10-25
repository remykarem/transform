
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct HalalEstablishment {
    name: String,
    address: String,
    r#type: String,
    number: String,
    scheme: String,
    id: String,
    postal: String,
    latitude: f64,
    longitude: f64,
}