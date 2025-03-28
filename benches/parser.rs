use divan::{black_box, Bencher};

use xoz::Xoz;

#[divan::bench]
fn test_parse_mondial(bencher: Bencher) {
    let xml = load_xml_data("mondial-3.0.xml");
    let mut xoz = Xoz::new();

    bencher.bench_local(move || {
        let root = xoz.parse_str(&xml).unwrap();
        black_box(root);
    });
}

fn main() {
    divan::main();
}

fn load_xml_data(name: &str) -> String {
    let xml_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("vendor/testdata/")
        .join(name);
    std::fs::read_to_string(xml_path).unwrap()
}
