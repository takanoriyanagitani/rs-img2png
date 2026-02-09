use std::io;

use io::Cursor;
use io::Seek;
use io::Write;

use io::Read;

use image::DynamicImage;
use image::ImageFormat;

pub struct Image {
    pub img: DynamicImage,
}

impl Image {
    pub fn to_writer<W>(&self, mut wtr: W) -> Result<W, io::Error>
    where
        W: Write + Seek,
    {
        let fmt: ImageFormat = ImageFormat::Png;
        self.img.write_to(&mut wtr, fmt).map_err(io::Error::other)?;
        wtr.flush()?;
        Ok(wtr)
    }
}

impl Image {
    pub fn from_bytes(bytes: &[u8], ifmt: Option<ImageFormat>) -> Result<Self, io::Error> {
        match ifmt {
            None => image::load_from_memory(bytes),
            Some(f) => image::load_from_memory_with_format(bytes, f),
        }
        .map_err(io::Error::other)
        .map(|img| Self { img })
    }
}

impl Image {
    pub fn from_reader<R>(rdr: R, limit: u64, ifmt: Option<ImageFormat>) -> Result<Self, io::Error>
    where
        R: Read,
    {
        let mut taken = rdr.take(limit);
        let mut buf: Vec<u8> = vec![];
        taken.read_to_end(&mut buf)?;
        Self::from_bytes(&buf, ifmt)
    }
}

pub const INPUT_BYTES_MAX: u64 = 16777216;

impl Image {
    pub fn stdin2stdout(limit: u64, ifmt: Option<ImageFormat>) -> Result<(), io::Error> {
        let img: Self = Self::from_reader(io::stdin().lock(), limit, ifmt)?;
        let buf: Vec<u8> = vec![];
        let cur = Cursor::new(buf);
        let wtr: Cursor<_> = img.to_writer(cur)?;
        let buf: Vec<u8> = wtr.into_inner();
        let o = io::stdout();
        let mut ol = o.lock();
        ol.write_all(&buf)?;
        ol.flush()
    }

    pub fn stdin2stdout_default() -> Result<(), io::Error> {
        Self::stdin2stdout(INPUT_BYTES_MAX, None)
    }
}

pub fn str2image_format(s: &str) -> Option<ImageFormat> {
    ImageFormat::from_extension(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str2image_format() {
        assert_eq!(str2image_format("png"), Some(ImageFormat::Png));
        assert_eq!(str2image_format("jpeg"), Some(ImageFormat::Jpeg));
        assert_eq!(str2image_format("jpg"), Some(ImageFormat::Jpeg)); // common alias
        assert_eq!(str2image_format("webp"), Some(ImageFormat::WebP));
        assert_eq!(str2image_format("xyz"), None); // invalid extension
        assert_eq!(str2image_format(""), None); // empty string
    }
}
