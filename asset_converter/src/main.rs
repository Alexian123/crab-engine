use asset_converter::{Exporter, Importer};

use std::env;

fn main() {
    let importer = Importer::new();
    let exporter = Exporter::new();

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!(
            "Usage: {} <third_party_asset_path> <output_dir_path>",
            args[0]
        );
    }
    let import_path = &args[1];
    let output_dir = &args[2];

    let model = importer
        .import(import_path)
        .expect("Failed to import model.");
    exporter
        .export(model, output_dir)
        .expect("Failed to export model.");
}
