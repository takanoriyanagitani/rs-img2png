use std::io;
use std::process::ExitCode;

use anyhow::Result;
use clap::Parser;

use image::ImageFormat;
use rs_img2png::{INPUT_BYTES_MAX, Image, str2image_format};

/// Simple program to convert various image formats to PNG
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Limit the number of input bytes to read
    #[arg(short, long)]
    limit: Option<u64>,

    /// Specify the input image format (e.g., "jpeg", "gif", "webp"). If not specified, the format will be guessed.
    #[arg(long = "input-image-format")]
    input_image_format: Option<String>,
}

fn sub(cli: Cli) -> Result<()> {
    let limit = cli.limit.unwrap_or(INPUT_BYTES_MAX);
    let ifmt: Option<ImageFormat> = match cli.input_image_format {
        Some(fmt_str) => Some(str2image_format(&fmt_str).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown image format: {}", fmt_str),
            )
        })?),
        None => None,
    };

    Image::stdin2stdout(limit, ifmt)?;
    Ok(())
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    sub(cli).map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_limit() {
        let cli = Cli::try_parse_from(["rs-img2png", "-l", "1024"]).unwrap();
        assert_eq!(cli.limit, Some(1024));

        let cli = Cli::try_parse_from(["rs-img2png", "--limit", "2048"]).unwrap();
        assert_eq!(cli.limit, Some(2048));

        let cli = Cli::try_parse_from(["rs-img2png"]).unwrap();
        assert_eq!(cli.limit, None);
    }

    #[test]
    fn test_cli_parse_input_image_format() {
        let cli = Cli::try_parse_from(["rs-img2png", "--input-image-format", "jpeg"]).unwrap();
        assert_eq!(cli.input_image_format, Some("jpeg".to_string()));

        let cli = Cli::try_parse_from(["rs-img2png"]).unwrap();
        assert_eq!(cli.input_image_format, None);
    }

    #[test]
    fn test_sub_unknown_format_error() {
        let cli = Cli {
            limit: None,
            input_image_format: Some("xyz".to_string()),
        };
        let err = sub(cli).unwrap_err();
        assert!(err.to_string().contains("Unknown image format: xyz"));
    }
}
