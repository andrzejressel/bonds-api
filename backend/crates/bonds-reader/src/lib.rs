mod value_generator;

use anyhow::{Context, Error, Result, bail};
use calamine::Data::{Float, String};
use calamine::{DataType, Reader, Xls};
use chrono::Datelike;
use model::{AllBonds, Bond, BondId};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub fn read_bonds<P: AsRef<Path>>(path: P) -> Result<AllBonds> {
    let mut workbook: Xls<_> =
        calamine::open_workbook(path.as_ref()).context("Failed to open workbook")?;

    let edo = extract_bond_type(&mut workbook, "EDO", 10)?;
    let rod = extract_bond_type(&mut workbook, "ROD", 12)?;

    let all_bonds = AllBonds { edo, rod };

    Ok(all_bonds)
}

fn extract_bond_type(
    workbook: &mut Xls<BufReader<File>>,
    bond_type: &str,
    bond_length_in_years: u8,
) -> Result<HashMap<BondId, Bond>, Error> {
    let range = workbook
        .worksheet_range(bond_type)
        .context(format!("Failed to get worksheet [{}]", bond_type))?;

    let mut bonds = HashMap::new();

    for (row_id, row) in range.rows().enumerate() {
        let first_cell = row.first();
        if let Some(cell) = first_cell
            && let String(value) = cell
            && value.starts_with(bond_type)
        {
            let sale_start = if let Some(date_time) = row.get(3).and_then(|f| f.as_datetime()) {
                date_time
            } else {
                bail!(
                    "Cannot extract date from cell [{:?}], row id: [{}], column: 3",
                    row.get(3),
                    row_id
                )
            };

            let sale_end = if let Some(date_time) = row.get(4).and_then(|f| f.as_datetime()) {
                date_time
            } else {
                bail!(
                    "Cannot extract date from cell [{:?}], row id: [{}], column: 4",
                    row.get(4),
                    row_id
                )
            };

            let bond_id = if let Some(String(bond_id)) = row.first() {
                BondId::new(bond_id.as_str())
            } else {
                bail!("Cannot extract bond ID from cell [{:?}]", row.first())
            };

            let buyout_date = sale_start
                .with_year(sale_start.year() + bond_length_in_years as i32)
                .unwrap();

            let mut generator = value_generator::ValueGenerator::new(100f64);

            for i in 9..(9 + bond_length_in_years) {
                if let Some(cell) = row.get(i as usize)
                    && let Float(value) = cell
                {
                    let d = Decimal::from_f64_retain(*value).unwrap().round_dp(5);
                    generator.add_yearly_return(d.to_f64().unwrap())
                }
            }

            let bond = Bond::builder()
                .id(bond_id.clone())
                .initial_date(sale_start.date())
                .buyout_date(buyout_date.date())
                .sale_end(sale_end.date())
                .values(generator.calculate_daily_bond_values(sale_start.clone().date()))
                .build();

            bonds.insert(bond_id, bond);
        }
    }
    Ok(bonds)
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_read_rod1235bond() {
        let path = "../../assets/Dane_dotyczace_obligacji_detalicznych.xls";
        let result = read_bonds(path).expect("Should read bondss");
        let rod1235bond_id = BondId::new("ROD1235");
        let rod1235bond = result
            .rod
            .get(&rod1235bond_id)
            .expect("Should find ROD1235 bond");

        assert_debug_snapshot!(rod1235bond);
    }

    // FIXME: It's wrong by 1 grosz at the end. Due to float rounding issues in excel
    #[test]
    fn test_read_edo1224bond() {
        let path = "../../assets/Dane_dotyczace_obligacji_detalicznych.xls";
        let result = read_bonds(path).expect("Should read bonds");
        let edo1224bond_id = BondId::new("EDO1224");
        let edo1224bond = result
            .edo
            .get(&edo1224bond_id)
            .expect("Should find edo1224 bond");

        assert_debug_snapshot!(edo1224bond);
    }

    #[test]
    fn test_read_edo0125bind() {
        let path = "../../assets/Dane_dotyczace_obligacji_detalicznych.xls";
        let result = read_bonds(path).expect("Should read bonds");
        let edo0125bond_id = BondId::new("EDO0125");
        let edo0125bond = result
            .edo
            .get(&edo0125bond_id)
            .expect("Should find edo0125 bond");
        assert_debug_snapshot!(edo0125bond);
    }
}
