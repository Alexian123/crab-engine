use asset_converter::{Exporter, Importer};

fn main() {
    let importer = Importer::new();
    let model = importer
        .import("./assets/models/survival_guitar_backpack.glb")
        .unwrap();
    let exporter = Exporter::new();
    exporter.export(model, "./test/").ok();
}
