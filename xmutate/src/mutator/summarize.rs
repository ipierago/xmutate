use serde_json::{Value, Map};
use crate::mutator::Mutator;
use crate::error::MutateError;
use thiserror::Error;

pub struct SummarizeMutator;

#[derive(Debug, Error)]
pub enum SummarizeError {
    #[error("Missing 'group_by' parameter")]
    MissingGroupBy,
    #[error("Input must be an array of objects")]
    InvalidInput,
}

impl From<SummarizeError> for MutateError {
    fn from(err: SummarizeError) -> Self {
        MutateError::Mutator {
            name: "summarize".into(),
            reason: err.to_string(),
        }
    }
}

impl Mutator for SummarizeMutator {
    fn name(&self) -> &'static str {
        "summarize"
    }

    fn run(&self, input: Value, params: &[(String, Value)]) -> Result<Value, MutateError> {
        let group_by = params.iter()
            .find(|(k, _)| k == "group_by")
            .and_then(|(_, v)| v.as_str())
            .ok_or(SummarizeError::MissingGroupBy)?;

        let array = input.as_array().ok_or(SummarizeError::InvalidInput)?;

        let mut counts = Map::new();
        for item in array {
            if let Some(Value::String(key)) = item.get(group_by) {
                let count = counts.entry(key.clone()).or_insert(Value::from(0));
                if let Some(c) = count.as_i64() {
                    *count = Value::from(c + 1);
                }
            }
        }

        Ok(Value::Object(counts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util::{param_map, example_input};
    use serde_json::json;

    #[test]
    fn groups_correctly() {
        let params = param_map(&[("group_by", json!("category"))]);
        let input = example_input();
        let output = SummarizeMutator.run(input, &params).unwrap();

        assert_eq!(output, json!({
            "fruit": 2,
            "vegetable": 1
        }));
    }
}
