use std::ops::Mul;
use anyhow::{bail, Result};

#[derive(Debug)]
#[derive(PartialEq)]
struct Price(i64);

impl Price {
    fn from_value(value: f64) -> Result<Self> {
        if value < 0.0 {
            bail!("Price cannot be negative");
        }
        Ok(Price((value * 100.0).round() as i64))
    }
    
    fn from_raw(value: i64) -> Self {
        Price(value)
    }
}

struct Percentage(i64);

impl Percentage {
    fn from_percentage(value: f64) -> Result<Self> {
        if value < 0.0 {
            bail!("Price cannot be negative");
        }
        Ok(Percentage((value * 100.0).round() as i64))
    }
}

#[cfg(test)]
mod tests {
    use crate::price::Percentage;

    #[test]
    fn percentage_is_rounded() {
        assert_eq!(Percentage::from_percentage(4.999).unwrap().0, 500);
        assert_eq!(Percentage::from_percentage(4.995).unwrap().0, 500);
        assert_eq!(Percentage::from_percentage(4.994).unwrap().0, 499);
        assert_eq!(Percentage::from_percentage(4.99).unwrap().0, 499);
    }
    
}