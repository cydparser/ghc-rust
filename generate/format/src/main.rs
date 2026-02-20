use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;
use std::{fs, io};

fn main() -> io::Result<()> {
    for arg in std::env::args().skip_while(|arg| !arg.ends_with(".rs")) {
        let path: PathBuf = arg.into();

        eprintln!("* Formatting {}", path.display());
        let mut file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .append(false)
            .open(&path)?;

        let mut src = String::new();
        file.read_to_string(&mut src)?;
        file.seek(SeekFrom::Start(0))?;

        let src = generate_format::add_blank_lines(src.as_ref());
        file.write_all(src.as_bytes())?;
    }
    Ok(())
}
