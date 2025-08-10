use chrono::{Datelike, NaiveDate};

#[derive(Debug, Clone, PartialEq)]
pub struct BondValue {
    pub date: NaiveDate,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ValueGenerator {
    yearly_returns: Vec<f64>,
    initial_value: f64,
}

impl ValueGenerator {
    pub fn new(initial_value: f64) -> Self {
        Self {
            yearly_returns: Vec::new(),
            initial_value,
        }
    }

    pub fn add_yearly_return(&mut self, return_rate: f64) {
        self.yearly_returns.push(return_rate);
    }

    /// Calculates bond values for every single day starting from the given date
    /// Returns a list of bond values with daily compounding interest
    pub fn calculate_daily_bond_values(
        &self,
        start_date: NaiveDate,
    ) -> Vec<f64> {
        let mut values = Vec::new();
        values.push(self.initial_value);
        let mut current_value = self.initial_value;
        
        for (i, annual_return_rate) in self.yearly_returns.iter().enumerate() {
            let start_date = start_date
                .with_year(start_date.year() + (i as i32)).unwrap();
            let end_date = start_date
                .with_year(start_date.year() + 1).unwrap();
            
            let days_in_year = (end_date - start_date).num_days();
            
            println!("Calculating values for year {}: {} to {}", i + 1, start_date, end_date);
            println!("Number of days in year: {}", days_in_year);
            
            for day in 1..=days_in_year {
                let additional_value = current_value * (day as f64 / days_in_year as f64) * (annual_return_rate);
                let today_value = current_value + additional_value;
                let today_value = Self::two_decimal_places(today_value);
                values.push(today_value);
            }
            
            println!("Last value for year {}: {}", i + 1, values.last().unwrap());
            current_value = values.last().unwrap().clone();
        }
        
        values
    }
    
    fn two_decimal_places(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_daily_bond_value_calculation_like_rod1235() {
        let mut generator = ValueGenerator::new(100.0);
        generator.add_yearly_return(0.0725);
        generator.add_yearly_return(0.07);

        let start_date = NaiveDate::from_ymd_opt(2023, 12, 01).unwrap();
        let values = generator.calculate_daily_bond_values(start_date);
        
        assert_debug_snapshot!(values[..=366]);
        assert_debug_snapshot!(values[367..]);
    }
}