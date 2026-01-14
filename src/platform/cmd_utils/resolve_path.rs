use std::env;
use std::path::PathBuf;

use normalize_path::NormalizePath;

pub fn resolve_path(input: &str) -> anyhow::Result<PathBuf> {
    let input = PathBuf::from(input);
    if input.is_absolute() {
        Ok(input)
    } else {
        Ok(env::current_dir()?.join(input).normalize())
    }
}
