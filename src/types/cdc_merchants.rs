use crate::types::newtypes::PostalCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdcData {
    // pub last_updated: String,
    pub locations: Vec<CdcMerchant>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CdcMerchant {
    // pub id: String,
    // pub entity_id: String,
    // pub name: String,
    pub address: String,
    pub postal_code: PostalCode,
    // pub r#type: MerchantType, // they're all HAWKER_HEARTLAND_MERCHANT
    // pub lat: f64,
    // pub lon: f64,
    pub filters: Filters,
    // pub last_reset_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct Filters {
    pub vouchers: Vouchers,
    // pub secondary: Secondary,
}

#[derive(Serialize, Deserialize)]
pub struct Vouchers {
    // pub supermarket: bool,
    pub hawker_heartland_merchant: bool,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Secondary {
//     pub budgetmeal: bool,
// }
