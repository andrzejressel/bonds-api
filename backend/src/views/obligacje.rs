use serde::{Deserialize, Serialize};

impl ObligacjeResponse {
    #[must_use]
    pub fn new(obligacje: Vec<String>) -> Self {
        Self { obligacje }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct ObligacjeResponse {
    pub obligacje: Vec<String>,
}
