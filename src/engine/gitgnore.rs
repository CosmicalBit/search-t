use std::{
    env::{current_dir, home_dir}, fs::File, io::{self, Error, ErrorKind, Read}, os::unix::fs::MetadataExt, path::{Path, PathBuf}, process::exit,
};

const GIT: &str = ".gitgnore";

macro_rules! Debug {
    ($code:expr) => {
        #[cfg(debug_assertions)]
        {
            $code
        }
    };
}

//TODO: implement a acctual parsing for the gitgnore
pub fn to_ignore() -> io::Result<Vec<PathBuf>> {
    let home_dir = home_dir().ok_or_else(|| Error::new(ErrorKind::InvalidData, "no home dir"))?;
    let current_dir = current_dir()?;
    let git_file = current_dir.join(GIT);

    let mut vec = Vec::new();

    if  git_file.is_file() {
        let mut file = File::open(git_file)?;
        let mut contents = String::with_capacity(file.metadata()?.size() as usize);

        file.read_to_string(&mut contents)?;

        for (line_number, line) in contents.lines().enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line = current_dir.join(line);

            
            vec.push(line);
        }
        return Ok(vec);
    }

    Ok(vec)
}

