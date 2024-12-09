# imageformat

Quickly probe the format of various image formats without reading the entire file.

The goal of this crate is to be able to detect format of a supported image without loading unnecessary data, and without pulling in more dependencies. Most reads only require 16 bytes or less.

## Usage

```rust
use image_format_detector::prelude::*;

fn main() {
    // Example: Read the first few bytes of a file
    let file = std::fs::File::open("/Somepath/example.webp")?;
    let mut cursor = std::io::Cursor::new(file);
    let format = detect_image_format(&mut cursor);

    // Or: pass in the file path
    let format = detect_image_format_path("/Somepath/example.png").unwrap();

    match format {
        ImageFormat::Jpeg => println!("It's a JPEG image."),
        ImageFormat::JpegXl => println!("It's a JPEG XL image."),
        ImageFormat::Png => println!("It's a PNG image."),
        ImageFormat::Webp => println!("It's a WEBP image."),
        ...
    }
}
```

### Supported Image Formats

- Aseprite
- Avif
- BMP
- DDS
- EXR
- Farbfeld
- GIF
- HDR
- HEIC / HEIF
- ICO\*
- ILBM (IFF)
- JPEG
- JPEG XL
- KTX2
- PNG
- PNM (PBM, PGM, PPM)
- PSD / PSB
- QOI
- TGA
- TIFF
- VTF
- WEBP

If you have a format you think should be added, feel free to create an issue.
