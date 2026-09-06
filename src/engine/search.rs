use std::{
    fmt::{self, Display},
    fs::{ File, read_dir},
    io::{self, Read},
    path::{Path, PathBuf},
};

use owo_colors::OwoColorize;

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

/// this function itenerates over a dir and calls read_file and then search file
/// it return Result<()> but it updates the 'found list' with new found items
pub fn dir_iter(dir: &Path, contents_storage: &mut Vec<u8>, found_list: &mut Vec<Found>, search_query: &[u8]) -> io::Result<()> {
    for item in read_dir(dir)? {
        let item = item?;
        let path = item.path();

        if path.is_dir() {
            dir_iter(&path, contents_storage, found_list, search_query)?;
        }
        if path.is_file() {
            read_file(&path, contents_storage)?;
            search_file(contents_storage, search_query, found_list, &path);

            //clear the contents  storage after use so it can be over written
            contents_storage.clear();
        }
    }
    Ok(())
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
    
        let end = word_start + self.search_query.len() + 1;

        let before_word = String::from_utf8_lossy(&self.line[..word_start]);
        let word = String::from_utf8_lossy(&self.search_query);
        let after_word = String::from_utf8_lossy(&self.line[end..]);

        write!(f, "{}-> {}: {}{}{}", self.path.display(),self.line_number, before_word, word.blue(), after_word)?;

        Ok(())
    }
}
