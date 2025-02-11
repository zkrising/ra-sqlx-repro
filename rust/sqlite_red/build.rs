use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("template.sqlite.db");

    let _ = fs::remove_file(&target_path);

    let status = Command::new("cargo")
        .args([
            "sqlx",
            "database",
            "setup",
            "-D",
            &format!("sqlite://{}", target_path.display()),
        ])
        .status()?;

    if !status.success() {
        panic!("failed to build template db {:?}", status);
    }

    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=migrations");

    Ok(())
}
