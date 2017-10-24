extern crate flate2;

#[test]
fn zero_write_is_error() {
    let mut buf = [0u8];
    let writer = flate2::write::DeflateEncoder::new(&mut buf[..], flate2::Compression::Default);
    assert!(writer.finish().is_err());
}
