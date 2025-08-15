use anyhow::{Context, Result};
use itertools::Itertools;
use model::{Bond, BondId};
use std::path::Path;

pub(crate) trait BondsService {
    fn get_bonds(&self) -> Vec<BondId>;
    fn get_bond(&self, id: &BondId) -> Option<&Bond>;
}

pub(crate) struct BondsServiceImpl {
    map: std::collections::HashMap<BondId, Bond>,
}

impl BondsServiceImpl {
    pub(crate) fn new<P: AsRef<Path>>(directory: P) -> Result<Self> {
        let all_bonds = bonds_reader::read_bonds(directory.as_ref())
            .with_context(|| format!("Failed to read Bonds from directory: {}", directory.as_ref().display()))?;
        let mut map = std::collections::HashMap::new();
        for bond in all_bonds.edo.values().chain(all_bonds.rod.values()) {
            map.insert(bond.id.clone(), bond.clone());
        }
        
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
