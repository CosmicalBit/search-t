use std::{
    env::current_dir,
    io::{self},
    path::PathBuf,
};

use clap::Parser;
use owo_colors::OwoColorize;
use crate::engine::search::dir_iter;

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

    let mut contents_storage = Vec::new();
    let mut found_list = Vec::with_capacity(256);

    let query = args.query;
    dir_iter(&dir_path, &mut contents_storage, &mut found_list, query.as_bytes())?;

    if found_list.is_empty(){
        println!("no occurence of {} was found", query.blue() );
        return Ok(())
    }
    found_list.iter().for_each(|f| println!("{f}"));

    Ok(())
}
