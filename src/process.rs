use anyhow::Result;
use csv::Reader;
use serde::{Deserialize, Serialize};

use std::fs;

use crate::opts::OutputFormat;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct _Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: String, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);

    let headers = reader.headers()?.clone();

    for resulit in reader.records() {
        let record = resulit?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        //println!("{:?}", record);
        ret.push(json_value);
    }

    let context: String = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    //let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, context)?; // => ()
    Ok(())
}
