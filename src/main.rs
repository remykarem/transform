use crate::cdc_merchants::Merchants;
use crate::halal_establishments::HalalEstablishment;

mod cdc_merchants;
mod halal_establishments;

fn main() {
    println!("Hello, world!");

    let merchants_str = include_str!("cdc_merchants.json");
    let merchants: Merchants = serde_json::from_str(merchants_str).unwrap();
    println!("{:#?}", merchants.locations.len());

    let halal_str = include_str!("halal_establishments.json");
    let establishments: Vec<HalalEstablishment> = serde_json::from_str(halal_str).unwrap();
    println!("{:#?}", establishments.len());
}
