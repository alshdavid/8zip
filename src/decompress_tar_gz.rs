use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context;
use flate2::read::GzDecoder;
use tar::Archive;

use crate::input_target::InputTarget;
use crate::platform;

pub fn decompress_tar_gz(
  target: InputTarget,
  output: &Path,
  strip_components: Option<usize>,
  path: Option<PathBuf>,
) -> anyhow::Result<()> {
  // Create output directory if it doesn't exist
  std::fs::create_dir_all(output)
    .context(format!("Failed to create output directory: {:?}", output))?;

  // Open the input source (file or stdin)
  let reader: Box<dyn Read> = match target {
    InputTarget::Path(path) => {
      let resolved = platform::cmd_utils::resolve_path(path.to_str().unwrap())?;

      // Validate that the archive exists
      if !resolved.exists() {
        anyhow::bail!("Archive does not exist: {:?}", resolved);
      }

      Box::new(File::open(&resolved).context(format!("Failed to open archive: {:?}", resolved))?)
    }
    InputTarget::Stdin(stdin) => stdin,
  };

  // Create gzip decoder
  let tar = GzDecoder::new(reader);

  // Create tar archive
  let mut archive = Archive::new(tar);

  let strip_count = strip_components.unwrap_or(0);

  // Extract entries
  for entry in archive
    .entries()
    .context("Failed to read archive entries")?
  {
    let mut entry = entry.context("Failed to read entry")?;
    let entry_path = entry.path().context("Failed to get entry path")?;

    // Strip the specified number of components first
    let components: Vec<_> = entry_path.components().collect();
    if components.len() <= strip_count {
      // Skip entries that don't have enough components
      continue;
    }

    // Build new path without the first n components
    let stripped_path = components[strip_count..].iter().collect::<PathBuf>();

    // Skip if the resulting path is empty
    if stripped_path.as_os_str().is_empty() {
      continue;
    }

    // If a specific path filter is provided, check if this entry matches
    if let Some(ref filter_path) = path {
      // Check if the stripped path starts with the filter path
      if !stripped_path.starts_with(filter_path) {
        continue;
      }
    }

    let output_path = output.join(&stripped_path);

    // Create parent directories if needed
    if let Some(parent) = output_path.parent() {
      std::fs::create_dir_all(parent)
        .context(format!("Failed to create parent directory: {:?}", parent))?;
    }

    // Extract the entry
    entry
      .unpack(&output_path)
      .context(format!("Failed to extract entry to: {:?}", output_path))?;
  }

  Ok(())
}
