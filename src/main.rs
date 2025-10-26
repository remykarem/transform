use crate::types::cdc_merchants::CdcMerchant;
use crate::types::halal_establishments::HalalEstablishment;
use crate::types::halal_establishments::HasCdc::{Maybe, Yes};
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::time::Instant;

mod candidates;
mod repository;
mod types;
mod utils;

fn main() {
    let instant = Instant::now();

    // Load data
    let (mut halal_establishments, cdc_merchants) = repository::load_data();

    // Construct a data structure that will be useful for our data access pattern
    let cdc_merchants_by_postal = repository::group_by_postal_code(&cdc_merchants);

    // Mutate
    halal_establishments
        .par_iter_mut()
        .for_each(|establishment| {
            // Get candidates
            let candidate_cdc_merchants =
                candidates::get_candidate_cdc_merchants(&establishment, &cdc_merchants_by_postal);

            // Mutate
            if let Some(candidates) = candidate_cdc_merchants {
                establishment.update_cdc(&candidates);
            }
        });

    // Write establishments to a json
    repository::write_data(halal_establishments);

    println!("{:?}", instant.elapsed());
}

impl HalalEstablishment {
    fn update_cdc(&mut self, candidates: &[&CdcMerchant]) {
        let has_cdc = match candidates {
            [candidate] => {
                if candidate.filters.vouchers.hawker_heartland_merchant {
                    Some(Yes)
                } else {
                    None
                }
            }
            [] => None,
            _ => Some(Maybe),
        };
        self.cdc = has_cdc;
    }
}
