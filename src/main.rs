use clap::Parser;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// CLI arguments for the wav-files-convert tool.
#[derive(Parser, Debug)]
#[command(author = "RustedBytes", version = "0.1.0", about = "Recursively converts audio files to 16-bit mono 16kHz WAV using FFmpeg", long_about = None)]
struct Args {
    /// Input directory containing audio files (processed recursively)
    input: PathBuf,

    /// Output directory for converted WAV files (subfolder structure preserved)
    output: PathBuf,

    /// Path to FFmpeg binary (defaults to 'ffmpeg' in PATH)
    #[arg(short = 'f', long, default_value = "ffmpeg")]
    ffmpeg_bin: PathBuf,
}

const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "flac", "ogg", "m4a", "aac", "wma", "aiff", "au", "mp2",
];

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Validate input directory exists
    if !args.input.exists() {
        anyhow::bail!("Input directory does not exist: {}", args.input.display());
    }

    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output)?;

    // Find all audio files recursively
    let audio_files = find_audio_files(&args.input)?;

    // Convert each file
    for input_path in audio_files.clone() {
        convert_audio_file(&args, &input_path)?;
    }

    println!(
        "Conversion complete. Processed {} files.",
        audio_files.len()
    );
    Ok(())
}

/// Recursively finds all audio files in the given directory.
fn find_audio_files(dir: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let extensions_set: HashSet<&OsStr> = AUDIO_EXTENSIONS
        .iter()
        .map(|&ext| OsStr::new(ext))
        .collect();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.append(&mut find_audio_files(&path)?);
        } else if let Some(ext) = path.extension()
            && extensions_set.contains(ext)
        {
            files.push(path);
        }
    }

    Ok(files)
}

/// Converts a single audio file to WAV format using FFmpeg.
fn convert_audio_file(args: &Args, input_path: &Path) -> anyhow::Result<()> {
    // Compute relative path to preserve directory structure
    let rel_path = input_path.strip_prefix(&args.input).map_err(|_| {
        anyhow::anyhow!(
            "Failed to compute relative path for: {}",
            input_path.display()
        )
    })?;

    // Construct output path with .wav extension
    let mut output_path = args.output.join(rel_path);
    output_path.set_extension("wav");

    // Create output directory if needed
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Run FFmpeg command
    let status = Command::new(&args.ffmpeg_bin)
        .args([
            "-i",
            input_path
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 path: {}", input_path.display()))?,
            "-f",
            "wav",
            "-acodec",
            "pcm_s16le",
            "-ar",
            "16000",
            "-ac",
            "1",
            "-y",
            output_path.to_str().ok_or_else(|| {
                anyhow::anyhow!("Invalid UTF-8 output path: {}", output_path.display())
            })?,
        ])
        .status()?;

    if !status.success() {
        anyhow::bail!(
            "FFmpeg failed for {} with status: {}",
            input_path.display(),
            status
        );
    }

    println!(
        "Converted: {} -> {}",
        input_path.display(),
        output_path.display()
    );

    Ok(())
}
