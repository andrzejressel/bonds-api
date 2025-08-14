use crate::services::bonds::{Bond, BondId, load_bonds_from_directory};
use anyhow::{Context, anyhow};
use chrono::NaiveDate;
use loco_rs::prelude::*;
use std::path::Path;

trait BondsDatastore {
    fn get_bond(&self, id: &BondId) -> Option<Bond>;
    fn get_bonds_sale_time_range(&self) -> (NaiveDate, NaiveDate);
    fn get_bonds_buyout_time_range(&self) -> (NaiveDate, NaiveDate);
    fn get_bond_for_sale_date(&self, date: NaiveDate) -> Option<Bond>;
    fn get_bond_for_buyout_date(&self, date: NaiveDate) -> Option<Bond>;
}

struct InMemoryBondsDatabase {
    bonds: Vec<Bond>,
}

impl InMemoryBondsDatabase {
    pub(crate) fn from_directory<P: AsRef<Path>>(path: P) -> Result<InMemoryBondsDatabase> {
        let mut bonds = load_bonds_from_directory(path)
            .context("Failed to load bonds from directory")
            .map_err(|e| Error::from(e.into_boxed_dyn_error()))?;

        bonds.sort_by_key(|b| b.initial_date());

        for win in bonds.windows(2) {
            let first_sale_end = win[0].sale_end();
            let second_initial_date = win[1].initial_date();
            if first_sale_end + chrono::Duration::days(1) != second_initial_date {
                return Err(Error::from(
                    anyhow!(
                        "Sale end [{}] + 1 days != next sale start [{}]",
                        first_sale_end,
                        second_initial_date
                    )
                    .into_boxed_dyn_error(),
                ));
            }
        }

        todo!()
        // let service = BondsServiceImpl::new(path)?;
        // let bonds = service.get_bonds().iter().map(|f| service.get_bond(f).unwrap().clone()).collect();
        // Ok(InMemoryBondsDatabase { bonds })
    }

    // fn find_bond_by_date(&self, date: NaiveTime) -> () {
    //     self.bonds
    //         .binary_search_by(|f| {
    //             if f.sale_date < date {
    //                 std::cmp::Ordering::Less
    //             } else if f.sale_date > date {
    //                 std::cmp::Ordering::Greater
    //             } else {
    //                 std::cmp::Ordering::Equal
    //             }
    //         });
    //
    //     ()
    // }
}

#[cfg(test)]
mod tests {}
