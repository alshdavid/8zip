mod compress_tar_gz;
mod compress_tar_xz;
mod compress_zip;
mod decompress_tar_gz;
mod decompress_tar_xz;
mod decompress_zip;
mod input_target;
mod platform;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::input_target::InputTarget;

#[derive(Debug, Parser)]
struct Command {
    #[command(subcommand)]
    command: CommandType,
}

#[derive(Debug, Subcommand)]
enum CommandType {
    CompressTarGz {
        target: PathBuf,
        #[arg(long="cwd", value_parser = platform::cmd_utils::resolve_path, default_value = ".")]
        cwd: PathBuf,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
    CompressTarXz {
        target: PathBuf,
        #[arg(long="cwd", value_parser = platform::cmd_utils::resolve_path, default_value = ".")]
        cwd: PathBuf,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
    CompressZip {
        target: PathBuf,
        #[arg(long="cwd", value_parser = platform::cmd_utils::resolve_path, default_value = ".")]
        cwd: PathBuf,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
    DecompressTarGz {
        target: Option<PathBuf>,
        #[arg(long = "strip-components")]
        strip_components: Option<usize>,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
    DecompressTarXz {
        target: Option<PathBuf>,
        #[arg(long = "strip-components")]
        strip_components: Option<usize>,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
    DecompressZip {
        target: Option<PathBuf>,
        #[arg(long = "strip-components")]
        strip_components: Option<usize>,
        #[arg(long="output", value_parser = platform::cmd_utils::resolve_path)]
        output: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Command::parse();
    // dbg!(&args);

    match args.command {
        CommandType::CompressTarGz {
            target,
            cwd,
            output,
        } => compress_tar_gz::compress_tar_gz(&target, &cwd, &output),
        CommandType::CompressTarXz {
            target,
            cwd,
            output,
        } => compress_tar_xz::compress_tar_xz(&target, &cwd, &output),
        CommandType::CompressZip {
            target,
            cwd,
            output,
        } => compress_zip::compress_zip(&target, &cwd, &output),
        CommandType::DecompressTarGz {
            target,
            strip_components,
            output,
        } => decompress_tar_gz::decompress_tar_gz(
            InputTarget::detect(target)?,
            &output,
            strip_components,
        ),
        CommandType::DecompressTarXz {
            target,
            strip_components,
            output,
        } => decompress_tar_xz::decompress_tar_xz(
            InputTarget::detect(target)?,
            &output,
            strip_components,
        ),
        CommandType::DecompressZip {
            target,
            strip_components,
            output,
        } => {
            decompress_zip::decompress_zip(InputTarget::detect(target)?, &output, strip_components)
        }
    }
}
