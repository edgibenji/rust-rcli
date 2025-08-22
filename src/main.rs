use clap::{Parser, Subcommand};
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;
// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Parser, Debug)]
#[command(name = "rcli", version, author, about = "convert CSV to JSON", long_about = None)]
struct Options {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(name = "csv", about = "Convert CSV to JSON")]
    Csv(CsvOpts),
    #[command(name = "json", about = "Convert JSON to CSV")]
    Json(JsonOpts),
}

#[derive(Parser, Debug)]
struct JsonOpts {
    #[arg(short, long, value_name = "abc")]
    input: String,
}
#[derive(Parser, Debug)]
struct CsvOpts {
    #[arg(short, long, value_name = "INPUT", value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, value_name = "OUTPUT", default_value = "output.json")]
    output: String,

    #[arg(long, value_name = "HEADER", default_value_t = true)]
    header: bool,

    #[arg(short, long, value_name = "DELIMITER", default_value_t = ',')]
    delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("Input file cannot be empty".into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    println!("{opts:?}");

    match opts.command {
        Commands::Csv(csv_opts) => {
            let mut rdr = Reader::from_path(&csv_opts.input)?;
            for result in rdr.deserialize() {
                let record: Player = result?;
                println!("{record:?}");
            }
            // println!("{:?}", record);
        }
        _ => {
            println!("This command is not implemented yet.");
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    #[test]
    fn dummy_test() {
        assert_eq!(2 + 2, 4);
    }
}
