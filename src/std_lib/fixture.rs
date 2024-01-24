use std::path::Path;

pub fn load_fixture_file(file_name: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let fixture = path.join("tests").join("fixtures").join(file_name);

    fixture.to_str().unwrap().to_string()
}
