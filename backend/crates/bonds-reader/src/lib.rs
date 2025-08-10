mod value_generator;

use std::path::Path;
use anyhow::{Context, Result};
use calamine::{Reader, Xls};

pub fn read_bonds<P: AsRef<Path>>(path: P) -> Result<()> {
 
    let mut workbook: Xls<_> = calamine::open_workbook(path.as_ref())
        .context("Failed to open workbook")?;
    
    let range = workbook.worksheet_range("ROD")
        .context("Failed to get worksheet [ROD]")?;

    // Use the Rows iterator returned by Range::rows().
    for (row_num, row) in range.rows().enumerate() {
        for (col_num, data) in row.iter().enumerate() {
            // Print the data in each cell of the row.
            println!("({row_num}, {col_num}): {data}");
        }
    }
    
    println!("Headers: {:?}", range.headers());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_bonds() {
        let path = "../../assets/Dane_dotyczace_obligacji_detalicznych.xls";
        let result = read_bonds(path);
        assert!(result.is_ok(), "Failed to read bonds: {:?}", result.err());
    }
}