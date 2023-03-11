extern crate napi_build;
use std::path::PathBuf;

fn main() {
  napi_build::setup();
  let dir: PathBuf = ["tree-sitter-typescript", "tsx", "src"].iter().collect();

  cc::Build::new()
    .include(&dir)
    .file(dir.join("parser.c"))
    .file(dir.join("scanner.c"))
    .compile("tree-sitter-tsx");
}
