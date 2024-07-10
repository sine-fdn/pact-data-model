use pact_data_model::*;
use schemars::gen::{SchemaGenerator, SchemaSettings};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{Error, Write};

fn main() -> Result<(), Error> {
    let mut openapi_settings = SchemaSettings::draft07();

    openapi_settings.option_nullable = true;

    let schema_generator = SchemaGenerator::from(openapi_settings);

    let schema = schema_generator.into_root_schema_for::<ProductFootprint>();

    let mut schema_json = to_string_pretty(&schema).expect("Failed to serialize schema");

    if !schema_json.ends_with('\n') {
        schema_json.push('\n');
    }

    let mut file = File::create("./schema/data-model-schema.json")?;

    file.write_all(schema_json.as_bytes())?;

    println!("data-model-schema.json successfully created");

    Ok(())
}
