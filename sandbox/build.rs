use std::{
    env, fs,
    path::{Path, PathBuf},
};

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // OUT_DIR looks like:
    // target/debug/build/sandbox-<hash>/out
    //
    // We want:
    // target/debug/
    let profile_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to determine Cargo profile directory");

    let source = manifest_dir.join("assets");
    let destination = profile_dir.join("assets");

    println!("cargo:rerun-if-changed={}", source.display());

    copy_dir(&source, &destination)?;

    Ok(())
}
