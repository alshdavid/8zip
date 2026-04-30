use std::fs::File;
use std::path::Path;

use anyhow::Context;
use anyhow::Result;
use tar::{self};
use walkdir::WalkDir;
use xz2::stream::MtStreamBuilder;
use xz2::write::XzEncoder;

pub fn compress_tar_xz(
  target: &Path,
  cwd: &Path,
  output: &Path,
) -> Result<()> {
  // Resolve the full path to compress
  let source_path = if target.is_absolute() {
    target.to_path_buf()
  } else {
    cwd.join(target)
  };

  // Validate that source exists
  if !source_path.exists() {
    anyhow::bail!("Source path does not exist: {:?}", source_path);
  }

  // Create the output file
  let tar_xz =
    File::create(output).context(format!("Failed to create output file: {:?}", output))?;

  let stream = MtStreamBuilder::new()
    .threads(num_cpus::get_physical() as u32)
    .preset(9) // Compression level 0-9
    .encoder()
    .map_err(|e| anyhow::anyhow!("Failed to create XZ stream: {}", e))?;

  // Create xz encoder with default compression level (6)
  let enc = XzEncoder::new_stream(tar_xz, stream);

  // Create tar archive builder
  let mut tar = tar::Builder::new(enc);
  tar.follow_symlinks(false);

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
      tar
        .append_path_with_name(path, relative_path)
        .context(format!("Failed to add file to archive: {:?}", path))?;
    } else if path.is_dir() {
      tar
        .append_dir(relative_path, path)
        .context(format!("Failed to add directory to archive: {:?}", path))?;
    }
  }

  // Finish writing the archive
  tar.finish().context("Failed to finish tar archive")?;

  Ok(())
}
