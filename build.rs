use std::path::Path;

fn main() {
    let common = Path::new("schemas/common.fbs");
    let requests = Path::new("schemas/v1/requests.fbs");
    let responses = Path::new("schemas/v1/responses.fbs");

    println!("cargo:rerun-if-changed={}", common.display());
    println!("cargo:rerun-if-changed={}", requests.display());
    println!("cargo:rerun-if-changed={}", responses.display());

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[common],
        out_dir: Path::new("src/schemas"),
        includes: &[Path::new("schemas")],
        ..Default::default()
    })
    .expect("flatc failed");

    flatc_rust::run(flatc_rust::Args {
        lang: "rust",
        inputs: &[requests, responses],
        out_dir: Path::new("src/schemas/v1"),
        includes: &[Path::new("schemas")],
        ..Default::default()
    })
    .expect("flatc failed");
}
