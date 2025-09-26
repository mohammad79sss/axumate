use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

/// Ensure directory exists, return PathBuf
pub fn ensure_dir(path: &Path) -> anyhow::Result<PathBuf> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(path.to_path_buf())
}

/// Ensure a mod.rs exists at the given directory. If not present create with a small header comment.
pub fn ensure_mod_rs(dir: &Path) -> anyhow::Result<PathBuf> {
    let mod_rs = dir.join("mod.rs");
    if !mod_rs.exists() {
        fs::write(&mod_rs, format!("// module: {}\n\n", dir.file_name().and_then(|s| s.to_str()).unwrap_or("?")))?;
    }
    Ok(mod_rs)
}

/// Append `pub mod name;` to specified mod.rs if not already present.
pub fn ensure_pub_mod_decl(mod_rs: &Path, child_mod: &str) -> anyhow::Result<()> {
    let decl = format!("pub mod {};", child_mod);

    // read existing content (or empty)
    let mut content = String::new();
    if mod_rs.exists() {
        let mut f = fs::File::open(mod_rs)?;
        f.read_to_string(&mut content)?;
    } else {
        // create directory of mod_rs if necessary and create file
        if let Some(parent) = mod_rs.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::File::create(mod_rs)?;
    }

    // if declaration already present, nothing to do
    if content.lines().any(|l| l.trim() == decl) {
        return Ok(());
    }

    // append declaration at end (with newline)
    let mut f = fs::OpenOptions::new().append(true).open(mod_rs)?;
    writeln!(f, "{}", decl)?;
    Ok(())
}

/// Ensure the root (src/lib.rs) contains `pub mod module_name;` so crate-level access is available.
/// If src/lib.rs does not exist, we create it.
pub fn ensure_root_mod(module_name: &str) -> anyhow::Result<()> {
    let lib_rs = Path::new("src").join("lib.rs");
    let decl = format!("pub mod {};", module_name);

    // create src/ if missing
    ensure_dir(Path::new("src"))?;
    if !lib_rs.exists() {
        fs::write(&lib_rs, "// auto-generated lib.rs\n\n")?;
    }

    let mut content = String::new();
    {
        let mut f = fs::File::open(&lib_rs)?;
        f.read_to_string(&mut content)?;
    }

    if content.lines().any(|l| l.trim() == decl) {
        return Ok(());
    }

    let mut f = fs::OpenOptions::new().append(true).open(&lib_rs)?;
    writeln!(f, "{}", decl)?;
    Ok(())
}

/// Helper to join src/{module}
pub fn module_dir(module: &str) -> PathBuf {
    Path::new("src").join(module)
}

/// Decide which mod file to update:
/// - If current dir is `src/`, return `lib.rs`
/// - Otherwise, return the `mod.rs` of the current dir
pub fn current_mod_file() -> PathBuf {
    let cwd = std::env::current_dir().unwrap();

    if cwd.ends_with("src") {
        cwd.join("lib.rs")
    } else {
        cwd.join("mod.rs")
    }
}

