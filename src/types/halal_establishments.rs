use crate::types::newtypes::PostalCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct HalalEstablishment {
    pub name: String,
    pub address: String,
    pub r#type: String,
    pub number: String,
    pub scheme: String,
    pub id: String,
    pub postal: PostalCode,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc: Option<HasCdc>,
}

#[derive(Serialize, Deserialize)]
pub enum HasCdc {
    Yes,
    Maybe,
}
