mod value_generator;

use anyhow::{Context, Result, anyhow, bail};
use calamine::Data::{DateTime, DateTimeIso, Float, String};
use calamine::{DataType, ExcelDateTime, Reader, Xls};
use chrono::{Datelike, TimeDelta};
use model::{Bond, BondId};
use std::collections::HashMap;
use std::ops::Add;
use std::path::Path;

pub fn read_bonds<P: AsRef<Path>>(path: P) -> Result<HashMap<BondId, Bond>> {
    let mut workbook: Xls<_> =
        calamine::open_workbook(path.as_ref()).context("Failed to open workbook")?;

    let range = workbook
        .worksheet_range("ROD")
        .context("Failed to get worksheet [ROD]")?;

    let mut bonds = HashMap::new();

    for (row_id, row) in range.rows().enumerate() {
        let first_cell = row.get(0);
        if let Some(cell) = first_cell
            && let String(value) = cell
            && value.starts_with("ROD")
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
                    "Cannot extract date from cell [{:?}], row id: [{}], column: 3",
                    row.get(4),
                    row_id
                )
            };

            let bond_id = if let Some(String(bond_id)) = row.get(0) {
                BondId::new(bond_id.as_str())
            } else {
                bail!("Cannot extract bond ID from cell [{:?}]", row.get(0))
            };

            let buyout_date = sale_start.with_year(sale_start.year() + 12).unwrap();

            let mut generator = value_generator::ValueGenerator::new(100f64);

            for i in (9..=20) {
                if let Some(cell) = row.get(i)
                    && let Float(value) = cell
                {
                    generator.add_yearly_return(*value)
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
    use chrono::NaiveDate;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_read_rod1235bond() {
        let path = "../../assets/Dane_dotyczace_obligacji_detalicznych.xls";
        let result = read_bonds(path).expect("Should read bondss");
        let rod1235bond_id = BondId::new("ROD1235");
        let rod1235bond = result
            .get(&rod1235bond_id)
            .expect("Should find ROD1235 bond");

        assert_debug_snapshot!(rod1235bond);
    }
}
