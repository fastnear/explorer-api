#[cfg(feature = "openapi")]
fn main() -> anyhow::Result<()> {
    let check = std::env::args().skip(1).any(|arg| arg == "--check");
    explorer_api::openapi::generate(check)
}

#[cfg(not(feature = "openapi"))]
fn main() {
    eprintln!("This binary requires the `openapi` feature.");
    std::process::exit(1);
}
