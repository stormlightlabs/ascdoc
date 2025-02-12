use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let vendor_dir = PathBuf::from("vendor/tree-sitter-markdown");

    let markdown_dir = vendor_dir.join("tree-sitter-markdown");
    println!("cargo:rerun-if-changed={}", markdown_dir.display());

    cc::Build::new()
        .include(&markdown_dir)
        .file(markdown_dir.join("src/parser.c"))
        .file(markdown_dir.join("src/scanner.c"))
        .opt_level(2)
        .warnings(false)
        .compile("tree-sitter-markdown");
}
