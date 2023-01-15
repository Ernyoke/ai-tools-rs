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
    /// Linux (Unix) command. For example: ls -lart
    cmd: String,
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
    You are given the following Unix/Linux command: {cmd}.
    Please explain clearly what will this command accomplish.
    If the command is invalid, state it clearly, otherwise avoid saying that "This command is valid" and
    provide just the explanation.
    "#;

    let mut vars: HashMap<String, String> = HashMap::new();
    vars.insert("cmd".to_string(), args.cmd);

    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .n(1)
        .prompt(strfmt(&prompt, &vars)?)
        .temperature(0.5)
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
