use crate::cdc_merchants::{CdcData, CdcMerchant};
use crate::halal_establishments::HalalEstablishment;
use crate::halal_establishments::HasCdc::{Maybe, Yes};
use crate::newtypes::PostalCode;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use regex::Regex;
use std::collections::HashMap;
use std::time::Instant;

mod cdc_merchants;
mod halal_establishments;
mod newtypes;

fn main() {
    let instant = Instant::now();

    // Load data
    let (mut establishments, cdc_merchants) = load_data();

    // Construct a data structure that will be useful for our data access pattern
    let mut cdc_merchants_by_postal: HashMap<&PostalCode, Vec<&CdcMerchant>> =
        HashMap::with_capacity(cdc_merchants.len());
    for merchant in &cdc_merchants {
        cdc_merchants_by_postal
            .entry(&merchant.postal_code)
            .or_insert_with(Vec::new)
            .push(merchant);
    }

    establishments.par_iter_mut().for_each(|establishment| {
        // Get similar merchants
        let candidate_cdc_merchants =
            get_candidate_cdc_merchants(&establishment, &cdc_merchants_by_postal);

        // Tag
        match candidate_cdc_merchants {
            None => {}
            Some(mut similar_merchants) => {
                let has_cdc = match similar_merchants.len() {
                    0 => None,
                    1 => {
                        if similar_merchants
                            .pop()
                            .unwrap()
                            .filters
                            .vouchers
                            .hawker_heartland_merchant
                        {
                            Some(Yes)
                        } else {
                            None
                        }
                    }
                    _ => Some(Maybe),
                };
                establishment.cdc = has_cdc;
            }
        }
    });

    // Write establishments to a json
    let json = serde_json::to_string_pretty(&establishments).unwrap();
    std::fs::write("halal_establishments_new.json", json).unwrap();

    println!("{:?}", instant.elapsed());
}

fn load_data() -> (Vec<HalalEstablishment>, Vec<CdcMerchant>) {
    let merchants_str = include_str!("cdc_merchants.json");
    let cdc_data: CdcData = serde_json::from_str(merchants_str).unwrap();

    let halal_str = include_str!("halal_establishments.json");
    let establishments: Vec<HalalEstablishment> = serde_json::from_str(halal_str).unwrap();

    (establishments, cdc_data.locations)
}

fn get_candidate_cdc_merchants<'a>(
    establishment: &HalalEstablishment,
    cdc_merchants_by_postal: &'a HashMap<&PostalCode, Vec<&CdcMerchant>>,
) -> Option<Vec<&'a CdcMerchant>> {
    let candidates = cdc_merchants_by_postal.get(&establishment.postal);

    let Some(candidates) = candidates else {
        return None;
    };

    let mut merchants: Vec<&CdcMerchant> = Vec::with_capacity(candidates.len());
    for candidate in candidates {
        if compare_merchant(candidate, establishment) {
            merchants.push(candidate);
        }
    }

    // if merchants.len() > 1 {
        // println!("------- {}", establishment.name);
        // for merchant in &merchants {
        //     println!("• {}", merchant.name);
        // }
    // }

    Some(merchants)
}

fn compare_merchant(merchant: &CdcMerchant, establishment: &HalalEstablishment) -> bool {
    // 1. Filter by postal code
    let same_postal = merchant.postal_code == establishment.postal;
    // Short circuit
    if !same_postal {
        return false;
    }

    // 2. Filter by unit no
    let merchant_unit = extract_unit(&merchant.address);
    let halal_unit = extract_unit(&establishment.address);
    match (merchant_unit, halal_unit) {
        (Ok(m), Ok(h)) => m == h,
        _ => false,
    }
    
    // 3. Compare by name
}

fn extract_unit(address: &str) -> Result<String, String> {
    let re = Regex::new(r#"(#[A-Za-z0-9]+[-[A-Za-z0-9/]+]+)"#).unwrap();
    match re.captures(address) {
        Some(caps) => Ok(caps.get(1).unwrap().as_str().to_string()),
        None => Err("No valid unit found".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1234 Main St #123-01", "#123-01")]
    #[case("23 SERANGOON CENTRAL #B2-49 NEX 556083", "#B2-49")]
    #[case(
        "11 BEDOK NORTH STREET 1 #01-28/29 HEARTBEAT @ BEDOK, Stall 5 469662",
        "#01-28/29"
    )]
    // #[case("23 UPPER DICKSON ROAD 01-01 207482", "eturn err )]
    // #[case("101 THOMSON ROAD #01-14 / 15 / 16 UNITED SQUARE 307591", "eturn err )]
    #[case("421C NORTHSHORE DRIVE #01-01, Stall 9 823421", "#01-01")]
    #[case(
        "11 JALAN TAN TOCK SENG #01-13/14/15 TAN TOCK SENG HOSPITAL 308433",
        "#01-13/14/15"
    )]
    #[case(
        "80 AIRPORT BOULEVARD #M021-53 CHANGI AIRPORT T1 Departure/Transit Lounge West 819642",
        "#M021-53"
    )]
    #[case(
        "33 SENGKANG WEST AVENUE #01-09-14 THE SELETAR MALL 797653",
        "#01-09-14"
    )]
    fn test_extract_unit(#[case] address: &str, #[case] expected: &str) {
        assert_eq!(extract_unit(address).unwrap(), expected.to_string());
    }
}
