use serde_json::{Map, Value};
use std::fs;

pub mod error;
pub mod mutator;

#[cfg(test)]
pub mod test_util;

pub fn run(
    mutator_name: &str,
    input_path: &str,
    output_path: &str,
    params: &Map<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let input_json = fs::read_to_string(input_path)?;
    let output_json = run_in_memory(mutator_name, &input_json, params)?;
    fs::write(output_path, output_json)?;
    Ok(())
}

pub fn run_in_memory(
    mutator_name: &str,
    input_json: &str,
    params: &Map<String, Value>,
) -> Result<String, Box<dyn std::error::Error>> {
    let input: Value = serde_json::from_str(input_json)?;

    let mutator = mutator::get_mutator(mutator_name)
        .ok_or_else(|| format!("Unknown mutator: {}", mutator_name))?;

    let param_pairs: Vec<(String, Value)> =
        params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    let result = mutator.run(input, &param_pairs)?;

    Ok(serde_json::to_string_pretty(&result)?)
}
