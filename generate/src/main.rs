use std::path::{Path, PathBuf};

fn main() {
    let source_dir = PathBuf::from(String::from(env!("OUT_DIR")));

    let target_dir = PathBuf::from(
        std::env::args()
            .next()
            .expect("Missing TARGET_DIR argument"),
    );

    // Create nested directories in the target_dir.
    traverse_dir(&source_dir, &|(_path, is_dir)| {
        if is_dir {
            todo!();
        }
        Ok(())
    });
}

fn traverse_dir<F>(dir: &Path, f: &F) -> Result<(), Box<dyn std::error::Error>>
where
    F: Fn((&Path, bool)) -> Result<(), Box<dyn std::error::Error>>,
{
    for entry in dir.read_dir().unwrap() {
        let entry = entry?;
        let path = entry.path();

        if path.starts_with(".") {
            continue;
        }
        let meta = entry.metadata()?;

        f((&path, meta.is_dir()))?;
        traverse_dir(&path, &f)?;
    }
    Ok(())
}
