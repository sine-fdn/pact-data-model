use pact_data_model::*;
use schemars::schema_for;
use serde_json::{to_string_pretty, Value};
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let mut schema = schema_for!(ProductFootprint<Value>);

    if let Some(metadata) = schema.schema.metadata.as_mut() {
        metadata.title = Some("ProductFootprint".to_string());
    }
    let schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    let mut file = File::create("./schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}
