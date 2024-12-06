#[derive(Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    JpegXl,
    Png,
    Webp,
    Aseprite,
    Avif,
    Bmp,
    Dds,
    Exr,
    Farbfeld,
    Gif,
    Hdr,
    Heif,
    Ico,
    Ilbm,
    Ktx2,
    Pnm,
    Psd,
    Qoi,
    Tga,
    Tiff,
    Vtf,
    Unknown,
}

pub fn detect_image_format(bytes: &[u8]) -> ImageFormat {
    if is_jpeg(bytes) {
        ImageFormat::Jpeg
    } else if is_jpeg_xl(bytes) {
        ImageFormat::JpegXl
    } else if is_png(bytes) {
        ImageFormat::Png
    } else if is_webp(bytes) {
        ImageFormat::Webp
    } else if is_aseprite(bytes) {
        ImageFormat::Aseprite
    } else if is_avif(bytes) {
        ImageFormat::Avif
    } else if is_bmp(bytes) {
        ImageFormat::Bmp
    } else if is_dds(bytes) {
        ImageFormat::Dds
    } else if is_exr(bytes) {
        ImageFormat::Exr
    } else if is_farbfeld(bytes) {
        ImageFormat::Farbfeld
    } else if is_gif(bytes) {
        ImageFormat::Gif
    } else if is_hdr(bytes) {
        ImageFormat::Hdr
    } else if is_heif(bytes) {
        ImageFormat::Heif
    } else if is_ico(bytes) {
        ImageFormat::Ico
    } else if is_ilbm(bytes) {
        ImageFormat::Ilbm
    } else if is_ktx2(bytes) {
        ImageFormat::Ktx2
    } else if is_pnm(bytes) {
        ImageFormat::Pnm
    } else if is_psd(bytes) {
        ImageFormat::Psd
    } else if is_qoi(bytes) {
        ImageFormat::Qoi
    } else if is_tga(bytes) {
        ImageFormat::Tga
    } else if is_tiff(bytes) {
        ImageFormat::Tiff
    } else if is_vtf(bytes) {
        ImageFormat::Vtf
    } else {
        ImageFormat::Unknown
    }
}

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8])
}

fn is_jpeg_xl(bytes: &[u8]) -> bool {
    let codestream_signature = [0xFF, 0x0A];
    let container_signature = [
        0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
    ];
    bytes.starts_with(&codestream_signature) || bytes.starts_with(&container_signature)
}

fn is_png(bytes: &[u8]) -> bool {
    let png_signature = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    bytes.starts_with(&png_signature)
}

fn is_webp(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return false;
    }
    &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WEBP"
}

fn is_aseprite(bytes: &[u8]) -> bool {
    bytes.starts_with(b"ASEF")
}

fn is_avif(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return false;
    }
    &bytes[4..8] == b"ftyp" && &bytes[8..12] == b"avif"
}

fn is_bmp(bytes: &[u8]) -> bool {
    bytes.starts_with(b"BM")
}

fn is_dds(bytes: &[u8]) -> bool {
    bytes.starts_with(b"DDS ")
}

fn is_exr(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0x76, 0x2F, 0x31, 0x01])
}

fn is_farbfeld(bytes: &[u8]) -> bool {
    bytes.starts_with(b"farbfeld")
}

fn is_gif(bytes: &[u8]) -> bool {
    bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a")
}

fn is_hdr(bytes: &[u8]) -> bool {
    bytes.starts_with(b"#?RADIANCE") || bytes.starts_with(b"#?RGBE")
}

fn is_heif(bytes: &[u8]) -> bool {
    if bytes.len() < 12 {
        return false;
    }
    &bytes[4..8] == b"ftyp" && (&bytes[8..12] == b"heic" || &bytes[8..12] == b"heif")
}

fn is_ico(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0x00, 0x00, 0x01, 0x00])
}

fn is_ilbm(bytes: &[u8]) -> bool {
    if bytes.len() < 8 {
        return false;
    }
    &bytes[0..4] == b"FORM" && &bytes[8..12] == b"ILBM"
}

fn is_ktx2(bytes: &[u8]) -> bool {
    bytes.starts_with(&[
        0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
    ])
}

fn is_pnm(bytes: &[u8]) -> bool {
    matches!(
        bytes.get(0..2),
        Some(b"P1") | Some(b"P2") | Some(b"P3") | Some(b"P4") | Some(b"P5") | Some(b"P6")
    )
}

fn is_psd(bytes: &[u8]) -> bool {
    bytes.starts_with(b"8BPS")
}

fn is_qoi(bytes: &[u8]) -> bool {
    bytes.starts_with(b"qoif")
}

fn is_tga(bytes: &[u8]) -> bool {
    bytes.len() > 2 && (bytes[2] == 0x02 || bytes[2] == 0x0A)
}

fn is_tiff(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0x49, 0x49, 0x2A, 0x00]) || bytes.starts_with(&[0x4D, 0x4D, 0x00, 0x2A])
}

fn is_vtf(bytes: &[u8]) -> bool {
    bytes.starts_with(b"VTF")
}

/// A module containing the most commonly used items from the library.
pub mod prelude {
    pub use super::{detect_image_format, ImageFormat};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpeg_detection() {
        let jpeg_bytes = [0xFF, 0xD8, 0xFF, 0xDB];
        assert_eq!(detect_image_format(&jpeg_bytes), ImageFormat::Jpeg);
    }

    #[test]
    fn test_jpeg_xl_codestream_detection() {
        let jxl_codestream_bytes = [0xFF, 0x0A, 0x00, 0x00];
        assert_eq!(
            detect_image_format(&jxl_codestream_bytes),
            ImageFormat::JpegXl
        );
    }

