use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct BondId(String);

impl BondId {
    pub fn new<S: Into<String>>(id: S) -> Self {
        BondId(id.into())
    }
    pub fn value(self) -> String {
        self.0
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq, bon::Builder)]
pub struct Bond {
    pub id: BondId,
    pub initial_date: NaiveDate,
    pub sale_end: NaiveDate,
    pub buyout_date: NaiveDate,
    pub values: Vec<f64>,
}

impl Bond {
    pub fn to_csv(&self) -> String {
        let mut csv = String::from("date,value\n");

        for (index, value) in self.values.iter().enumerate() {
            let date = self.initial_date + chrono::Duration::days(index as i64);
            csv.push_str(&format!("{},{}\n", date.format("%Y-%m-%d"), value));
        }

        csv
    }
}

pub struct AllBonds {
    pub edo: HashMap<BondId, Bond>,
    pub rod: HashMap<BondId, Bond>,
}
