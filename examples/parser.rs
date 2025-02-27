use std::hint::black_box;

use xoz::Xoz;

fn main() {
    let xml = load_xml_data("mondial-3.0.xml");
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(&xml).unwrap();
    black_box(root);
    println!("Parsed mondial-3.0.xml");
}

fn load_xml_data(name: &str) -> String {
    let xml_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("vendor/testdata/")
        .join(name);
    std::fs::read_to_string(xml_path).unwrap()
}
