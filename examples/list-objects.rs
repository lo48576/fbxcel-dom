//! Loads the given FBX file and list objects.

use std::borrow::Cow;
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
            print_objects_v7400(&doc);
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

fn print_objects_v7400(doc: &fbxcel_dom::v7400::Document) {
    for object in doc.objects() {
        println!(
            "object: {}, obj_id={}, name={:?}, class={:?}, subclass={:?}",
            object.node_name(),
            object.id().raw(),
            object.name(),
            object.class(),
            object.subclass(),
        );

        for prop in object.direct_props().into_iter().flatten() {
            let name = prop
                .name()
                .map_or_else(|e| Cow::Owned(format!("[ERROR] {}", e)), Cow::Borrowed);
            let tyname = prop
                .typename()
                .map_or_else(|e| Cow::Owned(format!("[ERROR] {}", e)), Cow::Borrowed);
            let label = prop
                .label()
                .map_or_else(|e| Cow::Owned(format!("[ERROR] {}", e)), Cow::Borrowed);
            let values = prop.value_raw().map_or_else(
                |e| format!("[ERROR] {}", e),
                |values| {
                    if values.len() > 4 {
                        format!("[_; {}]", values.len())
                    } else {
                        format!("{:?}", values)
                    }
                },
            );
            println!(
                "\tprop: name={:?}, type={:?}, label={:?}, values={}",
                name, tyname, label, values
            );
        }

        for conn in object.source_objects() {
            let source_id = conn.source_id();
            let obj = doc.get_object_by_id(source_id);
            let node_info = match obj {
                Some(obj) => Cow::Owned(format!(
                    "(node={}, class={:?}, subclass={:?}, name={:?})",
                    obj.node_name(),
                    obj.class(),
                    obj.subclass(),
                    obj.name()
                )),
                None => Cow::Borrowed("(dummy)"),
            };
            println!(
                "\tchild: {:?} {}, label={:?}",
                conn.destination_id(),
                node_info,
                conn.label()
            );
        }
    }
}
