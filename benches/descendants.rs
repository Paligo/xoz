use divan::{black_box, Bencher};

use xoz::{NodeType, Xoz};

#[divan::bench]
fn test_descendants_filtered_mondial1(bencher: Bencher) {
    let xml = load_xml_data("mondial-3.0.xml");
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(&xml).unwrap();

    bencher.bench_local(move || {
        let count = xoz
            .descendants(root)
            .filter(|node| xoz.node_type(*node) == &NodeType::element("ethnicgroups"))
            .count();
        black_box(count);
    });
}

#[divan::bench]
fn test_typed_descendants_mondial1(bencher: Bencher) {
    let xml = load_xml_data("mondial-3.0.xml");
    let mut xoz = Xoz::new();
    let root = xoz.parse_str(&xml).unwrap();

    bencher.bench_local(move || {
        let count = xoz
            .typed_descendants(root, NodeType::element("ethnicgroups"))
            .count();
        black_box(count);
    });
}

// #[divan::bench]
// fn test_descendants_filtered_treebank1(bencher: Bencher) {
//     let xml = load_xml_data("treebank_e.xml");
//     let mut xoz = Xoz::new();
//     let root = xoz.parse_str(&xml).unwrap();

//     bencher.bench_local(move || {
//         let count = xoz
//             .descendants(root)
//             .filter(|node| xoz.node_type(*node) == &NodeType::element("NN"))
//             .count();
//         black_box(count);
//     });
// }

// #[divan::bench]
// fn test_typed_descendants_treebank1(bencher: Bencher) {
//     let xml = load_xml_data("treebank_e.xml");
//     let mut xoz = Xoz::new();
//     let root = xoz.parse_str(&xml).unwrap();

//     bencher.bench_local(move || {
//         let count = xoz.typed_descendants(root, NodeType::element("NN")).count();
//         black_box(count);
//     });
// }

fn main() {
    divan::main();
}

fn load_xml_data(name: &str) -> String {
    let xml_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("vendor/testdata/")
        .join(name);
    std::fs::read_to_string(xml_path).unwrap()
}
