//! Loads the given FBX file and prints objects hierarchy.

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Result;
use fbxcel_dom::any::AnyDocument;
use fbxcel_dom::v7400::object::model::AnyModelHandle;
use fbxcel_dom::v7400::object::{ObjectSubtypeHandle as _, SceneHandle};

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
            print_hierarchy_v7400(&doc);
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

fn print_hierarchy_v7400(doc: &fbxcel_dom::v7400::Document) {
    for (scene_i, scene) in doc.scenes().enumerate() {
        print_scene_hierarchy_v7400(&scene, scene_i)
    }
}

fn print_scene_hierarchy_v7400(scene: &SceneHandle<'_>, scene_i: usize) {
    println!(
        "Scene #{}: scene_obj_id={:?}, root_object_id={:?}",
        scene_i,
        scene.scene_object_id(),
        scene.root_object_id()
    );
    match scene.children() {
        Ok(children) => {
            for child in children {
                println!(
                    "\tchild_of_root: {:?} (class={}, subclass={}, name={:?})",
                    child.node_name(),
                    child.class(),
                    child.subclass(),
                    child.name()
                );
                match AnyModelHandle::from_object(&child) {
                    Ok(model) => print_model_hierarchy_v7400(&model, 1),
                    Err(e) => eprintln!(
                        "[ERROR] Unexpected scene root: expected object \
                        but got something else: {}",
                        e
                    ),
                }
            }
        }
        Err(e) => eprintln!("[ERROR] Failed to get child objects of the scene: {}", e),
    };
}

fn print_model_hierarchy_v7400(model: &AnyModelHandle<'_>, level: usize) {
    indent(level);
    println!(
        "{}({}): {:?} (id={:?})",
        model.as_object().class(),
        model.subclass(),
        model.as_object().name().unwrap_or_default(),
        model.object_id()
    );
    for child in model.child_models() {
        print_model_hierarchy_v7400(&child, level + 1);
    }
}

fn indent(level: usize) {
    print!("{:width$}", " ", width = level * 4);
}
