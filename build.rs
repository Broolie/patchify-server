use flatc_rust;

use std::path::Path;

fn main() {

    let common = Path::new("schemas/common.fbs");
    let requests = Path::new("schemas/requests.fbs");
    let responses = Path::new("schemas/responses.fbs");

    println!("cargo:rerun-if-changed={}", common.display());
    println!("cargo:rerun-if-changed={}", requests.display());
    println!("cargo:rerun-if-changed={}", responses.display());

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[common, requests, responses],
        out_dir: Path::new("src/schemas/patchify"),
        includes: &[Path::new("schemas")],
        ..Default::default()
    }).expect("flatc failed");
}