use std::{fs::File, io::Read, path::Path};

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, strum::Display)]
pub enum ImageFormat {
    #[strum(to_string = "jpg")]
    Jpeg,
    #[strum(to_string = "jpg")]
    JpegXl,
    #[strum(to_string = "png")]
    Png,
    #[strum(to_string = "webp")]
    Webp,
    #[strum(to_string = "ase")]
    Aseprite,
    #[strum(to_string = "avif")]
    Avif,
    #[strum(to_string = "bmp")]
    Bmp,
    #[strum(to_string = "dds")]
    Dds,
    #[strum(to_string = "exr")]
    Exr,
    #[strum(to_string = "ff")]
    Farbfeld,
    #[strum(to_string = "gif")]
    Gif,
    #[strum(to_string = "hdr")]
    Hdr,
    #[strum(to_string = "heif")]
    Heif,
    #[strum(to_string = "ico")]
    Ico,
    #[strum(to_string = "ilbm")]
    Ilbm,
    #[strum(to_string = "ktx2")]
    Ktx2,
    #[strum(to_string = "pcx")]
    Pcx,
    #[strum(to_string = "pnm")]
    Pnm,
    #[strum(to_string = "psd")]
    Psd,
    #[strum(to_string = "qoi")]
    Qoi,
    #[strum(to_string = "tga")]
    Tga,
    #[strum(to_string = "tiff")]
    Tiff,
    #[strum(to_string = "vtf")]
    Vtf,
}

pub fn detect_image_format<R: Read>(reader: &mut R) -> std::io::Result<ImageFormat> {
    let mut buffer = [0u8; 12];
    reader.read_exact(&mut buffer)?;

    Ok(match buffer {
        // JPEG
        [0xFF, 0xD8, 0xFF, ..] => ImageFormat::Jpeg,
        // JPEG XL
        [0xFF, 0x0A, ..] => ImageFormat::JpegXl,
        // PNG
        [0x89, b'P', b'N', b'G', ..] => ImageFormat::Png,
        // WebP
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'E', b'B', b'P'] => ImageFormat::Webp,
        // AVIF
        [_, _, _, _, b'f', b't', b'y', b'p', b'a', b'v', b'i', b'f'] => ImageFormat::Avif,
        // BMP
        [b'B', b'M', ..] => ImageFormat::Bmp,
        // DDS
        [b'D', b'D', b'S', b' ', ..] => ImageFormat::Dds,
        // EXR
        [b'v', b'/', b'1', b'0', ..] => ImageFormat::Exr,
        // Farbfeld
        [b'f', b'a', b'r', b'b', b'f', b'e', b'l', b'd', ..] => ImageFormat::Farbfeld,
        // GIF
        [b'G', b'I', b'F', b'8', b'9', b'a', ..] | [b'G', b'I', b'F', b'8', b'7', b'a', ..] => {
            ImageFormat::Gif
        }
        // HDR
        [b'#', b'?', ..] => ImageFormat::Hdr,
        // HEIF
        [_, _, _, _, b'f', b't', b'y', b'p', b'h', b'e', b'i', b'c'] => ImageFormat::Heif,
        // ICO
        [0x00, 0x00, 0x01, 0x00, ..] => ImageFormat::Ico,
        // ILBM
        [b'F', b'O', b'R', b'M', ..] if buffer[8..12] == *b"ILBM" => ImageFormat::Ilbm,
        // KTX2
        [0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A] => {
            ImageFormat::Ktx2
        }
        // PCX
        [0x0A, 0x00, 0x01, ..] => ImageFormat::Pcx,
        // PNM (PBM, PGM, PPM)
        [b'P', b'1'..=b'6', ..] => ImageFormat::Pnm,
        // PSD
        [0x38, 0x42, 0x50, 0x53, ..] => ImageFormat::Psd,
        // QOI
        [b'q', b'o', b'i', b'f', ..] => ImageFormat::Qoi,
        // TGA
        [0x00, 0x00, 0x02, ..] => ImageFormat::Tga,
        // TIFF
        [0x49, 0x49, 0x2A, 0x00, ..] | [0x4D, 0x4D, 0x00, 0x2A, ..] => ImageFormat::Tiff,
        // VTF
        [b'V', b'T', b'F', b'\x00', ..] => ImageFormat::Vtf,
        // Aseprite
        [b'A', b'S', b'E', b'F', ..] => ImageFormat::Aseprite,
        // Default case when no match is found
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "unknown image format",
            ));
        }
    })
}

/// Read first 12 bytes to detect image format
pub fn detect_image_format_path<P: AsRef<Path>>(path: P) -> std::io::Result<ImageFormat> {
    let mut file = File::open(path)?;

    detect_image_format(&mut file)
}

