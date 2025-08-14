use chrono::NaiveDate;

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
