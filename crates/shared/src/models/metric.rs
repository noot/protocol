use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Entry {
    pub key: Key,
    pub value: f64,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Key {
    pub task_id: String,
    pub label: String,
}

impl Entry {
    /// Creates a new `MetricEntry` with the given key and value.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not finite (NaN, positive infinity, or negative infinity).
    pub fn new(key: Key, value: f64) -> Result<Self> {
        let entry = Self { key, value };
        entry.validate()?;
        Ok(entry)
    }

    /// Validates the metric entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the value is not finite (NaN, positive infinity, or negative infinity).
    pub fn validate(&self) -> Result<()> {
        if !self.value.is_finite() {
            bail!("Value must be a finite number");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_metric() -> Result<()> {
        let key = Key {
            task_id: "task1".to_string(),
            label: "cpu".to_string(),
        };

        let valid_values = vec![1.0, 100.0, -5.0, 0.0];
        for value in valid_values {
            let entry = Entry::new(key.clone(), value)?;
            assert!(entry.validate().is_ok());
        }
        Ok(())
    }

    #[test]
    fn test_invalid_metrics() {
        let key = Key {
            task_id: "task1".to_string(),
            label: "cpu".to_string(),
        };

        let invalid_values = vec![(f64::INFINITY, "infinite value"), (f64::NAN, "NaN value")];
        for (value, case) in invalid_values {
            let entry = Entry::new(key.clone(), value);
            assert!(entry.is_err(), "Should fail for {case}");
        }
    }

    #[test]
    fn test_metric_key() {
        let key1 = Key {
            task_id: "task1".to_string(),
            label: "cpu".to_string(),
        };
        let key2 = Key {
            task_id: "task1".to_string(),
            label: "cpu".to_string(),
        };
        let key3 = Key {
            task_id: "task2".to_string(),
            label: "cpu".to_string(),
        };
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }
}
