use std::collections::HashMap;
use crate::types::cdc_merchants::{CdcData, CdcMerchant};
use crate::types::halal_establishments::HalalEstablishment;
use crate::types::newtypes::PostalCode;

pub fn load_data() -> (Vec<HalalEstablishment>, Vec<CdcMerchant>) {
    let merchants_str = include_str!("../data/cdc_merchants.json");
    let cdc_data: CdcData = serde_json::from_str(merchants_str).unwrap();

    let halal_str = include_str!("../data/halal_establishments.json");
    let establishments: Vec<HalalEstablishment> = serde_json::from_str(halal_str).unwrap();

    (establishments, cdc_data.locations)
}

pub fn write_data(halal_establishments: Vec<HalalEstablishment>) {
    let json = serde_json::to_string_pretty(&halal_establishments).unwrap();
    std::fs::write("../data/halal_establishments_new.json", json).unwrap();
}

pub fn group_by_postal_code(cdc_merchants: &[CdcMerchant]) -> HashMap<&PostalCode, Vec<&CdcMerchant>> {
    let mut cdc_merchants_by_postal: HashMap<&PostalCode, Vec<&CdcMerchant>> =
        HashMap::with_capacity(cdc_merchants.len());

    for merchant in cdc_merchants {
        cdc_merchants_by_postal
            .entry(&merchant.postal_code)
            .or_insert_with(|| Vec::with_capacity(8))
            .push(merchant);
    }

    cdc_merchants_by_postal
}