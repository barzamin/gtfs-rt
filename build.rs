use prost_build::Config;

fn main() {
    Config::new()
        .type_attribute(
            ".",
            "#[derive(Serialize)] #[serde(rename_all = \"snake_case\")]",
        )
        .compile_protos(&["src/gtfs-realtime.proto"], &["src/"])
        .unwrap();
}
