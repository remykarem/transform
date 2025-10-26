use crate::types::cdc_merchants::CdcMerchant;
use crate::types::halal_establishments::HalalEstablishment;
use crate::types::newtypes::PostalCode;
use crate::utils;
use std::collections::HashMap;

pub fn compare_merchant(merchant: &CdcMerchant, establishment: &HalalEstablishment) -> bool {
    // 1. Filter by postal code
    let same_postal = merchant.postal_code == establishment.postal;
    // Short circuit
    if !same_postal {
        return false;
    }

    // 2. Filter by unit no
    let merchant_unit = utils::extract_unit(&merchant.address);
    let halal_unit = utils::extract_unit(&establishment.address);
    match (merchant_unit, halal_unit) {
        (Ok(m), Ok(h)) => m == h,
        _ => false,
    }

    // 3. Compare by name
}

pub fn get_candidate_cdc_merchants<'a>(
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
