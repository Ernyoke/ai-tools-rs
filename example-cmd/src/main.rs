use anyhow::anyhow;
use async_openai::types::CreateCompletionRequestArgs;
use async_openai::Client;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use std::path::Path;
use strfmt::strfmt;
use tokio::fs::read_to_string;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Short description of the command for which we would like an example. Example: list files which start with my_pic"
    cmd_description: String,

    /// Number of examples to be requested"
    #[arg(short, long, default_value_t = 1)]
    nr_examples: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    #[serde(rename = "api-key")]
    api_key: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Arguments = Arguments::parse();

    let config_path = match home::home_dir() {
        Some(path) => path.join(".config").join("openai").join("config.json"),
        None => Path::new(".").to_path_buf(),
    };

    if !config_path.exists() {
        return Err(anyhow!("No config file with API key is provided!"));
    }

    let config = read_config(config_path).await?;

    let client = Client::new().with_api_key(config.api_key);
    let prompt = r#"
    Give me {nr_examples} example(s) on how to accomplish the following task under a Linux terminal: {cmd_description}
    Avoid explanation, show me just the command.
    "#;

    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("nr_examples".to_string(), args.nr_examples.to_string());
    vars.insert("cmd_description".to_string(), args.cmd_description);

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .n(1)
        .prompt(strfmt(&prompt, &vars)?)
        .temperature(0.1)
        .max_tokens(512_u16)
        .build()?;

    let response = client.completions().create(request).await?;
    let choice = response.choices.iter().nth(0).unwrap();

    println!("{}", choice.text.trim());
    Ok(())
}

async fn read_config<P: AsRef<Path>>(path: P) -> Result<Config, io::Error> {
    let contents = read_to_string(path).await?;
    let config = serde_json::from_str::<Config>(&*contents)?;
    Ok(config)
}
