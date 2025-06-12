use flate2::write::GzEncoder;
use flate2::Compression as GzipCompression;
use bzip2::write::BzEncoder;
use bzip2::Compression as BzipCompression;
use lzma_rs::lzma_compress;
use xz2::write::XzEncoder;
use zstd::stream::encode_all;
use std::io::Cursor;
use std::io::Write;


pub fn compress_gzip_size(data: &str) -> usize {
    let mut encoder = GzEncoder::new(Vec::new(), GzipCompression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    compressed_data.len()
}

pub fn compress_bzip2_size(data: &str) -> usize {
    let mut encoder = BzEncoder::new(Vec::new(), BzipCompression::default());
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    compressed_data.len()
}

pub fn compress_xz_size(data: &str) -> usize {
    let mut encoder = XzEncoder::new(Vec::new(), 6);
    encoder.write_all(data.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();
    compressed_data.len()
}

pub fn compress_zstd_size(data: &str) -> usize {
    let compressed = encode_all(Cursor::new(data), 0).unwrap();
    compressed.len()
}

pub fn compress_lzma_size(data: &str) -> usize {
    let mut compressed = Vec::new();
    lzma_compress(&mut Cursor::new(data.as_bytes()), &mut compressed).unwrap();
    compressed.len()
}

