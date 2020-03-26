#![allow(unused_variables)]
use anyhow::Result;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct MyData {
    field_one: usize,
    field_two: String,
    field_three: bool,
    some_data: std::collections::HashMap<String, usize>,
}

fn main() -> Result<()> {
    let my_data_yaml = r#"
        fieldOne: 7
        fieldTwo: "lorem"
        fieldThree: true
        someData:
            x: 1
            y: 2
            z: 3
        "#;

    let my_data_toml = r#"
        fieldOne = 7
        fieldTwo = "lorem"
        fieldThree = true

        [someData]
        x = 1
        y = 2
        z = 3
        "#;

    let my_data_json = r#"
        {
          "fieldOne": 7,
          "fieldTwo": "lorem",
          "fieldThree": true,
          "someData": {
            "x": 1,
            "y": 2,
            "z": 3
          }
        }
        "#;

    let deserialized_yaml = serde_yaml::from_str::<MyData>(my_data_yaml);
    let deserialized_toml = toml::from_str::<MyData>(my_data_toml);
    let deserialized_json = serde_json::from_str::<MyData>(my_data_json);

    assert!(deserialized_yaml.is_ok());
    assert!(deserialized_toml.is_ok());
    assert!(deserialized_json.is_ok());

    let deserialized_toml_copy = deserialized_toml.clone();

    assert_eq!(deserialized_yaml?, deserialized_toml?);
    assert_eq!(deserialized_toml_copy?, deserialized_json?);

    let my_data_yaml_missing_field = r#"
        fieldOne: 7
        fieldTwo: "lorem"
        someData:
            x: 1
            y: 2
            z: 3
        "#;

    let my_data_yaml_extra_field = r#"
        fieldOne: 7
        fieldTwo: "lorem"
        fieldThree: true
        someData:
            x: 1
            y: 2
            z: 3
        out_of_schema_data: 42
        "#;

    let data_missing_field = serde_yaml::from_str::<MyData>(my_data_yaml_missing_field);
    let data_extra_field = serde_yaml::from_str::<MyData>(my_data_yaml_extra_field);

    assert!(data_missing_field.is_err());
    // Because MyData is decorated with `deny_unknown_fields`, adding extra fields
    // will cause parsing to fail.
    assert!(data_extra_field.is_err());

    let yaml_data = serde_yaml::from_str::<serde_yaml::Value>(my_data_yaml)?;
    let toml_data = toml::from_str::<toml::Value>(my_data_toml)?;
    let json_data = serde_json::from_str::<serde_json::Value>(my_data_json)?;

    let toml_from_yaml = serde_yaml::from_str::<toml::Value>(my_data_yaml)?;
    let toml_from_json = serde_json::from_str::<toml::Value>(my_data_json)?;
    let yaml_from_toml = toml::from_str::<serde_yaml::Value>(my_data_toml)?;
    let yaml_from_json = serde_json::from_str::<serde_yaml::Value>(my_data_json)?;
    let json_from_toml = toml::from_str::<serde_json::Value>(my_data_toml)?;
    let json_from_yaml = serde_yaml::from_str::<serde_json::Value>(my_data_yaml)?;

    let json_schema = serde_json::from_str::<schemars::schema::RootSchema>(
        &std::fs::read_to_string("example.schema.json")?,
    )?;

    let json_schema_from_yaml = serde_yaml::from_str::<schemars::schema::RootSchema>(
        &std::fs::read_to_string("example.schema.yaml")?,
    )?;

    // As YAML is a superset of JSON, you can also do this.
    let json_schema_from_json_via_yaml = serde_yaml::from_str::<schemars::schema::RootSchema>(
        &std::fs::read_to_string("example.schema.json")?,
    )?;

    println!("All good!");

    Ok(())
}
