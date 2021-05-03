//! Loads the given FBX file.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use fbxcel_dom::any::AnyDocument;

fn usage() -> ! {
    println!("Pass an FBX file path.");
    std::process::exit(1);
}

fn main() -> Result<()> {
    let path = match std::env::args_os().nth(1) {
        Some(s) if s == "--help" => usage(),
        Some(s) => PathBuf::from(s),
        None => usage(),
    };
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let _data = AnyDocument::from_seekable_reader(reader)?;

    Ok(())
}
