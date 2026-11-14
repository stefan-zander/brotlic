use brotlic::BrotliDecoder;

/// Lowest bundled brotli version this crate accepts (1.2.1), encoded as
/// `major << 24 | minor << 12 | patch` like `BrotliDecoderVersion()`.
const MIN_BROTLI_VERSION: u32 = (1 << 24) | (2 << 12) | 1;

#[test]
fn bundled_brotli_is_at_least_minimum_version() {
    let version = BrotliDecoder::version();

    assert!(
        version >= MIN_BROTLI_VERSION,
        "bundled brotli decoder is {}.{}.{}, expected at least 1.2.1",
        version >> 24,
        (version >> 12) & 0xfff,
        version & 0xfff
    );
}
