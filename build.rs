use glob::glob;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    for entry in glob("src/**/*.asm")?.filter_map(Result::ok) {
        let asm_path = entry.display().to_string();
        let asm = entry.strip_prefix("src/")?;
        let obj_path = out_dir.join(asm.with_extension("o")).display().to_string();

        println!("cargo:rerun-if-changed={}", &asm_path);

        let status = Command::new("nasm")
            .args(["-f", "elf32", &asm_path, "-o", &obj_path])
            .status()
            .map_err(|e| format!("Failed to execute nasm: {e}"))?;

        if !status.success() {
            return Err(format!("nasm failed on {asm_path}").into());
        }

        println!("cargo:rustc-link-arg={}", &obj_path);
    }

    Ok(())
}
