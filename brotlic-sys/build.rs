use std::env;
use std::fs;
use std::path::{Path, PathBuf};

/// Lowest bundled brotli version this crate may be built with.
const MIN_BROTLI_VERSION: (u32, u32, u32) = (1, 2, 1);

fn main() {
    let manifest_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let include_dir = manifest_dir.join("brotli/c/include");

    check_brotli_version(&manifest_dir.join("brotli/c/common/version.h"));

    cc::Build::new()
        .files(&[
            "brotli/c/common/constants.c",
            "brotli/c/common/context.c",
            "brotli/c/common/dictionary.c",
            "brotli/c/common/platform.c",
            "brotli/c/common/shared_dictionary.c",
            "brotli/c/common/transform.c",
            "brotli/c/dec/bit_reader.c",
            "brotli/c/dec/decode.c",
            "brotli/c/dec/huffman.c",
            "brotli/c/dec/prefix.c",
            "brotli/c/dec/state.c",
            "brotli/c/dec/static_init.c",
            "brotli/c/enc/backward_references.c",
            "brotli/c/enc/backward_references_hq.c",
            "brotli/c/enc/bit_cost.c",
            "brotli/c/enc/block_splitter.c",
            "brotli/c/enc/brotli_bit_stream.c",
            "brotli/c/enc/cluster.c",
            "brotli/c/enc/command.c",
            "brotli/c/enc/compound_dictionary.c",
            "brotli/c/enc/compress_fragment.c",
            "brotli/c/enc/compress_fragment_two_pass.c",
            "brotli/c/enc/dictionary_hash.c",
            "brotli/c/enc/encode.c",
            "brotli/c/enc/encoder_dict.c",
            "brotli/c/enc/entropy_encode.c",
            "brotli/c/enc/fast_log.c",
            "brotli/c/enc/histogram.c",
            "brotli/c/enc/literal_cost.c",
            "brotli/c/enc/memory.c",
            "brotli/c/enc/metablock.c",
            "brotli/c/enc/static_dict.c",
            "brotli/c/enc/static_dict_lut.c",
            "brotli/c/enc/static_init.c",
            "brotli/c/enc/utf8_util.c",
        ])
        .include("brotli/c/include")
        .define("BROTLI_BUILD_ENC_EXTRA_API", None)
        .define("BROTLI_HAVE_LOG2", "1")
        .warnings(false)
        .compile("brotli");

    println!("cargo:include={}", include_dir.display());
    println!("cargo:rerun-if-changed=brotli/c");
}

/// Fails the build if the bundled brotli sources are older than
/// [`MIN_BROTLI_VERSION`].
fn check_brotli_version(version_h: &Path) {
    let contents = fs::read_to_string(version_h)
        .unwrap_or_else(|e| panic!("failed to read {}: {}", version_h.display(), e));

    let read = |name: &str| -> u32 {
        contents
            .lines()
            .find_map(|line| {
                let mut parts = line.split_whitespace();
                match (parts.next(), parts.next(), parts.next()) {
                    (Some("#define"), Some(n), Some(value)) if n == name => value.parse().ok(),
                    _ => None,
                }
            })
            .unwrap_or_else(|| panic!("{} not found in {}", name, version_h.display()))
    };

    let version = (
        read("BROTLI_VERSION_MAJOR"),
        read("BROTLI_VERSION_MINOR"),
        read("BROTLI_VERSION_PATCH"),
    );

    if version < MIN_BROTLI_VERSION {
        panic!(
            "bundled brotli is {}.{}.{}, but at least {}.{}.{} is required",
            version.0,
            version.1,
            version.2,
            MIN_BROTLI_VERSION.0,
            MIN_BROTLI_VERSION.1,
            MIN_BROTLI_VERSION.2
        );
    }
}
