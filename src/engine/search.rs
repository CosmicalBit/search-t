use std::{
    env::current_dir,
    fmt::{self, Display},
    fs::{DirEntry, OpenOptions, read_dir},
    io::{self, Read},
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use memchr::memmem;
use owo_colors::OwoColorize;
use rayon::prelude::*;

pub struct Found {
    path: PathBuf,
    text: Vec<u8>,
    line: usize,
    collum: usize,
    len: usize,
}
impl Display for Found {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (before, rest) = self.text.split_at(self.collum);
        let (matched, after) = rest.split_at(self.len);

        write!(
            formatter,
            "{}: {}: {}: {}{}{}",
            self.path.display(),
            self.line,
            self.collum,
            String::from_utf8_lossy(before),
            String::from_utf8_lossy(matched).green(),
            String::from_utf8_lossy(after)
        )
    }
}

///Recursively searches  the directory for a byte pattern.
/// returns struct Found that contains path, text, line, collum.
///
/// #Error
///
/// returns a error if io op fails or if entry metadata cant be read
pub fn dir_recursive_search(path: &Path, pattern: &[u8], to_ignore: Option<&[PathBuf]>) -> io::Result<Vec<Found>> {
    let files: Vec<DirEntry> = read_dir(path)?.collect::<io::Result<Vec<_>>>()?;

    let collections = files
        .into_par_iter()
        .map(|entry: DirEntry| {
            let metadata = entry.metadata()?;
            let path = entry.path();

            if let Some(to_ignore) = to_ignore
                && to_ignore.contains(&path)
            {
                return Ok(Vec::new());
            }

            if metadata.is_file() {
                file_search(&path, pattern)
            } else if metadata.is_dir() {
                dir_recursive_search(&path, pattern, to_ignore)
            } else {
                Ok(Vec::new())
            }
        })
        .collect::<io::Result<Vec<_>>>()?;

    Ok(collections.into_iter().flatten().collect())
}

fn file_search(path: &Path, pattern: &[u8]) -> io::Result<Vec<Found>> {
    let mut vec = Vec::new();

    let mut file = OpenOptions::new().read(true).write(false).create(false).open(path)?;

    let size = file.metadata()?.size();

    let mut contents = Vec::with_capacity(size as usize);
    file.read_to_end(&mut contents)?;

    find_pattern(pattern, &contents, &mut vec, path)?;

    Ok(vec)
}

fn find_pattern(pattern: &[u8], contents: &[u8], collection_found: &mut Vec<Found>, path: &Path) -> io::Result<()> {
    for (line_num, line) in contents.split(|byte| *byte == b'\n').enumerate() {
        if line.windows(pattern.len()).any(|candidate| candidate == pattern) {
            let collum = memmem::find(line, pattern).ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "cant find collum of pattern in line that contains pattern"))?;

            let current_path = current_dir()?;
            let data = Found {
                path: path.strip_prefix(current_path).map_err(io::Error::other)?.into(),
                text: line.to_vec(),
                line: line_num,
                collum,
                len: pattern.len(),
            };

            collection_found.push(data);
        }
    }
    Ok(())
}
