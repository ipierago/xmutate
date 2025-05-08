pub mod summarize;

use serde_json::Value;
use crate::error::MutateError;

pub trait Mutator {
    fn name(&self) -> &'static str;
    fn run(&self, input: Value, params: &[(String, Value)]) -> Result<Value, MutateError>;
}

pub fn get_mutator(name: &str) -> Option<Box<dyn Mutator>> {
    match name {
        "summarize" => Some(Box::new(summarize::SummarizeMutator)),
        _ => None,
    }
}
