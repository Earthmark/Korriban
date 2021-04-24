use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct Manifest {
    name: String,
    version: i32,
    extensions: Vec<String>,
    templates: Vec<TemplateDefinition>,
    wat: String,
}

#[derive(Deserialize)]
struct TemplateDefinition {
    name: String,
    imputs: BTreeMap<i32, FieldDefinition>,
    outputs: BTreeMap<i32, FieldDefinition>,
    default: String,
}

#[derive(Deserialize)]
struct FieldDefinition {
    name: String,
    field_type: String,
}
