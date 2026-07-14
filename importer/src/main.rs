use asset_importer::Importer;
use asset_importer::postprocess::PostProcessSteps;

fn main() {
    let importer = Importer::new();

    // Load your 3D model and apply post-processing (e.g., triangulate, calculate tangents)
    let scene = importer
        .read_file("./assets/models/survival_guitar_backpack.glb")
        .with_post_process(PostProcessSteps::TRIANGULATE | PostProcessSteps::CALC_TANGENT_SPACE)
        .import()
        .expect("Failed to load model");

    println!("Number of meshes: {}", scene.num_meshes());

    // Iterate through meshes
    for mesh in scene.meshes() {
        println!("Vertices: {}", mesh.num_vertices());
    }
}
