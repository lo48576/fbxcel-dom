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
            print_global_settings_v7400(&doc);
            print_doc_summary_v7400(&doc);
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

    match meta.file_id() {
        Ok(v) => println!("File ID: {:02x?}", v),
        Err(e) => eprintln!("[ERROR] Failed to get file ID: {}", e),
    }
}

fn print_global_settings_v7400(doc: &fbxcel_dom::v7400::Document) {
    let global_settings = match doc.global_settings() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[ERROR] Failed to get global settings: {}", e);
            return;
        }
    };

    println!("Global settings:");

    match global_settings.axis_system() {
        Ok(asys) => println!("\taxis system: {:?}", asys),
        Err(e) => eprintln!("[ERROR] Failed to get axis system: {}", e),
    }
    match global_settings.original_up_axis() {
        Ok(orig_up) => println!("\toriginal up axis: {}", orig_up),
        Err(e) => eprintln!("[ERROR] Failed to get original up axis: {}", e),
    }
}

fn print_doc_summary_v7400(doc: &fbxcel_dom::v7400::Document) {
    {
        println!("Scenes:");
        for (scene_i, scene) in doc.scenes().enumerate() {
            println!(
                "\tScene #{}: scene_obj_id={:?}, root_object_id={:?}",
                scene_i,
                scene.scene_object_id(),
                scene.root_object_id()
            );
            match scene.children() {
                Ok(children) => {
                    for child in children {
                        println!(
                            "\t\tchild_of_root: {:?} (class={}, subclass={}, name={:?})",
                            child.node_name(),
                            child.class(),
                            child.subclass(),
                            child.name()
                        );
                    }
                }
                Err(e) => eprintln!("[ERROR] Failed to get child objects of the scene: {}", e),
            };
        }
    }
}
