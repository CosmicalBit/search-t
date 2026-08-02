use std::{
    fs::OpenOptions,
    io::{self, Read},
    path::{Path, PathBuf},
};

pub fn parse_gitgnore(path: &Path) -> io::Result<Option<Vec<PathBuf>>> {
    let mut file = match OpenOptions::new().read(true).write(false).create(false).open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error),
    };
    let base = path.parent().unwrap_or_else(|| Path::new("."));

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let mut to_ignore = Vec::with_capacity(4);

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('!') {
            continue;
        }

        let path = base.join(line.trim_start_matches('/'));

        let Ok(path) = path.canonicalize() else {
            continue;
        };

        to_ignore.push(path);
    }

    Ok(Some(to_ignore))
}
