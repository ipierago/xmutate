use std::fs;
use std::process::Command;

#[test]
fn cli_summarize_works() {
    fs::write("input.json", r#"[{"category":"x"},{"category":"y"},{"category":"x"}]"#).unwrap();

    Command::new("cargo")
        .args(&["run", "--", "run", "--mutator", "summarize", "--input", "input.json", "--output", "out.json", "--param", "group_by=category"])
        .status()
        .unwrap();

    let output = fs::read_to_string("out.json").unwrap();
    assert!(output.contains("\"x\": 2"));
}
