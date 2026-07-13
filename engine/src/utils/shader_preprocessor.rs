use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShaderPreprocessError {
    #[error("failed to read shader: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid include syntax: {0}")]
    InvalidInclude(String),
}

pub fn preprocess_shader(path: impl AsRef<Path>) -> Result<String, ShaderPreprocessError> {
    let path = fs::canonicalize(path)?;
    let mut included = HashSet::new();

    process_file(&path, &mut included)
}

fn process_file(
    path: &Path,
    included: &mut HashSet<PathBuf>,
) -> Result<String, ShaderPreprocessError> {
    let canonical = fs::canonicalize(path)?;

    if included.contains(&canonical) {
        return Ok(String::new());
    }

    included.insert(canonical.clone());

    let source = fs::read_to_string(&canonical)?;
    let current_dir = canonical.parent().unwrap();

    let mut output = String::new();

    for line in source.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("#include") {
            let first = line.find('"');
            let last = line.rfind('"');

            let (first, last) = match (first, last) {
                (Some(f), Some(l)) if f != l => (f, l),
                _ => return Err(ShaderPreprocessError::InvalidInclude(line.into())),
            };

            let include = &line[first + 1..last];

            let include_path = current_dir.join(include);

            output.push_str(&process_file(&include_path, included)?);
        } else {
            output.push_str(line);
            output.push('\n');
        }
    }

    Ok(output)
}
