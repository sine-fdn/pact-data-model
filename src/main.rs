use pact_data_model::*;
use schemars::schema_for;
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let schema = schema_for!(ProductFootprint);

    let mut schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    if !schema_json.ends_with('\n') {
        schema_json.push('\n');
    }

    let mut file = File::create("./schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}
