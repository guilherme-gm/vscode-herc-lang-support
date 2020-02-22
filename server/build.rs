extern crate cc;

use std::path::PathBuf;

fn main() {
    let dir: PathBuf = ["tree-sitter-hercscript", "src"].iter().collect();

    cc::Build::new()
        .include(&dir)
        .file(dir.join("parser.c"))
        .file(dir.join("scanner.c"))
        .compile("tree-sitter-hercscript");
}
