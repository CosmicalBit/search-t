use std::{
    env::current_dir,
    io::{self},
    path::PathBuf,
};

use clap::Parser;
use owo_colors::OwoColorize;

use crate::engine::{gitgnore::to_ignore, search::dir_iter};

mod engine;

#[derive(Parser)]
struct Args {
    query: String,

    #[arg(default_value = ".")]
    path: Option<PathBuf>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let dir_path = match args.path {
        Some(path) => path,
        None => current_dir()?,
    };

    let query = args.query;

    let ignore_list = to_ignore()?;
    
    let found_list = dir_iter(&dir_path, query.as_bytes(),&ignore_list)?;

    if found_list.is_empty() {
        println!("no occurence of {} was found", query.blue());
        return Ok(());
    }
    
    found_list.iter().for_each(|f| println!("{f}"));

    Ok(())
}