    #[test]
    fn test_jpeg_xl_container_detection() {
        let jxl_container_bytes = [
            0x00, 0x00, 0x00, 0x0C, 0x4A, 0x58, 0x4C, 0x20, 0x0D, 0x0A, 0x87, 0x0A,
        ];
        assert_eq!(
            detect_image_format(&jxl_container_bytes),
            ImageFormat::JpegXl
        );
    }

    #[test]
    fn test_png_detection() {
        let png_bytes = [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
        assert_eq!(detect_image_format(&png_bytes), ImageFormat::Png);
    }

    #[test]
    fn test_webp_detection() {
        let webp_bytes = [
            b'R', b'I', b'F', b'F', // "RIFF"
            0x00, 0x00, 0x00, 0x00, // Chunk size (ignored)
            b'W', b'E', b'B', b'P', // "WEBP"
        ];
        assert_eq!(detect_image_format(&webp_bytes), ImageFormat::Webp);
    }

    #[test]
    fn test_aseprite_detection() {
        let aseprite_bytes = b"ASEF\x00\x01\x02\x03";
        assert_eq!(detect_image_format(aseprite_bytes), ImageFormat::Aseprite);
    }

    #[test]
    fn test_avif_detection() {
        let avif_bytes = [
            0x00, 0x00, 0x00, 0x1C, // Size (ignored)
            b'f', b't', b'y', b'p', // "ftyp"
            b'a', b'v', b'i', b'f', // "avif"
        ];
        assert_eq!(detect_image_format(&avif_bytes), ImageFormat::Avif);
    }

    #[test]
    fn test_bmp_detection() {
        let bmp_bytes = b"BM\x00\x01\x02\x03";
        assert_eq!(detect_image_format(bmp_bytes), ImageFormat::Bmp);
    }

    #[test]
    fn test_dds_detection() {
        let dds_bytes = b"DDS \x7C\x00\x00\x00";
        assert_eq!(detect_image_format(dds_bytes), ImageFormat::Dds);
    }

    #[test]
    fn test_exr_detection() {
        let exr_bytes = [0x76, 0x2F, 0x31, 0x01, 0x02, 0x02];
        assert_eq!(detect_image_format(&exr_bytes), ImageFormat::Exr);
    }

    #[test]
    fn test_farbfeld_detection() {
        let farbfeld_bytes = b"farbfeld\x00\x00\x00\x00";
        assert_eq!(detect_image_format(farbfeld_bytes), ImageFormat::Farbfeld);
    }

    #[test]
    fn test_gif_detection() {
        let gif_bytes = b"GIF89a\x00\x00\x00\x00";
        assert_eq!(detect_image_format(gif_bytes), ImageFormat::Gif);
    }

    #[test]
    fn test_hdr_detection() {
        let hdr_bytes = b"#?RADIANCE\nFORMAT=32-bit_rle_rgbe\n";
        assert_eq!(detect_image_format(hdr_bytes), ImageFormat::Hdr);
    }

    #[test]
    fn test_heif_detection() {
        let heif_bytes = [
            0x00, 0x00, 0x00, 0x18, // Size (ignored)
            b'f', b't', b'y', b'p', // "ftyp"
            b'h', b'e', b'i', b'c', // "heic"
        ];
        assert_eq!(detect_image_format(&heif_bytes), ImageFormat::Heif);
    }

    #[test]
    fn test_ico_detection() {
        let ico_bytes = [0x00, 0x00, 0x01, 0x00, 0x02, 0x00];
        assert_eq!(detect_image_format(&ico_bytes), ImageFormat::Ico);
    }

    #[test]
    fn test_ilbm_detection() {
        let ilbm_bytes = [
            b'F', b'O', b'R', b'M', // "FORM"
            0x00, 0x00, 0x00, 0x00, // Chunk size (ignored)
            b'I', b'L', b'B', b'M', // "ILBM"
        ];
        assert_eq!(detect_image_format(&ilbm_bytes), ImageFormat::Ilbm);
    }

    #[test]
    fn test_ktx2_detection() {
        let ktx2_bytes = [
            0xAB, 0x4B, 0x54, 0x58, 0x20, 0x32, 0x30, 0xBB, 0x0D, 0x0A, 0x1A, 0x0A,
        ];
        assert_eq!(detect_image_format(&ktx2_bytes), ImageFormat::Ktx2);
    }

    #[test]
    fn test_pnm_detection() {
        let pnm_bytes = b"P6\n# Comment\n";
        assert_eq!(detect_image_format(pnm_bytes), ImageFormat::Pnm);
    }

    #[test]
    fn test_psd_detection() {
        let psd_bytes = b"8BPS\x00\x01";
        assert_eq!(detect_image_format(psd_bytes), ImageFormat::Psd);
    }

    #[test]
    fn test_qoi_detection() {
        let qoi_bytes = b"qoif\x00\x01";
        assert_eq!(detect_image_format(qoi_bytes), ImageFormat::Qoi);
    }

    #[test]
    fn test_tga_detection() {
        let tga_bytes = [0x00, 0x01, 0x02, 0x00, 0x00];
        assert_eq!(detect_image_format(&tga_bytes), ImageFormat::Tga);
    }

    #[test]
    fn test_tiff_detection() {
        let tiff_bytes = [0x49, 0x49, 0x2A, 0x00];
        assert_eq!(detect_image_format(&tiff_bytes), ImageFormat::Tiff);
    }

    #[test]
    fn test_vtf_detection() {
        let vtf_bytes = b"VTF\x00";
        assert_eq!(detect_image_format(vtf_bytes), ImageFormat::Vtf);
    }

    #[test]
    fn test_unknown_format() {
        let unknown_bytes = [0x00, 0x11, 0x22, 0x33];
        assert_eq!(detect_image_format(&unknown_bytes), ImageFormat::Unknown);
    }
}
