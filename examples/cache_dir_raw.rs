//! A trivial example demonstrating the use of [CacheDir::read_file] with
//! [CacheFieldRawPrinter]

use ccache_stats_reader::{CacheDir, CacheFieldRawPrinter};
use std::path::PathBuf;

fn main() {
    let leaf = CacheDir::read_dir(PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("Must pass a path to a ccache root dir"),
    ))
    .unwrap();
    leaf.raw_print();
}
