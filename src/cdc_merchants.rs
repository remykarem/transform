use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Merchants {
    pub last_updated: String,
    pub locations: Vec<Merchant>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Merchant {
    pub id: String,
    pub entity_id: String,
    pub name: String,
    pub address: String,
    pub postal_code: String,
    // pub r#type: MerchantType, // they're all HAWKER_HEARTLAND_MERCHANT
    // pub lat: f64,
    // pub lon: f64,
    pub filters: Filters,
    // pub last_reset_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filters {
    pub vouchers: Vouchers,
    pub secondary: Secondary,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vouchers {
    pub supermarket: bool,
    pub hawker_heartland_merchant: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Secondary {
    pub budgetmeal: bool,
}
