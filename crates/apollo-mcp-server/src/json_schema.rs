/// Macro to generate a JSON schema from a type
#[macro_export]
macro_rules! schema_from_type {
    ($type:ty) => {{
        let schema = schemars::SchemaGenerator::new(
            schemars::generate::SchemaSettings::draft07(),
        )
        .into_root_schema_for::<$type>();

        match serde_json::to_value(schema) {
            Ok(Value::Object(schema)) => schema,
            _ => panic!("Failed to generate schema for {}", stringify!($type)),
        }
    }};
}
