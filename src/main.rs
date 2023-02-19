use std::{fs::File, io::Read};

use clap::Parser;
use serde_json::Value;
use serde_json_path::JsonPathExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'p', long)]
    json_path: String,
    #[arg(short, long)]
    file: String,
}

fn main() {
    let cli = Cli::parse();
    let mut file = File::open(cli.file).expect("open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("read file");
    let json = serde_json::from_str::<Value>(contents.as_str()).expect("is valid JSON");
    match json.json_path(cli.json_path.as_str()) {
        Ok(q) => {
            for n in q {
                println!("{n}");
            }
        }
        Err(e) => println!("{e}"),
    }
}
