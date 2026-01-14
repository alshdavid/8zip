use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

pub fn compress_zip(target: &Path, cwd: &Path, output: &Path) -> Result<()> {
    // Resolve the full path to compress
    let source_path = cwd.join(target);

    // Validate that source exists
    if !source_path.exists() {
        anyhow::bail!("Source path does not exist: {:?}", source_path);
    }

    // Create the output file
    let file =
        File::create(output).context(format!("Failed to create output file: {:?}", output))?;

    // Create zip writer
    let mut zip = ZipWriter::new(file);

    // Set compression options
    let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

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

        // Convert path to string for zip entry name
        let name = relative_path
            .to_str()
            .context("Path contains invalid UTF-8")?;

        if path.is_file() {
            // Add file to zip
            zip.start_file(name, options)
                .context(format!("Failed to start zip entry: {}", name))?;

            // Read and write file contents
            let mut f = File::open(path).context(format!("Failed to open file: {:?}", path))?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)
                .context(format!("Failed to read file: {:?}", path))?;
            zip.write_all(&buffer)
                .context(format!("Failed to write to zip: {}", name))?;
        } else if path.is_dir() {
            // Add directory to zip (with trailing slash)
            let dir_name = format!("{}/", name);
            zip.add_directory(&dir_name, options)
                .context(format!("Failed to add directory to zip: {}", dir_name))?;
        }
    }

    // Finish writing the zip archive
    zip.finish().context("Failed to finish zip archive")?;

    Ok(())
}
