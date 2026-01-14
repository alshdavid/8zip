use anyhow::{Context, Result};
use std::fs::File;
use std::path::Path;
use tar::{self};
use walkdir::WalkDir;
use xz2::write::XzEncoder;

pub fn compress_tar_xz(target: &Path, cwd: &Path, output: &Path) -> Result<()> {
    // Resolve the full path to compress
    let source_path = cwd.join(target);

    // Validate that source exists
    if !source_path.exists() {
        anyhow::bail!("Source path does not exist: {:?}", source_path);
    }

    // Create the output file
    let tar_xz =
        File::create(output).context(format!("Failed to create output file: {:?}", output))?;

    // Create xz encoder with default compression level (6)
    let enc = XzEncoder::new(tar_xz, 6);

    // Create tar archive builder
    let mut tar = tar::Builder::new(enc);

    // Walk through the directory recursively
    for entry in WalkDir::new(&source_path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip the root directory itself
        if path == source_path {
            continue;
        }

        // Calculate the relative path for the archive
        let relative_path = path
            .strip_prefix(&source_path)
            .context("Failed to strip prefix")?;

        // Add file or directory to the archive
        if path.is_file() {
            tar.append_path_with_name(path, relative_path)
                .context(format!("Failed to add file to archive: {:?}", path))?;
        } else if path.is_dir() {
            tar.append_dir(relative_path, path)
                .context(format!("Failed to add directory to archive: {:?}", path))?;
        }
    }

    // Finish writing the archive
    tar.finish().context("Failed to finish tar archive")?;

    Ok(())
}
