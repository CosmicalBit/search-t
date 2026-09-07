use std::{
    fmt::{self, Display},
    fs::{DirEntry, File, read_dir},
    io::{self, Read},
    os::unix::fs::FileTypeExt,
    path::{Path, PathBuf},
};

use owo_colors::OwoColorize;
use rayon::prelude::*;

pub struct Found {
    line: Box<[u8]>,
    path: PathBuf,
    line_number: usize,
    search_query: Box<[u8]>,
}

impl Found {
    fn new(path: PathBuf, line_number: usize, line: &[u8], search_query: &[u8]) -> Self {
        Found {
            path,
            line_number,
            line: Box::<[u8]>::from(line),
            search_query: Box::<[u8]>::from(search_query),
        }
    }
}

///fn returns Vec<Found> but it calls underneath the search fn and read file fn
/// it is multithreaded
pub fn dir_iter(dir: &Path, search_query: &[u8], to_ignore: &Vec<PathBuf>) -> io::Result<Vec<Found>> {
    let entries: Vec<Result<DirEntry, io::Error>> = read_dir(dir)?.collect();

    //itenerates building a vec of vec of found
    let result = entries
        .into_par_iter()
        .map(|item| -> io::Result<Vec<Found>> {
            let item = item?;
            let path = item.path();
            let file_type = item.file_type()?;

            if to_ignore.contains(&path) || file_type.is_symlink() {
                return Ok(Vec::new());
            }

            if path.is_dir() {
                return dir_iter(&path, search_query, to_ignore);
            }

            if path.is_file() {
                let mut contents_storage = Vec::new();
                let mut found_list = Vec::new();

                read_file(&path, &mut contents_storage)?;

                if contents_storage.is_binary() {
                    return Ok(Vec::new());
                }

                search_file(&contents_storage, search_query, &mut found_list, &path);

                return Ok(found_list);
            }

            Ok(Vec::new())
        })
        .collect::<io::Result<Vec<Vec<Found>>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(result)
}
fn read_file(path: &Path, contents_storage: &mut Vec<u8>) -> io::Result<()> {
    let mut file = File::open(path)?;

    let len = file.metadata()?.len();
    contents_storage.reserve(len as usize);

    file.read_to_end(contents_storage)?;

    Ok(())
}

fn search_file(contents: &[u8], search_query: &[u8], found_list: &mut Vec<Found>, path: &Path) {
    if search_query.is_empty() {
        return;
    }

    for (line_number, line) in contents.split(|&b| b == b'\n').enumerate() {
        //found at index
        if line.windows(search_query.len()).any(|f| f == search_query) {
            let found = Found::new(PathBuf::from(path), line_number, line, search_query);

            found_list.push(found);
        }
    }
}

impl Display for Found {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let word_start = self
            .line
            .windows(self.search_query.len())
            .position(|x| x == self.search_query.as_ref())
            .unwrap();

        let end = word_start + self.search_query.len();

        let before_word = String::from_utf8_lossy(&self.line[..word_start]).to_string();
        let word = String::from_utf8_lossy(&self.search_query).to_string();
        let after_word = String::from_utf8_lossy(&self.line[end..]).to_string();

        write!(
            f,
            "{}-> {}: {}{}{}",
            self.path.display(),
            self.line_number,
            before_word,
            word.blue(),
            after_word
        )?;

        Ok(())
    }
}

pub trait Files: AsRef<[u8]> {
    fn is_binary(&self) -> bool {
        self.as_ref().iter().take(8192).any(|&byte| byte == 0)
    }
}

impl Files for Vec<u8> {}
