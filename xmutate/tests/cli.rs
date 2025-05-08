use std::fs::{self, create_dir_all};
use std::path::Path;
use std::process::Command;

fn clean_tmp_dir() {
    let tmp = std::path::Path::new("tests/tmp");
    if tmp.exists() {
        for entry in std::fs::read_dir(tmp).unwrap() {
            let path = entry.unwrap().path();
            let _ = std::fs::remove_file(path);
        }
    } else {
        std::fs::create_dir_all(tmp).unwrap();
    }
}

#[test]
fn cli_summarize_works() {
    clean_tmp_dir();
    let tmp_dir = Path::new("tests/tmp");
    let input_path = tmp_dir.join("input.json");
    let output_path = tmp_dir.join("output.json");

    create_dir_all(&tmp_dir).unwrap();

    fs::write(
        &input_path,
        r#"
        [
            { "category": "fruit" },
            { "category": "fruit" },
            { "category": "veg" }
        ]
    "#,
    )
    .unwrap();

    let status = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--",
            "run",
            "--mutator",
            "summarize",
            "--input",
            input_path.to_str().unwrap(),
            "--output",
            output_path.to_str().unwrap(),
            "--param",
            "group_by=category",
        ])
        .status()
        .unwrap();

    assert!(status.success());

    let output = fs::read_to_string(&output_path).unwrap();
    assert!(output.contains("\"fruit\": 2"));
    assert!(output.contains("\"veg\": 1"));
}
