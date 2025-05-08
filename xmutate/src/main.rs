use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "xmutate", version, author)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        mutator: String,
        #[arg(long)]
        input: String,
        #[arg(long)]
        output: String,
        #[arg(long, value_parser = parse_key_val, num_args = 0..)]
        param: Vec<(String, serde_json::Value)>,
    },
}

fn parse_key_val(s: &str) -> Result<(String, serde_json::Value), String> {
    let parts: Vec<_> = s.splitn(2, '=').collect();
    if parts.len() != 2 {
        return Err(format!("Expected key=value, got '{}'", s));
    }

    let key = parts[0].to_string();
    let raw_val = parts[1];

    let value = match serde_json::from_str(raw_val) {
        Ok(v) => v,
        Err(_) => serde_json::Value::String(raw_val.to_string()),
    };

    Ok((key, value))
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { mutator, input, output, param } => {
            let param_map: serde_json::Map<String, serde_json::Value> =
                param.into_iter().collect();
            xmutate::run(&mutator, &input, &output, &param_map)?;
        }
    }
    
    Ok(())
}
