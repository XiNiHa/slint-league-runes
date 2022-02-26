use std::{
    collections::HashMap,
    fs::{create_dir_all, read_to_string, File},
    io, thread,
};

use slint_build::*;
use toml::{value::Table, Value};

fn add_to_map(
    map: &mut HashMap<String, HashMap<String, String>>,
    dir_path: Vec<String>,
    table: &Table,
) {
    for (key, value) in table.iter() {
        match value {
            Value::String(str) => {
                let inner_map = map.entry(dir_path.join("/")).or_insert(HashMap::new());
                inner_map.insert(key.clone(), str.clone());
            }
            Value::Table(table) => {
                let mut new_dir_path = Vec::new();
                for segment in &dir_path {
                    new_dir_path.push(segment.clone());
                }
                new_dir_path.push(key.clone());
                add_to_map(map, new_dir_path, table);
            }
            _ => unimplemented!(),
        }
    }
}

fn main() {
    let mut file_map = HashMap::new();
    let resources: Table =
        toml::from_str(read_to_string("resources.toml").unwrap().as_str()).unwrap();

    add_to_map(&mut file_map, vec!["resources".to_owned()], &resources);

    let mut handles = Vec::new();

    std::fs::remove_dir_all("resources").unwrap();

    for (dir, files) in file_map {
        for (file, url) in files {
            let dir = dir.clone();
            handles.push(thread::spawn(move || {
                let mut resp = reqwest::blocking::get(url).unwrap();
                create_dir_all(&dir).unwrap();
                let mut out = File::create(format!("{}/{}", &dir, &file)).unwrap();
                io::copy(&mut resp, &mut out).unwrap();
            }));
        }
    }

    for handle in handles {
        handle.join().unwrap();
    }

    compile("ui/App.slint").unwrap();
}
