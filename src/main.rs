use std::{
    env::{self, args_os},
    io::{self},
    os::unix::ffi::OsStringExt,
};

use crate::engine::{gitgnore::parse_gitgnore, search::dir_recursive_search};

mod engine;

fn main() -> io::Result<()> {
    let Some(to_search) = parse() else {
        println!("no args provided");
        return Ok(());
    };

    let current_dir = env::current_dir()?;
    let to_ignore = parse_gitgnore(&current_dir.join(".gitignore"))?;
    let search_result = dir_recursive_search(&current_dir, &to_search, to_ignore.as_deref())?;
    search_result.iter().for_each(|res| println!("{res}"));

    Ok(())
}

fn parse() -> Option<Vec<u8>> {
    let args: Vec<u8> = args_os().skip(1).flat_map(OsStringExt::into_vec).collect();

    if args.is_empty() {
        return None;
    }

    Some(args)
}
