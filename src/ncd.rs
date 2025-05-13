use crate::compressors::{compress_bzip2_size, compress_gzip_size, compress_xz_size, compress_zstd_size};

fn get_compressed_size(data: &str, compressor: &str) -> usize {
    match compressor {
        "gz" => compress_gzip_size(data),
        "bz2" => compress_bzip2_size(data),
        "xz" => compress_xz_size(data),
        "zst" => compress_zstd_size(data),
        _ => panic!("Unsupported compressor: {}", compressor),
    }
}

pub fn compute_ncd(
    x: &str,
    y: &str,
    compressor: &str
) -> f64 {
    let cx = get_compressed_size(x, compressor) as f64;
    let cy = get_compressed_size(y, compressor) as f64;
    let cxy = get_compressed_size(&format!("{}{}", x, y), compressor) as f64;

    let ncd = (cxy - cx.min(cy)) as f64 / cx.max(cy) as f64;

    ncd
}