/// A module containing the most commonly used items from the library.
pub mod prelude {
    pub use super::{detect_image_format, detect_image_format_path, ImageFormat};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpeg_detection() {
        let jpeg_bytes = [
            0xFF, 0xD8, 0xFF, 0xDB, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(jpeg_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Jpeg);
    }

    #[test]
    fn test_jpeg_xl_codestream_detection() {
        let jxl_codestream_bytes = [
            0xFF, 0x0A, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(jxl_codestream_bytes);
        assert_eq!(
            detect_image_format(&mut cursor).unwrap(),
            ImageFormat::JpegXl
        );
    }

    #[test]
    fn test_png_detection() {
        let png_bytes = [
            0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(png_bytes);

        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Png);
    }

    #[test]
    fn test_webp_detection() {
        let webp_bytes = [
            b'R', b'I', b'F', b'F', // "RIFF"
            0x00, 0x00, 0x00, 0x00, // Chunk size (ignored)
            b'W', b'E', b'B', b'P', // "WEBP"
        ];
        let mut cursor = std::io::Cursor::new(webp_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Webp);
    }

    #[test]
    fn test_aseprite_detection() {
        let aseprite_bytes = b"ASEF\x00\x01\x02\x03\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(aseprite_bytes);
        assert_eq!(
            detect_image_format(&mut cursor).unwrap(),
            ImageFormat::Aseprite
        );
    }

    #[test]
    fn test_avif_detection() {
        let avif_bytes = [
            0x00, 0x00, 0x00, 0x1C, // Size (ignored)
            b'f', b't', b'y', b'p', // "ftyp"
            b'a', b'v', b'i', b'f', // "avif"
        ];
        let mut cursor = std::io::Cursor::new(avif_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Avif);
    }

    #[test]
    fn test_bmp_detection() {
        let bmp_bytes = b"BM\x00\x01\x02\x03\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(bmp_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Bmp);
    }

    #[test]
    fn test_dds_detection() {
        let dds_bytes = b"DDS \x7C\x00\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(dds_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Dds);
    }

    #[test]
    fn test_exr_detection() {
        let exr_bytes = b"v/10\x00\x00\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(exr_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Exr);
    }

    #[test]
    fn test_farbfeld_detection() {
        let farbfeld_bytes = b"farbfeld\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(farbfeld_bytes);
        assert_eq!(
            detect_image_format(&mut cursor).unwrap(),
            ImageFormat::Farbfeld
        );
    }

    #[test]
    fn test_gif_detection() {
        let gif_bytes = b"GIF89a\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(gif_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Gif);
    }

    #[test]
    fn test_hdr_detection() {
        let hdr_bytes = b"#?RADIANCE\nFORMAT=32-bit_rle_rgbe\n";
        let mut cursor = std::io::Cursor::new(hdr_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Hdr);
    }

    #[test]
    fn test_heif_detection() {
        let heif_bytes = [
            0x00, 0x00, 0x00, 0x18, // Size (ignored)
            b'f', b't', b'y', b'p', // "ftyp"
            b'h', b'e', b'i', b'c', // "heic"
        ];
        let mut cursor = std::io::Cursor::new(heif_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Heif);
    }

    #[test]
    fn test_ico_detection() {
        let ico_bytes = [
            0x00, 0x00, 0x01, 0x00, 0x02, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(ico_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Ico);
    }

    #[test]
    fn test_ilbm_detection() {
        let ilbm_bytes = [
            b'F', b'O', b'R', b'M', // "FORM"
            0x00, 0x00, 0x00, 0x00, // Chunk size (ignored)
            b'I', b'L', b'B', b'M', // "ILBM"
        ];
        let mut cursor = std::io::Cursor::new(ilbm_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Ilbm);
    }

    #[test]
    fn test_ktx2_detection() {
        let ktx2_bytes = [
            0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
        ];
        let mut cursor = std::io::Cursor::new(ktx2_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Ktx2);
    }

    #[test]
    fn test_pnm_detection() {
        let pnm_bytes = b"P6\n# Comment\n";
        let mut cursor = std::io::Cursor::new(pnm_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Pnm);
    }

    #[test]
    fn test_psd_detection() {
        let psd_bytes = b"8BPS\x00\x01\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(psd_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Psd);
    }

    #[test]
    fn test_qoi_detection() {
        let qoi_bytes = b"qoif\x00\x01\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(qoi_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Qoi);
    }

    #[test]
    fn test_tga_detection() {
        let tga_bytes = [
            0x00, 0x00, 0x02, 0x00, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(tga_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Tga);
    }

    #[test]
    fn test_tiff_detection() {
        let tiff_bytes = [
            0x49, 0x49, 0x2A, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
        ];
        let mut cursor = std::io::Cursor::new(tiff_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Tiff);
    }

    #[test]
    fn test_vtf_detection() {
        let vtf_bytes = b"VTF\x00\x00\x00\x00\x00\x00\x00\x00\x00";
        let mut cursor = std::io::Cursor::new(vtf_bytes);
        assert_eq!(detect_image_format(&mut cursor).unwrap(), ImageFormat::Vtf);
    }
}
