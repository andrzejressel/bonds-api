use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub(crate) struct BondId(String);

impl BondId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        BondId(id.into())
    }
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct BondData {
    first_date: NaiveDate,
    values: Vec<f64>,
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub(crate) struct Bond {
    id: BondId,
    initial_date: NaiveDate,
    values: Vec<f64>,
}

impl Bond {
    fn from_file_data(filename: &str, data: BondData) -> Result<Self> {
        let bond_id = filename
            .strip_suffix(".json")
            .unwrap_or(filename)
            .to_string();

        Ok(Bond {
            id: BondId(bond_id),
            initial_date: data.first_date,
            values: data.values,
        })
    }

    pub fn to_csv(&self) -> String {
        let mut csv = String::from("date,value\n");

        for (index, value) in self.values.iter().enumerate() {
            let date = self.initial_date + chrono::Duration::days(index as i64);
            csv.push_str(&format!("{},{}\n", date.format("%Y-%m-%d"), value));
        }

        csv
    }
}

/// Reads all JSON files from the specified directory and deserializes them into Bond structs
fn load_bonds_from_directory<P: AsRef<Path>>(directory: P) -> Result<Vec<Bond>> {
    let dir_path = fs::canonicalize(directory.as_ref()).with_context(|| {
        format!(
            "Failed to canonicalize path: {}",
            directory.as_ref().display()
        )
    })?;

    if !dir_path.exists() {
        anyhow::bail!("directory {:?} does not exist", dir_path);
    }

    if !dir_path.is_dir() {
        anyhow::bail!("directory {:?} is not a directory", dir_path);
    }

    let mut bonds = Vec::new();

    let entries = fs::read_dir(&dir_path)
        .with_context(|| format!("Failed to read directory: {}", dir_path.display()))?;

    for entry in entries {
        let entry = entry.with_context(|| "Failed to read directory entry")?;
        let path = entry.path();

        // Only process .json files
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let filename = path
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("Invalid filename: {}", path.display()))?;

            let file_content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read file: {}", path.display()))?;

            let bond_data: BondData = serde_json::from_str(&file_content)
                .with_context(|| format!("Failed to parse JSON from file: {}", path.display()))?;

            let bond = Bond::from_file_data(filename, bond_data)
                .with_context(|| format!("Failed to create Bond from file: {filename}"))?;

            bonds.push(bond);
        }
    }

    Ok(bonds)
}

pub(crate) trait BondsService {
    fn get_bonds(&self) -> Vec<BondId>;
    fn get_bond(&self, id: &BondId) -> Option<&Bond>;
}

pub(crate) struct BondsServiceImpl {
    map: std::collections::HashMap<BondId, Bond>,
}

impl BondsServiceImpl {
    pub(crate) fn new<P: AsRef<Path>>(directory: P) -> Result<Self> {
        let bonds = load_bonds_from_directory(directory)?;
        let map = bonds
            .into_iter()
            .map(|bond| (bond.id.clone(), bond))
            .collect();
        Ok(Self { map })
    }
}

impl BondsService for BondsServiceImpl {
    fn get_bonds(&self) -> Vec<BondId> {
        let mut v: Vec<_> = self.map.keys().cloned().collect();
        v.sort();
        v
    }

    fn get_bond(&self, id: &BondId) -> Option<&Bond> {
        self.map.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_load_bonds_from_directory() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        // Create test JSON files
        let test_json1 = r#"{
            "first_date": "2022-03-01",
            "values": [0.0, 0.01, 0.02, 0.03]
        }"#;

        let test_json2 = r#"{
            "first_date": "2023-01-15",
            "values": [1.0, 1.5, 2.0]
        }"#;

        // Write test files
        let mut file1 = fs::File::create(temp_path.join("EDO0332.json")).unwrap();
        file1.write_all(test_json1.as_bytes()).unwrap();

        let mut file2 = fs::File::create(temp_path.join("TEST123.json")).unwrap();
        file2.write_all(test_json2.as_bytes()).unwrap();

        // Create a non-JSON file that should be ignored
        let mut txt_file = fs::File::create(temp_path.join("ignore.txt")).unwrap();
        txt_file.write_all(b"This should be ignored").unwrap();

        // Test the function
        let bonds = load_bonds_from_directory(temp_path).expect("Failed to load bonds");

        // Verify results
        assert_eq!(bonds.len(), 2);

        // Find bonds by ID
        let edo_bond = bonds
            .iter()
            .find(|b| matches!(&b.id, BondId(id) if id == "EDO0332"))
            .unwrap();
        let test_bond = bonds
            .iter()
            .find(|b| matches!(&b.id, BondId(id) if id == "TEST123"))
            .unwrap();

        // Verify EDO0332 bond
        assert_eq!(
            edo_bond.initial_date,
            NaiveDate::from_ymd_opt(2022, 3, 1).unwrap()
        );
        assert_eq!(edo_bond.values, vec![0.0, 0.01, 0.02, 0.03]);

        // Verify TEST123 bond
        assert_eq!(
            test_bond.initial_date,
            NaiveDate::from_ymd_opt(2023, 1, 15).unwrap()
        );
        assert_eq!(test_bond.values, vec![1.0, 1.5, 2.0]);
    }

    #[test]
    fn test_load_bonds_service_method() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path();

        let test_json = r#"{
            "first_date": "2024-01-01",
            "values": [5.0, 5.5, 6.0]
        }"#;

        let mut file = fs::File::create(temp_path.join("SERVICE_TEST.json")).unwrap();
        file.write_all(test_json.as_bytes()).unwrap();

        let service = BondsServiceImpl::new(temp_dir).expect("Failed to create BondsServiceImpl");

        let bond_ids = service.get_bonds();

        assert_eq!(bond_ids, vec![BondId("SERVICE_TEST".to_string())]);
    }

    #[test]
    fn test_bond_to_csv() {
        // Create a test bond
        let bond = Bond {
            id: BondId("TEST_CSV".to_string()),
            initial_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            values: vec![100.0, 100.5, 101.0, 99.5],
        };

        let csv_output = bond.to_csv();

        let expected_csv =
            "date,value\n2024-01-01,100\n2024-01-02,100.5\n2024-01-03,101\n2024-01-04,99.5\n";

        assert_eq!(csv_output, expected_csv);
    }
}
