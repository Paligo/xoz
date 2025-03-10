use std::hint::black_box;

use xoz::Xoz;

fn main() {
    let xml = load_xml_data("mondial-3.0.xml");
    let mut xoz = Xoz::new();
    let start = std::time::Instant::now();
    let root = xoz.parse_str(&xml).unwrap();
    black_box(root);
    let elapsed = start.elapsed();
    println!("Parsed mondial-3.0.xml, elapsed: {:?}", elapsed);
    println!("Heap size: {}", xoz.heap_size())
}

fn load_xml_data(name: &str) -> String {
    let xml_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("vendor/testdata/")
        .join(name);
    std::fs::read_to_string(xml_path).unwrap()
}
