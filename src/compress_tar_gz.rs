use anyhow::{Context, Result};
use flate2::Compression;
use flate2::write::GzEncoder;
use std::fs::File;
use std::path::Path;
use tar::{self};
use walkdir::WalkDir;

pub fn compress_tar_gz(target: &Path, cwd: &Path, output: &Path) -> Result<()> {
    // Resolve the full path to compress
    let source_path = cwd.join(target);

    // Validate that source exists
    if !source_path.exists() {
        anyhow::bail!("Source path does not exist: {:?}", source_path);
    }

    // Create the output file
    let tar_gz =
        File::create(output).context(format!("Failed to create output file: {:?}", output))?;

    // Create gzip encoder
    let enc = GzEncoder::new(tar_gz, Compression::default());

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
