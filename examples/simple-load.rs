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

    match AnyDocument::from_seekable_reader(reader)? {
        AnyDocument::V7400(ver, doc) => {
            println!("FBX version: {}.{}", ver.major(), ver.minor());
            print_doc_meta_v7400(&doc);
        }
        v => {
            anyhow::bail!(
                "unsupported FBX version {}.{}",
                v.fbx_version().major(),
                v.fbx_version().minor()
            );
        }
    }

    Ok(())
}

fn print_doc_meta_v7400(doc: &fbxcel_dom::v7400::Document) {
    let meta = doc.meta();

    match meta.creation_timestamp() {
        Ok(v) => println!("Creation timestamp: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get creation timestamp: {}", e),
    }

    match meta.creator() {
        Ok(v) => println!("Creator: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get creator: {}", e),
    }

    match meta.original_filename() {
        Ok(v) => println!("Original filename: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get original filename: {}", e),
    }

    match meta.original_application_vendor() {
        Ok(v) => println!("Original application vendor: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get original application vendor: {}", e),
    }
    match meta.original_application_name() {
        Ok(v) => println!("Original application name: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get original application name: {}", e),
    }
    match meta.original_application_version() {
        Ok(v) => println!("Original application version: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get original application version: {}", e),
    }

    match meta.last_saved_application_vendor() {
        Ok(v) => println!("Last saved application vendor: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get last saved application vendor: {}", e),
    }
    match meta.last_saved_application_name() {
        Ok(v) => println!("Last saved application name: {:?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get last saved application name: {}", e),
    }
    match meta.last_saved_application_version() {
        Ok(v) => println!("Last saved application version: {:?}", v),
        Err(e) => eprintln!(
            "[ERROR] Failed to get last saved application version: {}",
            e
        ),
    }
}
